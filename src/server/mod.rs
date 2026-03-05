mod tools;

use crate::common::config::Config;
use crate::common::env::exe_dir;
use anyhow::Result;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::*;
use rmcp::{tool_handler, ServerHandler};
use std::path::PathBuf;
use tracing::info;

#[derive(Clone)]
pub struct Server(Option<PathBuf>);

impl Server {
    pub fn new(config: Option<PathBuf>) -> Self {
        Self(config)
    }

    fn tool_router(&self) -> ToolRouter<Self> {
        let mut tool_router = ToolRouter::new();
        tool_router += Self::tool_touter_env();
        tool_router += Self::tool_touter_time();
        tool_router += Self::tool_touter_encode();
        tool_router += Self::tool_touter_fs();
        tool_router
    }

    #[allow(dead_code)]
    fn load_config(&self) -> Config {
        if let Some(path) = self.0.as_deref() {
            let config = Config::load_from_file(path);
            info!("load config from {path:?}, config={config:?}");
            config
        } else if let Some(dir) = exe_dir() {
            let path = dir.join("config.yaml");
            let config = Config::load_from_file(path.as_path());
            info!("load config from {path:?}, config={config:?}");
            config
        } else {
            Default::default()
        }
    }
}

#[tool_handler(router = self.tool_router())]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_experimental()
                .enable_logging()
                .enable_completions()
                .enable_prompts_with(PromptsCapability {
                    list_changed: Some(true),
                })
                .enable_resources_with(ResourcesCapability {
                    subscribe: Some(true),
                    list_changed: Some(true),
                })
                .enable_tools_with(ToolsCapability {
                    list_changed: Some(true),
                })
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
    }
}

#[cfg(test)]
mod test {
    use crate::server::Server;

    #[test]
    fn test() {
        let server = Server::new(None);
        let tools = server.tool_router().list_all();
        for tool in tools {
            let name = tool.name.as_ref();
            let input_schema = tool.input_schema.as_ref();
            println!("{name:<30} -> {input_schema:?}");
        }
    }
}
