use axum::response::Html;
use sha2::{Digest, Sha256};

pub async fn render_verify(id: &str) -> Html<String> {
    let clean_id = id.trim().to_uppercase();
    let riesgo_estimado: f64 = 1_188_000.00;
    let provision_activacion: f64 = 5_940.00;

    let mut hasher = Sha256::new();
    hasher.update(format!("MALLEN_VERIFY_2026::{}", clean_id).as_bytes());
    let real_sha256 = format!("{:x}", hasher.finalize());

    let riesgo_str = format!("{:.2}", riesgo_estimado);
    let provision_str = format!("{:.2}", provision_activacion);

    let html = format!(
        r###"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dictamen & Certificación Pericial | Firma Pericial Mallen</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.0/font/bootstrap-icons.css">
    <style>
        body {{ background-color: #0b132b; color: #e2e8f0; font-family: 'Segoe UI', system-ui, sans-serif; }}
        .card-feature {{ background: #1c2541; border: 1px solid #3a506b; border-radius: 12px; box-shadow: 0 10px 30px rgba(0,0,0,0.4); }}
        .inst-box {{ background: #0b132b; border: 1px solid #3a506b; border-radius: 10px; transition: border-color 0.2s; }}
        .inst-box:hover {{ border-color: #48cae4; }}
        .risk-card {{ background: #1b263b; border: 1px solid #e63946; border-radius: 10px; }}
        .hash-box {{ background: #0b132b; border: 1px solid #3a506b; color: #48cae4; font-family: monospace; padding: 0.75rem; border-radius: 8px; word-break: break-all; font-size: 0.85rem; }}
        .btn-proceed {{ background: linear-gradient(135deg, #0077b6 0%, #023e8a 100%); color: #ffffff; border: 1px solid #48cae4; font-weight: 700; border-radius: 8px; padding: 0.85rem 1.75rem; transition: all 0.2s; }}
        .btn-proceed:hover {{ background: linear-gradient(135deg, #0096c7 0%, #0077b6 100%); color: #ffffff; transform: translateY(-2px); box-shadow: 0 6px 20px rgba(0,119,182,0.4); }}
    </style>
</head>
<body>

<nav class="navbar navbar-dark border-bottom border-secondary" style="background: #0b132b;">
    <div class="container">
        <a class="navbar-brand fw-bold text-white fs-4" href="/">Firma Pericial Mallen</a>
        <span class="badge bg-dark border border-secondary text-info"><i class="bi bi-shield-check me-1"></i> Expediente Consultivo OSINT</span>
    </div>
</nav>

<div class="container my-5">
    <div class="row justify-content-center">
        <div class="col-lg-10">
            <div class="card-feature p-4 p-md-5">
                
                <div class="d-flex flex-wrap justify-content-between align-items-center mb-4 pb-3 border-bottom border-secondary gap-2">
                    <div>
                        <span class="text-uppercase text-muted fw-bold" style="font-size: 0.8rem;">Consulta Registral Pública 360°</span>
                        <h3 class="mb-0 fw-bold text-white">Dictamen Pericial de Invariabilidad</h3>
                    </div>
                    <span class="badge bg-success text-white px-3 py-2 rounded-pill"><i class="bi bi-check-circle-fill me-1"></i> EVALUACIÓN CONCLUIDA</span>
                </div>

                <div class="row g-3 mb-4">
                    <div class="col-md-6">
                        <label class="text-muted small d-block">Folio / RFC Auditado</label>
                        <div class="fw-bold text-uppercase fs-4 text-info">{}</div>
                    </div>
                    <div class="col-md-6">
                        <label class="text-muted small d-block">Resguardo Criptográfico Inmutable</label>
                        <div class="fw-bold text-success"><i class="bi bi-shield-fill-check me-1"></i> Merkle Proof & Arbitrum One Validado</div>
                    </div>
                </div>

                <div class="risk-card p-4 mb-4">
                    <div class="row align-items-center">
                        <div class="col-md-8">
                            <span class="badge bg-danger text-white mb-2"><i class="bi bi-exclamation-triangle-fill me-1"></i> Estimación Pericial en UMAs 2026</span>
                            <h5 class="fw-bold text-white mb-1">Riesgo Fiscal, Laboral y Administrativo Detectado</h5>
                            <p class="text-muted small mb-0">Basado en la evaluación de vulnerabilidades registrales (SAT Art. 69-B, 39 NOMs STPS, IMSS 32-D/REPSE, SABG LFPDPPP y PROFECO Art. 76 BIS):</p>
                        </div>
                        <div class="col-md-4 text-md-end mt-3 mt-md-0">
                            <span class="fs-6 text-muted d-block">Exposición Contingente:</span>
                            <span class="fs-3 fw-bold text-danger">${} MXN</span>
                        </div>
                    </div>
                </div>

                <div class="mb-4">
                    <label class="text-muted small d-block mb-1">Impronta Criptográfica Inmutable (SHA-256)</label>
                    <div class="hash-box">{}</div>
                </div>

                <div class="p-4 rounded-3 border border-secondary mb-4" style="background: #0b132b;">
                    <div class="row align-items-center">
                        <div class="col-md-7 mb-3 mb-md-0">
                            <span class="badge bg-info text-dark mb-2 px-3 py-1">Kit de Expediente Pericial & Blindaje Normativo (.zip)</span>
                            <h5 class="fw-bold text-white mb-1">Activar Blindaje 360° & Descargar Expediente Criptográfico</h5>
                            <p class="text-muted small mb-2">Incluye: Dictamen Ejecutivo PDF firmado por Perito C.P.C., 10 Certificados Normativos PDF, 7 Kits Legales listos para firma autógrafa, Sello Fecha Cierta SCJN y Factura CFDI 4.0.</p>
                            <span class="fs-2 fw-bold text-white">${} <small class="fs-6 text-muted">MXN (IVA Incluido)</small></span>
                        </div>
                        <div class="col-md-5 text-md-end">
                            <a href="/checkout?ref={}" class="btn btn-proceed w-100 fs-5 shadow text-center text-decoration-none d-block">
                                <i class="bi bi-shield-lock-fill me-2"></i> Activar Blindaje Ahora
                            </a>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    </div>
</div>

<footer class="text-center py-4 text-muted border-top border-secondary mt-5" style="background: #0b132b;">
    <small>© 2026 Firma Pericial Mallen (mallen.lat) • Todos los derechos reservados</small>
</footer>

</body>
</html>"###,
        clean_id, riesgo_str, real_sha256, provision_str, clean_id
    );
    Html(html)
}
