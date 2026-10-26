use crate::engine::cleaner::clean_email;
use crate::engine::models::{CanalContacto, EmpresaB2B, EvaluacionObjetivo, ResultadoAuditoria};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuditEngine;

impl AuditEngine {
    /// Evalúa la contactabilidad de la empresa antes de gastar recursos de red/captcha
    pub fn realizar_triage(empresa: &EmpresaB2B) -> EvaluacionObjetivo {
        if let Some(valid_email) = clean_email(&empresa.email) {
            return EvaluacionObjetivo {
                id_empresa: empresa.id.clone(),
                canal_optimo: CanalContacto::EmailDirecto(valid_email),
                score_efectividad: 95,
                requiere_bypass: false,
            };
        }

        if let Some(url) = &empresa.sitio_web {
            if url.contains("protected") || url.contains("cloudflare") {
                return EvaluacionObjetivo {
                    id_empresa: empresa.id.clone(),
                    canal_optimo: CanalContacto::FormularioConCaptcha {
                        url: url.clone(),
                        tipo: "Cloudflare/Turnstile".to_string(),
                    },
                    score_efectividad: 20,
                    requiere_bypass: true,
                };
            }

            return EvaluacionObjetivo {
                id_empresa: empresa.id.clone(),
                canal_optimo: CanalContacto::FormularioLimpio(url.clone()),
                score_efectividad: 80,
                requiere_bypass: false,
            };
        }

        EvaluacionObjetivo {
            id_empresa: empresa.id.clone(),
            canal_optimo: CanalContacto::SinContactoUtil,
            score_efectividad: 0,
            requiere_bypass: false,
        }
    }

    /// Realiza la auditoría preliminar y genera el hash criptográfico SHA-256 del reporte
    pub async fn auditar_empresa(empresa: &EmpresaB2B) -> ResultadoAuditoria {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let payload = format!("{}:{}:{}", empresa.id, empresa.nombre, now);

        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let hash_auditoria = format!("{:x}", hasher.finalize());

        ResultadoAuditoria {
            id_empresa: empresa.id.clone(),
            estado_riesgo: if empresa.risk_score > 70 {
                "CRÍTICO".to_string()
            } else {
                "BAJO".to_string()
            },
            score_cumplimiento: 100u8.saturating_sub(empresa.risk_score.min(100) as u8),
            hash_auditoria,
            merkle_root: None,
            timestamp: now,
        }
    }
}
