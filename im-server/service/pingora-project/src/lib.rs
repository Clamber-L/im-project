use async_trait::async_trait;
use pingora::prelude::*;

pub struct SimpleProxy {}

#[async_trait]
impl ProxyHttp for SimpleProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = HttpPeer::new("127.0.0.1:8100".to_string(), false, "localhost".to_string());
        Ok(Box::new(peer))
    }
}
