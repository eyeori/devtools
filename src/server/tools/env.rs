use crate::server::tools::meta::env::ListConfigKeysResult;
use crate::server::tools::meta::EmptyParams;
use crate::server::Server;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router, Json};

#[tool_router(router = tool_touter_env, vis = "pub")]
impl Server {
    #[tool(
        name = "env.list_config_keys",
        description = "List all local config keys"
    )]
    fn list_config_keys(&self, _: Parameters<EmptyParams>) -> Json<ListConfigKeysResult> {
        let result = ListConfigKeysResult::with_keys(self.load_config().keys());
        Json(result)
    }
}
