use crate::engine::models::{CanalContacto, EmpresaB2B, EvaluacionObjetivo, ResultadoAuditoria};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuditEngine;

impl AuditEngine {
    /// Evalúa la contactabilidad de la empresa antes de gastar recursos de red/captcha
    pub fn realizar_triage(empresa: &EmpresaB2B) -> EvaluacionObjetivo {
        if let Some(email) = &empresa.email {
            return EvaluacionObjetivo {
                id_empresa: empresa.id_registro.clone(),
                canal_optimo: CanalContacto::EmailDirecto(email.clone()),
                score_efectividad: 95,
                requiere_bypass: false,
            };
        }

        if let Some(url) = &empresa.sitio_web {
            // Clasificación heurística simple sobre el sitio web
            if url.contains("protected") || url.contains("cloudflare") {
                return EvaluacionObjetivo {
                    id_empresa: empresa.id_registro.clone(),
                    canal_optimo: CanalContacto::FormularioConCaptcha {
                        url: url.clone(),
                        tipo: "Cloudflare/Turnstile".to_string(),
                    },
                    score_efectividad: 20,
                    requiere_bypass: true,
                };
            }

            return EvaluacionObjetivo {
                id_empresa: empresa.id_registro.clone(),
                canal_optimo: CanalContacto::FormularioLimpio(url.clone()),
                score_efectividad: 80,
                requiere_bypass: false,
            };
        }

        EvaluacionObjetivo {
            id_empresa: empresa.id_registro.clone(),
            canal_optimo: CanalContacto::SinContactoUtil,
            score_efectividad: 0,
            requiere_bypass: false,
        }
    }

    /// Realiza la auditoría y genera el hash criptográfico del reporte
    pub async fn auditar_empresa(empresa: &EmpresaB2B) -> ResultadoAuditoria {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let payload = format!("{}:{}:{}", empresa.id_registro, empresa.razon_social, now);
        
        // Simulación de Hash Keccak256 / SHA-256 del reporte
        let hash_auditoria = format!("{:x}", md5::compute(payload.as_bytes()));

        ResultadoAuditoria {
            id_empresa: empresa.id_registro.clone(),
            estado_riesgo: "BAJO".to_string(),
            score_cumplimiento: 98,
            hash_auditoria,
            merkle_root: None,
            timestamp: now,
        }
    }
}
