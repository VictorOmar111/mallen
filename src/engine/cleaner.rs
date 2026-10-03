/// Utilidades de normalización y limpieza de datos crudos del DENUE.

const INVALID_TOKENS: [&str; 5] = ["n/a", "null", "-", "na", "none"];

fn is_invalid(raw: &str) -> bool {
    let trimmed = raw.trim();
    trimmed.is_empty() || INVALID_TOKENS.contains(&trimmed.to_lowercase().as_str())
}

/// Normaliza una URL agregando "https://" si falta y descartando valores no válidos.
#[allow(dead_code)]
pub fn clean_url(raw: &str) -> Option<String> {
    if is_invalid(raw) {
        return None;
    }

    let trimmed = raw.trim().replace(' ', "");

    if trimmed.is_empty() {
        return None;
    }

    let normalized = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed
    } else {
        format!("https://{}", trimmed)
    };

    Some(normalized)
}

/// Extrae únicamente dígitos numéricos y valida que tenga al menos 10 dígitos.
#[allow(dead_code)]
pub fn clean_phone(raw: &str) -> Option<String> {
    if is_invalid(raw) {
        return None;
    }

    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() >= 10 {
        Some(digits)
    } else {
        None
    }
}

/// Convierte a minúsculas, elimina espacios y valida que contenga '@' y un punto.
#[allow(dead_code)]
pub fn clean_email(raw: &str) -> Option<String> {
    if is_invalid(raw) {
        return None;
    }

    let normalized = raw.trim().to_lowercase().replace(' ', "");

    if normalized.contains('@') && normalized.contains('.') {
        Some(normalized)
    } else {
        None
    }
}
