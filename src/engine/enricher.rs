use regex::Regex;
use reqwest::Client;
use std::time::Duration;

/// Mehrsprachige Schlüsselwörter, die auf eine Kontakt-/Impressumsseite hindeuten.
/// Abgedeckt: Spanisch, Englisch, Deutsch, Französisch, Portugiesisch, Italienisch.
const CONTACT_KEYWORDS: &[&str] = &[
    // Spanisch
    "contacto", "contactanos", "contáctanos", "aviso legal", "quienes somos",
    // Englisch
    "contact", "about", "about-us", "contact-us", "imprint", "legal",
    // Deutsch
    "kontakt", "impressum", "uber uns", "über uns", "datenschutz",
    // Französisch
    "contactez", "a propos", "à propos", "mentions legales", "mentions légales",
    // Portugiesisch
    "contato", "fale conosco", "quem somos",
    // Italienisch
    "contatti", "chi siamo", "note legali",
];

/// Cliente de enriquecimiento web B2B asíncrono (multilingüe).
pub struct WebEnricher {
    client: Client,
    email_regex: Regex,
    contact_regex: Regex,
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

        // Case-insensitive Suche nach mehrsprachigen Kontakt-Schlüsselwörtern.
        let contact_regex = Regex::new(&format!(
            r"(?i)({})",
            CONTACT_KEYWORDS
                .iter()
                .map(|kw| regex::escape(kw))
                .collect::<Vec<_>>()
                .join("|")
        ))
        .unwrap();

        Self {
            client,
            email_regex,
            contact_regex,
        }
    }

    /// Prüft, ob der Seitentext ein mehrsprachiges Kontakt-Schlüsselwort enthält.
    pub fn has_contact_keyword(&self, body: &str) -> bool {
        self.contact_regex.is_match(body)
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
