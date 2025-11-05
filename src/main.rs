mod cli;
mod common;
mod server;

use crate::cli::Args;
use crate::common::logger;
use crate::server::Server;
use anyhow::Result;
use clap::Parser;
use rmcp::transport::stdio;
use rmcp::ServiceExt;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        println!("shutdown with error, e={e}");
        exit(-1)
    }
}

#[tokio::main]
async fn run() -> Result<()> {
    // load cli args
    let args = Args::parse();

    // init log
    let _log_guard = logger::init(&args.log_str)?;

    // start server
    let server = Server::new(args.config);
    let transport = stdio();
    let service = server.serve(transport).await?;
    service.waiting().await?;
    Ok(())
}
