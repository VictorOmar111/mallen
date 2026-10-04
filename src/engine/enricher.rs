use crate::engine::models::EmpresaB2B;
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

/// Mehrsprachige Schlüsselwörter, die auf eine Datenschutz-/Privatsphäre-Seite hindeuten.
/// Abgedeckt: Spanisch, Englisch, Deutsch, Französisch, Portugiesisch, Italienisch.
const PRIVACY_KEYWORDS: &[&str] = &[
    // Spanisch
    "aviso de privacidad", "politica de privacidad", "política de privacidad",
    "privacidad", "proteccion de datos", "protección de datos",
    // Englisch
    "privacy policy", "privacy", "data protection",
    // Deutsch
    "datenschutz", "datenschutzerklarung", "datenschutzerklärung",
    // Französisch
    "politique de confidentialite", "politique de confidentialité", "confidentialite",
    // Portugiesisch
    "politica de privacidade", "política de privacidade", "privacidade",
    // Italienisch
    "informativa sulla privacy", "trattamento dei dati",
];

/// Cliente de enriquecimiento web B2B asíncrono (multilingüe).
pub struct WebEnricher {
    client: Client,
    email_regex: Regex,
    contact_regex: Regex,
    privacy_regex: Regex,
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

        // Case-insensitive Suche nach mehrsprachigen Datenschutz-Schlüsselwörtern.
        let privacy_regex = Regex::new(&format!(
            r"(?i)({})",
            PRIVACY_KEYWORDS
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
            privacy_regex,
        }
    }

    /// Prüft, ob der Seitentext ein mehrsprachiges Kontakt-Schlüsselwort enthält.
    pub fn has_contact_keyword(&self, body: &str) -> bool {
        self.contact_regex.is_match(body)
    }

    /// Prüft, ob der Seitentext ein mehrsprachiges Datenschutz-Schlüsselwort enthält.
    pub fn has_privacy_keyword(&self, body: &str) -> bool {
        self.privacy_regex.is_match(body)
    }

    /// Berechnet eine Pre-Auditoría-Risikopunktzahl (0–100) samt Etiketten.
    ///
    /// Regeln:
    /// - `SIN_SSL` (+40): URL verwendet kein HTTPS.
    /// - `SIN_AVISO_PRIVACIDAD` (+30): kein Datenschutz-/Privatsphäre-Hinweis gefunden.
    /// - `SIN_CONTACTO` (+30): kein Kontakt-Schlüsselwort gefunden.
    ///
    /// Der Score wird bei 100 gedeckelt.
    pub fn calculate_pre_audit_risk(&self, url: &str, html_body: &str) -> (u8, Vec<String>) {
        let mut score: u16 = 0;
        let mut flags: Vec<String> = Vec::new();

        if !url.trim().to_lowercase().starts_with("https://") {
            score += 40;
            flags.push("SIN_SSL".to_string());
        }

        if !self.has_privacy_keyword(html_body) {
            score += 30;
            flags.push("SIN_AVISO_PRIVACIDAD".to_string());
        }

        if !self.has_contact_keyword(html_body) {
            score += 30;
            flags.push("SIN_CONTACTO".to_string());
        }

        (score.min(100) as u8, flags)
    }

    /// Realiza un GET rápido y retorna (status_code, primer_email_encontrado, html_body).
    pub async fn inspect_site(
        &self,
        url: &str,
    ) -> (Option<u16>, Option<String>, Option<String>) {
        let response = match self.client.get(url).send().await {
            Ok(resp) => resp,
            Err(_) => return (None, None, None),
        };

        let status = response.status().as_u16();

        let body = match response.text().await {
            Ok(text) => text,
            Err(_) => return (Some(status), None, None),
        };

        let email = self
            .email_regex
            .find(&body)
            .map(|m| m.as_str().to_lowercase());

        (Some(status), email, Some(body))
    }
}

impl Default for WebEnricher {
    fn default() -> Self {
        Self::new()
    }
}

/// Schlüsselwörter, die auf einen Datenschutzhinweis (LFPDPPP / ARCO) hindeuten.
const PRIVACY_BODY_KEYWORDS: &[&str] = &[
    "aviso de privacidad",
    "derechos arco",
    "lfpdppp",
];

/// Schlüsselwörter, die auf einen E-Commerce-Shop hindeuten.
const ECOMMERCE_KEYWORDS: &[&str] = &[
    "checkout",
    "carrito",
    "stripe",
    "shopify",
];

/// Führt einen tiefen Website-Scan für eine einzelne Firma durch.
///
/// - Sendet einen GET-Request an `empresa.sitio_web`.
/// - Bei HTTP 200 werden SSL, Datenschutz-Hinweise (LFPDPPP/ARCO) und
///   E-Commerce-Indikatoren geprüft.
/// - Die erkannten Flags werden an `empresa.audit_flags` angehängt.
pub async fn deep_scan_website(empresa: &mut EmpresaB2B) {
    let url = match empresa.sitio_web.as_ref() {
        Some(u) if !u.trim().is_empty() => u.trim().to_string(),
        _ => return,
    };

    // SSL-Prüfung anhand des URL-Schemas.
    let has_ssl = url.to_lowercase().starts_with("https://");
    if !has_ssl {
        empresa.audit_flags.push("SSL_INVALIDO".to_string());
    }

    let client = match Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent("Mozilla/5.0 (compatible; B2BDeepScanner/1.0)")
        .build()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(_) => return,
    };

    let status = response.status().as_u16();
    empresa.status_web = Some(status);

    if status != 200 {
        return;
    }

    let body = match response.text().await {
        Ok(text) => text.to_lowercase(),
        Err(_) => return,
    };

    // Datenschutz / LFPDPPP / ARCO
    let has_privacy = PRIVACY_BODY_KEYWORDS
        .iter()
        .any(|kw| body.contains(kw));

    if has_privacy {
        empresa
            .audit_flags
            .push("LFPDPPP_AVISO_ENCONTRADO".to_string());
    } else {
        empresa
            .audit_flags
            .push("INAI_SIN_AVISO_PRIVACIDAD".to_string());
    }

    // E-Commerce-Erkennung
    let has_ecommerce = ECOMMERCE_KEYWORDS.iter().any(|kw| body.contains(kw));
    if has_ecommerce {
        empresa
            .audit_flags
            .push("PROFECO_ECOMMERCE_DETECTADO".to_string());
    }
}
