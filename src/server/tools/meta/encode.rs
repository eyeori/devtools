use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Md5EncodeParams {
    /// The string to encode
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Base64EncodeDecodeParams {
    /// The string to encode or decode
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UrlEncodeDecodeParams {
    /// The url to encode or decode
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HexToStringParams {
    /// The hex data to decode, like: \xe4\xb8\xad\xe5\x9b\xbd, e4b8ade59bbd, 0xE4B8ADE59BBD
    pub data: String,
}
