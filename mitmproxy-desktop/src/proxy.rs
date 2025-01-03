use anyhow::Result;
use hudsucker::{
    certificate_authority::OpensslAuthority,
    hyper::{Request, Response},
    openssl::{hash::MessageDigest, pkey::PKey, x509::X509},
    rustls::crypto::aws_lc_rs,
    tokio_tungstenite::tungstenite::Message,
    Body, HttpContext, HttpHandler, Proxy, RequestOrResponse, WebSocketContext, WebSocketHandler,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tracing::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ProxyState {
    Running,
    Stopped,
    Error,
}

#[derive(Clone)]
struct ProxyHandler;

impl HttpHandler for ProxyHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        info!("Intercepted request to {}: {:?}", req.uri(), req);
        req.into()
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        info!("Intercepted response: {:?}", res);
        res
    }
}

impl WebSocketHandler for ProxyHandler {
    async fn handle_message(&mut self, _ctx: &WebSocketContext, msg: Message) -> Option<Message> {
        info!("Intercepted WebSocket message: {:?}", msg);
        Some(msg)
    }
}

#[derive(Clone)]
pub struct ProxyManager {
    shutdown_sender: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl ProxyManager {
    pub fn new() -> Self {
        Self {
            shutdown_sender: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self, port: u16, cert_path: Option<String>) -> Result<()> {
        let mut sender = self.shutdown_sender.lock().await;
        if sender.is_some() {
            return Ok(());
        }

        // Load certificate and private key
        let cert_path = cert_path.ok_or_else(|| anyhow::anyhow!("Certificate path not set"))?;
        let cert_contents = tokio::fs::read(&cert_path).await?;

        // For this example, we're using the same file for both cert and key
        let ca_cert = X509::from_pem(&cert_contents)?;
        let private_key = PKey::private_key_from_pem(&cert_contents)?;

        // Create OpenSSL-based authority
        let ca = OpensslAuthority::new(
            private_key,
            ca_cert,
            MessageDigest::sha256(),
            1_000,
            aws_lc_rs::default_provider(),
        );

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        *sender = Some(shutdown_tx);

        // Build and start the proxy
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let proxy = Proxy::builder()
            .with_addr(addr)
            .with_ca(ca)
            .with_rustls_client(aws_lc_rs::default_provider())
            .with_http_handler(ProxyHandler)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .build()?;

        info!("Starting proxy on {}", addr);

        tokio::spawn(async move {
            if let Err(e) = proxy.start().await {
                error!("Proxy error: {}", e);
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut sender = self.shutdown_sender.lock().await;
        if let Some(s) = sender.take() {
            let _ = s.send(());
        }
        Ok(())
    }
}
