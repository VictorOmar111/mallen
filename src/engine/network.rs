use reqwest::{Client, Proxy};
use std::time::Duration;

pub struct NetworkClientManager;

impl NetworkClientManager {
    /// Construye un cliente HTTP optimizado con soporte opcional para proxies rotativos
    pub fn build_client(proxy_url: Option<&str>) -> Result<Client, Box<dyn std::error::Error>> {
        let mut builder = Client::builder()
            .timeout(Duration::from_millis(3000))
            .pool_max_idle_per_host(50)
            .danger_accept_invalid_certs(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) B2BAuditBot/5.0");

        // Si se provee un proxy (ej. SOCKS5 o HTTP), se inyecta al cliente
        if let Some(p_url) = proxy_url {
            let proxy = Proxy::all(p_url)?;
            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;
        Ok(client)
    }
}
