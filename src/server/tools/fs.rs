use crate::server::tools::meta::fs::{DirItem, FileOperateParams, FileOperateResult};
use crate::server::Server;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router, Json};
use serde_json::json;
use std::path::Path;

#[tool_router(router = tool_touter_fs, vis = "pub")]
impl Server {
    #[tool(
        name = "fs.file_operate",
        description = "Operate a file, can be read or execute"
    )]
    fn file_operate(
        &self,
        Parameters(params): Parameters<FileOperateParams>,
    ) -> Json<FileOperateResult> {
        let path = Path::new(&params.path);
        let operate = params.operate.to_ascii_lowercase();
        let arguments = params.arguments.as_deref();
        let result = file_operate(path, &operate, arguments);
        Json(result)
    }
}

fn file_operate(path: &Path, operate: &str, arguments: Option<&str>) -> FileOperateResult {
    match (path, operate) {
        (path, _) if !path.exists() => FileOperateResult::fail("File not exists"),
        (path, "execute") if path.is_file() => {
            let output = std::process::Command::new(path)
                .args(arguments.unwrap_or_default().split(' '))
                .output();
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let result = String::from_utf8_lossy(&output.stdout).to_string();
                        FileOperateResult::succ(result)
                    } else {
                        let result = String::from_utf8_lossy(&output.stderr).to_string();
                        FileOperateResult::fail(format!("Execute error: {result}"))
                    }
                }
                Err(e) => FileOperateResult::fail(format!("Execute error: {e}")),
            }
        }
        (path, "read") => {
            if path.is_dir() {
                match std::fs::read_dir(path) {
                    Ok(read_dir) => {
                        let mut items = vec![];
                        for entry in read_dir.flatten() {
                            let entry_path = entry.path();
                            let entry_name =
                                entry_path.file_name().unwrap_or_default().to_string_lossy();
                            if entry_path.is_dir() {
                                items.push(DirItem::dir(entry_name))
                            } else if entry_path.is_file() {
                                items.push(DirItem::file(entry_name))
                            } else if entry_path.is_symlink() {
                                items.push(DirItem::symlink(entry_name))
                            } else {
                                items.push(DirItem::unknown(entry_name))
                            }
                        }
                        FileOperateResult::succ(json!(items))
                    }
                    Err(e) => FileOperateResult::fail(format!("Read dir error: {e}")),
                }
            } else if path.is_file() {
                match std::fs::read_to_string(path) {
                    Ok(content) => FileOperateResult::succ(content),
                    Err(e) => FileOperateResult::fail(format!("Read file error: {e}")),
                }
            } else {
                FileOperateResult::fail("Invalid path")
            }
        }
        _ => FileOperateResult::fail("Invalid operate for path"),
    }
}
