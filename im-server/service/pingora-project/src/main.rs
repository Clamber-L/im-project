use anyhow::Result;
use lib_core::init_logger;
use pingora::prelude::*;
use pingora_project::SimpleProxy;

const PROXY_ADDR: &'static str = "0.0.0.0:9000";

fn main() -> Result<()> {
    init_logger("pingora");
    tracing::info!("Starting server on {}", PROXY_ADDR);

    let mut server = Server::new(None)?;
    server.bootstrap();

    let sp = SimpleProxy {};
    let mut proxy = http_proxy_service(&server.configuration, sp);
    proxy.add_tcp(PROXY_ADDR);
    server.add_service(proxy);

    server.run_forever();
}
