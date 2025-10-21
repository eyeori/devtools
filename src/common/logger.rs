use crate::common::env::{curr_dir, exe_dir};
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: LevelFilter,
    pub rolling_file: Option<RollingFileLogConfig>,
}

#[derive(Debug, Clone)]
pub struct RollingFileLogConfig {
    pub dir: PathBuf,
    pub filename_prefix: String,
    pub filename_suffix: String,
    pub rotation: Rotation,
    pub max_log_files: Option<usize>,
}

struct RotationCfg<'a>(&'a str);

impl<'a> RotationCfg<'a> {
    pub fn parse(self) -> Result<Rotation> {
        match self.0.to_ascii_lowercase().as_str() {
            "d" | "day" | "daily" => Ok(Rotation::DAILY),
            "h" | "hour" | "hourly" => Ok(Rotation::HOURLY),
            "m" | "minute" | "minutely" => Ok(Rotation::MINUTELY),
            "n" | "never" => Ok(Rotation::NEVER),
            _ => Err(anyhow!("Invalid rotation: {}", self.0)),
        }
    }
}

pub fn init(str: &str) -> Result<Option<WorkerGuard>> {
    let config = log_config_from_str(str)?;
    if let Some(config) = &config {
        let builder = fmt()
            .with_timer(LocalTime::rfc_3339())
            .with_max_level(config.level)
            .with_target(true)
            .with_line_number(true);
        let (writer, guard) = non_blocking(std::io::stdout());
        if let Some(file_config) = &config.rolling_file {
            let mut file_appender = RollingFileAppender::builder()
                .rotation(file_config.rotation.clone())
                .filename_prefix(&file_config.filename_prefix)
                .filename_suffix(&file_config.filename_suffix);
            if let Some(max_log_files) = file_config.max_log_files {
                file_appender = file_appender.max_log_files(max_log_files);
            }
            let file_appender = file_appender.build(log_dir(&file_config.dir))?;
            builder.with_writer(writer.and(file_appender)).init();
        } else {
            builder.with_writer(writer).init();
        }
        info!("log inited, {config:?}");
        return Ok(Some(guard));
    };
    Ok(None)
}

/// [[level][@[dir][/filename_prefix.][filename_suffix][:[rotation]#[max_log_files]]]]
fn log_config_from_str(str: &str) -> Result<Option<LogConfig>> {
    if str.is_empty() {
        return Ok(None);
    }
    let level_rolling_file: Vec<&str> = str.split("@").collect();
    let level = log_level(level_rolling_file[0])?;
    let rolling_file = if level_rolling_file.len() == 2 {
        log_rolling_file(level_rolling_file[1])?
    } else {
        None
    };
    Ok(Some(LogConfig {
        level,
        rolling_file,
    }))
}

fn log_level(level_str: &str) -> Result<LevelFilter> {
    let level = if !level_str.is_empty() {
        level_str.parse()?
    } else {
        LevelFilter::INFO
    };
    Ok(level)
}

fn log_rolling_file(rolling_file_str: &str) -> Result<Option<RollingFileLogConfig>> {
    let path_rolling: Vec<&str> = rolling_file_str.split(":").collect();
    let (dir, filename_prefix, filename_suffix) = log_rolling_file_path(path_rolling[0])?;
    let mut rolling_file = RollingFileLogConfig {
        dir,
        filename_prefix,
        filename_suffix,
        rotation: Rotation::NEVER,
        max_log_files: None,
    };
    if path_rolling.len() == 2 {
        let (rotation, max_log_files) = log_rolling_file_rotation(path_rolling[1])?;
        rolling_file.rotation = rotation;
        rolling_file.max_log_files = max_log_files;
    }
    Ok(Some(rolling_file))
}

fn log_rolling_file_path(rolling_file_path_str: &str) -> Result<(PathBuf, String, String)> {
    let mut result = (
        PathBuf::from("log"),
        "default".to_string(),
        "log".to_string(),
    );
    if rolling_file_path_str.is_empty() {
        return Ok(result);
    }
    let path = Path::new(rolling_file_path_str);
    if path.extension().is_some() {
        if let Some(parent) = path.parent()
            && !parent.to_string_lossy().to_string().is_empty()
        {
            result.0 = parent.to_path_buf();
        }
        if let Some(file_stem) = path.file_stem()
            && !file_stem.is_empty()
        {
            result.1 = file_stem.to_string_lossy().to_string();
        }
        if let Some(extension) = path.extension()
            && !extension.is_empty()
        {
            result.2 = extension.to_string_lossy().to_string();
        }
    } else {
        result.0 = path.to_path_buf();
    }
    Ok(result)
}

fn log_rolling_file_rotation(rolling_file_rotation_str: &str) -> Result<(Rotation, Option<usize>)> {
    let mut result = (Rotation::NEVER, None);
    if rolling_file_rotation_str.is_empty() {
        return Ok(result);
    }
    let rotation_max_log_files: Vec<&str> = rolling_file_rotation_str.split("#").collect();
    let log_rotation = rotation_max_log_files[0];
    if !log_rotation.is_empty() {
        result.0 = RotationCfg(&log_rotation).parse()?;
    }
    if rotation_max_log_files.len() == 2
        && let max_log_files = rotation_max_log_files[1]
        && !max_log_files.is_empty()
    {
        result.1 = Some(max_log_files.parse::<usize>()?);
    }
    Ok(result)
}

fn log_dir(dir: &Path) -> PathBuf {
    if dir.is_dir() && dir.exists() {
        dir.to_path_buf()
    } else if !dir.exists()
        && let real_dir = exe_dir().unwrap().join(dir)
        && real_dir.is_dir()
        && real_dir.exists()
    {
        real_dir
    } else if !dir.exists()
        && let real_dir = curr_dir().unwrap().join(dir)
        && real_dir.is_dir()
        && real_dir.exists()
    {
        real_dir
    } else if let real_dir = exe_dir().unwrap().join(dir)
        && real_dir.is_dir()
    {
        real_dir
    } else {
        exe_dir().unwrap().join("log")
    }
}

#[cfg(test)]
mod tests {
    use crate::common::logger::log_config_from_str;

    #[test]
    fn test() {
        let str = "Trace@logs/default.log:Daily#5";
        let result = log_config_from_str(str);
        println!("{result:?}");
    }
}
