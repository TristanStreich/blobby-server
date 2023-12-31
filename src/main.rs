mod clients;
mod env;
mod error;
mod server;

use error::{
    Context,
    MyResult,
};

use log::info;

use env::ENV;

#[tokio::main]
async fn main() -> MyResult<()> {
    env_logger::builder()
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    info!("Starting Server With Env {:#?}", *ENV);

    info!("Starting Server on Port {}", ENV.port0);
    server::start_server().await.context("Server Crashed")
}
