use crate::server::tools::meta::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileOperateParams {
    /// The path to the file
    pub path: String,

    /// The operation, which can be "read" or "execute"
    pub operate: String,

    /// The arguments for the "execute" operation
    pub arguments: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileOperateResult {
    pub success: bool,
    pub message: String,
    pub result: String,
}

impl FileOperateResult {
    pub fn succ(result: impl ToString) -> Self {
        Self {
            success: true,
            message: "OK".to_string(),
            result: result.to_string(),
        }
    }

    pub fn fail(msg: impl ToString) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            result: "".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct DirItem {
    name: String,
    r#type: String,
}

impl DirItem {
    pub fn dir(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            r#type: "dir".to_string(),
        }
    }

    pub fn file(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            r#type: "file".to_string(),
        }
    }

    pub fn symlink(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            r#type: "symlink".to_string(),
        }
    }

    pub fn unknown(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            r#type: "unknown".to_string(),
        }
    }
}
