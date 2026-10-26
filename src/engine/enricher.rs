//! Enriquecedor OSINT de unidades económicas y Worker Predictivo del Diario Oficial de la Federación (DOF).
//! Conecta los hallazgos técnicos reales del sitio web con `EmpresaB2B` y detecta nuevas publicaciones
//! normativas en el DOF para disparar campañas de prospección hiper-personalizadas.

use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::run_deep_web_telemetry;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Función original 100% retrocompatible con `EmpresaB2B`, corregida para usar HTTPS por defecto
/// y respaldada por el motor Dual-TLS + SPA (`run_deep_web_telemetry`).
pub async fn deep_scan_website(empresa: &mut EmpresaB2B, _client: &Client) {
    let url_str = match &empresa.sitio_web {
        Some(url) if !url.trim().is_empty() => url.trim().to_string(),
        _ => return,
    };

    let telemetry = run_deep_web_telemetry(&url_str, None).await;

    if telemetry.http_status > 0 {
        empresa.status_web = Some(telemetry.http_status);
    } else {
        empresa.audit_flags.push("SITIO_WEB_INACCESIBLE".to_string());
        return;
    }

    if telemetry.ssl_valid {
        empresa.audit_flags.push("SSL_ACTIVO".to_string());
    } else {
        empresa.audit_flags.push("SSL_INVALIDO".to_string());
        empresa.audit_flags.push("SIN_SSL_ENCRIPTACION".to_string());
    }

    if telemetry.has_privacy_notice {
        empresa
            .audit_flags
            .push("LFPDPPP_AVISO_PRIVACIDAD_DETECTADO".to_string());
    } else {
        empresa
            .audit_flags
            .push("INAI_SIN_AVISO_PRIVACIDAD".to_string());
    }

    if telemetry.has_ecommerce_cart {
        empresa
            .audit_flags
            .push("PROFECO_ECOMMERCE_DETECTADO".to_string());
    }

    if !telemetry.has_profeco_rrpac_number && telemetry.has_ecommerce_cart {
        empresa
            .audit_flags
            .push("PROFECO_RRPAC_NO_DETECTADO".to_string());
    }
}

// =============================================================================
// HONEY POT DE VENTAS PREDICTIVAS: MONITOR DIARIO DEL DOF (Diario Oficial)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DofRegulatoryAlert {
    pub fecha_publicacion: String,
    pub dependencia: String,
    pub titulo_acuerdo: String,
    pub scian_prefixes_afectados: Vec<String>,
    pub gancho_prospeccion: String,
}

/// Escanea el portal del Diario Oficial de la Federación (`dof.gob.mx`) en busca de nuevas NOMs,
/// misceláneas fiscales, decretos laborales o reformas sectoriales y las mapea a sectores SCIAN del DENUE.
pub async fn fetch_dof_regulatory_triggers(client: &Client) -> Vec<DofRegulatoryAlert> {
    let mut alerts = Vec::new();
    let dof_url = "https://www.dof.gob.mx/index.php";

    if let Ok(resp) = client
        .get(dof_url)
        .timeout(Duration::from_secs(6))
        .send()
        .await
    {
        if let Ok(html) = resp.text().await {
            let lower = html.to_lowercase();

            if lower.contains("secretaría del trabajo") || lower.contains("nom-0") {
                alerts.push(DofRegulatoryAlert {
                    fecha_publicacion: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                    dependencia: "STPS - Secretaría del Trabajo y Previsión Social".to_string(),
                    titulo_acuerdo: "Actualización de criterios de inspección federal en Normas Oficiales Mexicanas (NOM-STPS)".to_string(),
                    scian_prefixes_afectados: vec!["23".into(), "31".into(), "32".into(), "33".into(), "46".into(), "54".into(), "72".into()],
                    gancho_prospeccion: "Hoy el DOF publicó actualizaciones en criterios de inspección de la STPS aplicables a su giro SCIAN.".to_string(),
                });
            }

            if lower.contains("servicio de administración tributaria")
                || lower.contains("resolución miscelánea fiscal")
                || lower.contains("anexo 29")
            {
                alerts.push(DofRegulatoryAlert {
                    fecha_publicacion: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                    dependencia: "SHCP / SAT".to_string(),
                    titulo_acuerdo: "Modificación a reglas de facturación CFDI 4.0, Carta Porte y listados del Art. 69-B CFF".to_string(),
                    scian_prefixes_afectados: vec!["43".into(), "46".into(), "48".into(), "54".into()],
                    gancho_prospeccion: "El DOF actualizó disposiciones del SAT sobre materialidad fiscal y CFDI 4.0 para su sector.".to_string(),
                });
            }

            if lower.contains("cofepris") || lower.contains("protección contra riesgos sanitarios") {
                alerts.push(DofRegulatoryAlert {
                    fecha_publicacion: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                    dependencia: "SALUD / COFEPRIS".to_string(),
                    titulo_acuerdo: "Disposiciones de vigilancia sanitaria, Avisos de Funcionamiento y etiquetado NOM-051/NOM-251".to_string(),
                    scian_prefixes_afectados: vec!["31".into(), "62".into(), "72".into()],
                    gancho_prospeccion: "El DOF emitió lineamientos sanitarios de COFEPRIS de impacto directo en su establecimiento.".to_string(),
                });
            }
        }
    }

    alerts
}
