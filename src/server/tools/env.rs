use crate::server::tools::meta::env::ListConfigKeysResult;
use crate::server::tools::meta::EmptyParams;
use crate::server::Server;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router, Json};
use tracing::info;
use ulid::Ulid;

#[tool_router(router = tool_touter_env, vis = "pub")]
impl Server {
    #[tool(
        name = "env::list_config_keys",
        description = "List all local config keys"
    )]
    fn list_config_keys(&self, _: Parameters<EmptyParams>) -> Json<ListConfigKeysResult> {
        let id = Ulid::new().to_string();
        info!("env::list_config_keys, id={id}");
        let result = ListConfigKeysResult::with_keys(self.load_config().keys());
        info!("env::list_config_keys, id={id}, result={result:?}");
        Json(result)
    }
}
