//! Utilidades de normalización, limpieza del DENUE, validación de RFC canónico,
//! escudo Anti-SSRF para el crawler OSINT y sanitización PII (LFPDPPP / SABG).

use once_cell::sync::Lazy;
use regex::Regex;
use std::net::IpAddr;
use url::Url;

const INVALID_TOKENS: [&str; 6] = ["n/a", "null", "-", "na", "none", "sin dato"];

/// Regex oficial para RFC mexicano (Personas Morales: 12 chars, Personas Físicas: 13 chars)
static RE_RFC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-ZÑ&]{3,4}\d{6}[A-Z0-9]{3}$").expect("Regex RFC inválida")
});

/// Regex oficial para detectar CURP en textos escaneados (Sanitización PII)
static RE_CURP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[A-Z][AEIOUX][A-Z]{2}\d{6}[HM][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d\b")
        .expect("Regex CURP inválida")
});

/// Regex para detectar correos electrónicos personales en textos crudos de scraping
static RE_EMAIL_PII: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b[a-z0-9._%+-]+@(gmail|hotmail|outlook|yahoo|icloud|live)\.(com|mx|es)\b")
        .expect("Regex Email PII inválida")
});

/// Regex para detectar teléfonos celulares de 10 dígitos aislados en volcados de texto
static RE_PHONE_PII: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(?:55|56|33|81|\d{2})[\s-]?\d{4}[\s-]?\d{4}\b")
        .expect("Regex Phone PII inválida")
});

fn is_invalid(raw: &str) -> bool {
    let trimmed = raw.trim();
    trimmed.is_empty() || INVALID_TOKENS.contains(&trimmed.to_lowercase().as_str())
}

/// Normaliza una URL agregando "https://" si falta, descartando valores inválidos
/// y aplicando el escudo Anti-SSRF para impedir escaneos a IPs internas o metadatos AWS.
pub fn clean_url(raw: &str) -> Option<String> {
    if is_invalid(raw) {
        return None;
    }

    let trimmed = raw.trim().replace(' ', "");
    if trimmed.is_empty() {
        return None;
    }

    let candidate = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed
    } else {
        format!("https://{}", trimmed)
    };

    validate_safe_target_url(&candidate)
}

/// Escudo Anti-SSRF: Verifica que la URL tenga esquema HTTP/HTTPS válido y no apunte
/// a localhost, redes privadas (RFC 1918) ni al servicio de metadatos de AWS EC2 (169.254.169.254).
pub fn validate_safe_target_url(raw_url: &str) -> Option<String> {
    let parsed = Url::parse(raw_url).ok()?;

    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return None;
    }

    let host_str = parsed.host_str()?.to_lowercase();

    // Bloqueo de hosts locales o internos por nombre
    if host_str == "localhost"
        || host_str.ends_with(".local")
        || host_str.ends_with(".internal")
        || host_str == "metadata.google.internal"
        || !host_str.contains('.')
    {
        return None;
    }

    // Si el host es una dirección IP directa, validar que sea 100% pública
    if let Ok(ip) = host_str.parse::<IpAddr>() {
        if is_private_or_reserved_ip(&ip) {
            return None;
        }
    }

    Some(parsed.to_string())
}

/// Determina si una IP pertenece a rangos privados, loopback o link-local (AWS Metadata).
pub fn is_private_or_reserved_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            ipv4.is_loopback()
                || ipv4.is_private()
                || ipv4.is_link_local() // Bloquea 169.254.0.0/16 (AWS EC2 IAM Metadata)
                || ipv4.is_broadcast()
                || ipv4.is_documentation()
                || ipv4.is_unspecified()
                || ipv4.octets()[0] == 100 && (ipv4.octets()[1] & 0b1100_0000 == 0b0100_0000) // CGNAT 100.64.0.0/10
        }
        IpAddr::V6(ipv6) => {
            ipv6.is_loopback()
                || ipv6.is_unspecified()
                || ipv6.is_multicast()
                || (ipv6.segments()[0] & 0xfe00) == 0xfc00 // Unique Local (fc00::/7)
                || (ipv6.segments()[0] & 0xffc0) == 0xfe80 // Link-Local (fe80::/10)
        }
    }
}

/// Extrae únicamente dígitos numéricos y valida que tenga al menos 10 dígitos.
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

/// Valida y normaliza un RFC a su formato canónico estricto en mayúsculas (12 o 13 caracteres).
/// Evita falsos positivos por fuzzy matching al cruzar con listas negras SAT 69 y 69-B (EFOS).
pub fn normalize_canonical_rfc(raw: &str) -> Option<String> {
    if is_invalid(raw) {
        return None;
    }

    let cleaned: String = raw
        .trim()
        .to_uppercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .collect();

    // Excluir RFCs genéricos (Público en General / Extranjeros) de cruces de listas negras
    if cleaned == "XAXX010101000" || cleaned == "XEXX010101000" {
        return None;
    }

    if RE_RFC.is_match(&cleaned) {
        Some(cleaned)
    } else {
        None
    }
}

/// Normaliza una Razón Social o Nombre Comercial eliminando acentos, signos de puntuación
/// y sufijos societarios ("S.A. DE C.V.", "S.A.P.I.", etc.) para cruces FTS5 con DENUE y PROFECO.
pub fn normalize_company_name(raw: &str) -> String {
    let upper = raw.trim().to_uppercase();
    let ascii_mapped: String = upper
        .chars()
        .map(|c| match c {
            'Á' | 'À' | 'Ä' | 'Â' => 'A',
            'É' | 'È' | 'Ë' | 'Ê' => 'E',
            'Í' | 'Ì' | 'Ï' | 'Î' => 'I',
            'Ó' | 'Ò' | 'Ö' | 'Ô' => 'O',
            'Ú' | 'Ù' | 'Ü' | 'Û' => 'U',
            'Ñ' => 'N',
            c if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() => c,
            _ => ' ',
        })
        .collect();

    // Remover sufijos mercantiles comunes para mejorar el recall en el índice FTS5 del DENUE
    let stop_suffixes = [
        " S A P I DE C V",
        " S A DE C V",
        " S DE R L DE C V",
        " S A B DE C V",
        " S C",
        " A C",
        " SA DE CV",
        " SAPI DE CV",
    ];

    let mut collapsed = ascii_mapped
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    for suffix in stop_suffixes {
        if collapsed.ends_with(suffix) {
            let new_len = collapsed.len().saturating_sub(suffix.len());
            collapsed.truncate(new_len);
            break;
        }
    }

    collapsed.trim().to_string()
}

/// Filtro de Sanitización PII (Cumplimiento LFPDPPP / Autoridad Garante SABG):
/// Enmascara CURPs, correos personales y teléfonos móviles antes de persistir logs OSINT en SQLite.
pub fn sanitize_pii_text(raw_text: &str) -> String {
    let without_curp = RE_CURP.replace_all(raw_text, "[CURP-PROTEGIDO-LFPDPPP]");
    let without_personal_email =
        RE_EMAIL_PII.replace_all(&without_curp, "[EMAIL-PERSONAL-PROTEGIDO]");
    let sanitized =
        RE_PHONE_PII.replace_all(&without_personal_email, "[TEL-PROTEGIDO]");
    sanitized.into_owned()
}


use sha2::{Sha256, Digest};

pub fn sha256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
