use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListConfigKeysResult {
    pub keys: Vec<String>,
}

impl ListConfigKeysResult {
    #[inline]
    pub fn with_keys(keys: Vec<String>) -> Self {
        Self { keys }
    }
}
