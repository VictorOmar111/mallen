use regex::Regex;
use reqwest::Client;
use std::time::Duration;

/// Cliente de enriquecimiento web B2B asíncrono.
pub struct WebEnricher {
    client: Client,
    email_regex: Regex,
}

impl WebEnricher {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .user_agent("Mozilla/5.0 (compatible; B2BEnricherBot/1.0)")
            .build()
            .expect("No se pudo construir el cliente HTTP");

        let email_regex =
            Regex::new(r"[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}").unwrap();

        Self {
            client,
            email_regex,
        }
    }

    /// Realiza un GET rápido y retorna (status_code, primer_email_encontrado).
    pub async fn inspect_site(&self, url: &str) -> (Option<u16>, Option<String>) {
        let response = match self.client.get(url).send().await {
            Ok(resp) => resp,
            Err(_) => return (None, None),
        };

        let status = response.status().as_u16();

        let body = match response.text().await {
            Ok(text) => text,
            Err(_) => return (Some(status), None),
        };

        let email = self
            .email_regex
            .find(&body)
            .map(|m| m.as_str().to_lowercase());

        (Some(status), email)
    }
}

impl Default for WebEnricher {
    fn default() -> Self {
        Self::new()
    }
}
