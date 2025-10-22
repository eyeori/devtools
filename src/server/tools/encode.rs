use crate::server::tools::meta::encode::*;
use crate::server::Server;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use md5::{Digest, Md5};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use tracing::info;
use ulid::Ulid;

#[tool_router(router = tool_touter_encode, vis = "pub")]
impl Server {
    #[tool(name = "encode::md5_encode", description = "Md5 encode")]
    fn md5_encode(&self, Parameters(params): Parameters<Md5EncodeParams>) -> String {
        let id = Ulid::new().to_string();
        info!("encode::md5_encode, id={id}, params={params:?}");
        let mut hasher = Md5::new();
        hasher.update(params.data.as_bytes());
        let hash = hasher.finalize();
        let result = hex::encode(hash.as_slice());
        info!("encode::md5_encode, id={id}, result={result:?}");
        result
    }

    #[tool(name = "encode::base64_encode", description = "Base64 encode")]
    fn base64_encode(&self, Parameters(params): Parameters<Base64EncodeDecodeParams>) -> String {
        let id = Ulid::new().to_string();
        info!("encode::base64_encode, id={id}, params={params:?}");
        let result = STANDARD.encode(params.data.as_bytes());
        info!("encode::base64_encode, id={id}, result={result:?}");
        result
    }

    #[tool(name = "encode::base64_decode", description = "Base64 decode")]
    fn base64_decode(
        &self,
        Parameters(params): Parameters<Base64EncodeDecodeParams>,
    ) -> Result<String, String> {
        let id = Ulid::new().to_string();
        info!("encode::base64_decode, id={id}, params={params:?}");
        let result = STANDARD
            .decode(params.data.as_bytes())
            .map(|data| String::from_utf8_lossy(&data).to_string())
            .map_err(|e| e.to_string());
        info!("encode::base64_decode, id={id}, result={result:?}");
        result
    }

    #[tool(name = "encode::url_encode", description = "URL encode")]
    fn url_encode(&self, Parameters(params): Parameters<UrlEncodeDecodeParams>) -> String {
        let id = Ulid::new().to_string();
        info!("encode::url_encode, id={id}, params={params:?}");
        let result: String = form_urlencoded::byte_serialize(params.url.as_bytes()).collect();
        info!("encode::url_encode, id={id}, result={result:?}");
        result
    }

    #[tool(name = "encode::url_decode", description = "URL decode")]
    fn url_decode(&self, Parameters(params): Parameters<UrlEncodeDecodeParams>) -> String {
        let id = Ulid::new().to_string();
        info!("encode::url_decode, id={id}, params={params:?}");
        let result = form_urlencoded::parse(params.url.as_bytes())
            .map(|(k, v)| {
                if v.is_empty() {
                    k.to_string()
                } else {
                    format!("{k}={v}")
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        info!("encode::url_decode, id={id}, result={result:?}");
        result
    }

    #[tool(
        name = "encode::hex_to_string",
        description = "Hex data to utf8 string"
    )]
    fn hex_to_string(
        &self,
        Parameters(params): Parameters<HexToStringParams>,
    ) -> Result<String, String> {
        let id = Ulid::new().to_string();
        info!("encode::hex_to_string, id={id}, params={params:?}");
        let result = params
            .data
            .to_ascii_lowercase()
            .replace(" ", "")
            .replace("\\x", "")
            .trim_start_matches("0x")
            .as_bytes()
            .chunks(2)
            .map(|chunk| str::from_utf8(chunk).map_err(|e| e.to_string()))
            .try_fold(Vec::new(), |mut acc, item| {
                acc.push(item?);
                Ok::<Vec<&str>, String>(acc)
            })?
            .into_iter()
            .map(|hex_str| u8::from_str_radix(hex_str, 16).map_err(|e| e.to_string()))
            .try_fold(Vec::new(), |mut acc, item| {
                acc.push(item?);
                Ok(acc)
            })
            .map(|hex| String::from_utf8_lossy(&hex).to_string());
        info!("encode::hex_to_string, id={id}, result={result:?}");
        result
    }
}
