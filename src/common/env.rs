use std::env;
use std::path::{Path, PathBuf};

pub fn exe_dir() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
}

pub fn curr_dir() -> Option<PathBuf> {
    env::current_dir()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
}
