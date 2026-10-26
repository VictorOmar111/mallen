//! Gestor de red OSINT con Par Dual TLS (`strict_client` + `insecure_client`),
//! cabeceras de imitación de navegador comercial para evasión de WAF y control de concurrencia
//! por dominio mediante `tokio::sync::Semaphore`.

use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, UPGRADE_INSECURE_REQUESTS, USER_AGENT},
    Client, Proxy,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};

const CHROME_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

/// Par de clientes HTTP para el Vector LFPDPPP Art. 19 / PROFECO Art. 128 BIS:
/// 1. `strict`: Valida estrictamente la cadena de certificados TLS.
/// 2. `insecure`: Permite leer el DOM aunque el certificado esté vencido o autofirmado, registrando la infracción.
#[derive(Clone)]
pub struct DualTlsClientPair {
    pub strict: Client,
    pub insecure: Client,
}

pub struct NetworkClientManager;

impl NetworkClientManager {
    fn browser_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(CHROME_USER_AGENT));
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
        );
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("es-MX,es;q=0.9,en-US;q=0.8,en;q=0.7"),
        );
        headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
        headers
    }

    /// Función original 100% retrocompatible con tu código existente.
    pub fn build_client(proxy_url: Option<&str>) -> Result<Client, Box<dyn std::error::Error>> {
        let mut builder = Client::builder()
            .timeout(Duration::from_millis(4500))
            .pool_max_idle_per_host(50)
            .danger_accept_invalid_certs(true)
            .default_headers(Self::browser_headers());

        if let Some(p_url) = proxy_url {
            if !p_url.trim().is_empty() {
                let proxy = Proxy::all(p_url)?;
                builder = builder.proxy(proxy);
            }
        }

        Ok(builder.build()?)
    }

    /// Construye el par Dual TLS (`strict` + `insecure`) para detectar certificados SSL vencidos
    /// sin abortar la auditoría del texto del Aviso de Privacidad ni Términos de E-commerce.
    pub fn build_dual_tls_pair(
        proxy_url: Option<&str>,
    ) -> Result<DualTlsClientPair, Box<dyn std::error::Error + Send + Sync>> {
        let mut strict_builder = Client::builder()
            .timeout(Duration::from_millis(4500))
            .pool_max_idle_per_host(50)
            .danger_accept_invalid_certs(false)
            .default_headers(Self::browser_headers());

        let mut insecure_builder = Client::builder()
            .timeout(Duration::from_millis(4500))
            .pool_max_idle_per_host(50)
            .danger_accept_invalid_certs(true)
            .default_headers(Self::browser_headers());

        if let Some(p_url) = proxy_url {
            if !p_url.trim().is_empty() {
                strict_builder = strict_builder.proxy(Proxy::all(p_url)?);
                insecure_builder = insecure_builder.proxy(Proxy::all(p_url)?);
            }
        }

        Ok(DualTlsClientPair {
            strict: strict_builder.build()?,
            insecure: insecure_builder.build()?,
        })
    }
}

/// Controlador de concurrencia por dominio (Evita activar WAFs gubernamentales o quemar IPs en ráfagas B2B2B).
#[derive(Clone, Default)]
pub struct DomainRateLimiter {
    semaphores: Arc<Mutex<HashMap<String, Arc<Semaphore>>>>,
}

impl DomainRateLimiter {
    pub fn new() -> Self {
        Self {
            semaphores: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adquiere un permiso concurrente para un dominio específico (ej. máx 3 peticiones simultáneas por host).
    pub async fn acquire_domain_permit(
        &self,
        domain: &str,
        max_concurrent: usize,
    ) -> OwnedSemaphorePermit {
        let sem = {
            let mut map = self.semaphores.lock().await;
            map.entry(domain.to_lowercase())
                .or_insert_with(|| Arc::new(Semaphore::new(max_concurrent.max(1))))
                .clone()
        };
        sem.acquire_owned().await.expect("Semaphore cerrado inesperadamente")
    }
}
