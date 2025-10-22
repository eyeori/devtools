use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub(super) mod encode;
pub(super) mod env;
pub(super) mod time;
pub(super) mod fs;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub(super) struct EmptyParams {}
