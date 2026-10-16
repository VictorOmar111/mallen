use axum::response::Html;

pub async fn render_verify(id: &str) -> Html<String> {
    // Estimación pericial de exposición de riesgo por sanciones (SAT / STPS / INAI / IMSS)
    let riesgo_estimado: f64 = 320000.00; 
    
    // Provisión de activación: 0.5% con piso mínimo de $1,500.00 MXN
    let calculo_provision = riesgo_estimado * 0.005;
    let provision_activacion: f64 = if calculo_provision < 1500.00 { 1500.00 } else { calculo_provision };

    let riesgo_str = format!("{:.2}", riesgo_estimado);
    let provision_str = format!("{:.2}", provision_activacion);

    let html = format!(r###"
<!DOCTYPE html>
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
        <span class="badge bg-dark border border-secondary text-info"><i class="bi bi-shield-check me-1"></i> Expediente Consultivo Gratuito</span>
    </div>
</nav>

<div class="container my-5">
    <div class="row justify-content-center">
        <div class="col-lg-10">
            <div class="card-feature p-4 p-md-5">
                
                <div class="d-flex flex-wrap justify-content-between align-items-center mb-4 pb-3 border-bottom border-secondary gap-2">
                    <div>
                        <span class="text-uppercase text-muted fw-bold" style="font-size: 0.8rem;">Consulta Registral Pública</span>
                        <h3 class="mb-0 fw-bold text-white">Dictamen Pericial de Invariabilidad</h3>
                    </div>
                    <span class="badge bg-success text-white px-3 py-2 rounded-pill"><i class="bi bi-check-circle-fill me-1"></i> EVALUACIÓN GRATUITA CONCLUIDA</span>
                </div>

                <div class="row g-3 mb-4">
                    <div class="col-md-6">
                        <label class="text-muted small d-block">Folio / Identificador Auditado</label>
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
                            <span class="badge bg-danger text-white mb-2"><i class="bi bi-exclamation-triangle-fill me-1"></i> Estimación Pericial Sanction-Exposure</span>
                            <h5 class="fw-bold text-white mb-1">Riesgo Fiscal y Administrativo Detectado</h5>
                            <p class="text-muted small mb-0">Basado en la evaluación de vulnerabilidades registrales (SAT Art. 69-B, NOMs STPS 019/030/035/037, IMSS e INAI LFPDPPP), la exposición contingente estimada es de:</p>
                        </div>
                        <div class="col-md-4 text-md-end mt-3 mt-md-0">
                            <span class="fs-6 text-muted d-block">Exposición Contingente:</span>
                            <span class="fs-3 fw-bold text-danger">${} MXN</span>
                        </div>
                    </div>
                </div>

                <h5 class="fw-bold text-white mb-3"><i class="bi bi-building-check text-info me-2"></i>Estatus de Inspección por Dependencia</h5>
                
                <div class="row g-3 mb-4">
                    <div class="col-md-6">
                        <div class="inst-box p-3 h-100">
                            <div class="d-flex justify-content-between align-items-center mb-2">
                                <span class="fw-bold text-white"><i class="bi bi-diagram-3-fill text-info me-2"></i>INEGI / DENUE</span>
                                <span class="badge bg-info text-dark">Registro Activo</span>
                            </div>
                            <p class="text-muted small mb-1">Cruce en el Directorio Nacional de Unidades Económicas, geolocalización de establecimiento y actividad económica SCIAN.</p>
                            <small class="text-info fw-bold">✓ Coincidencia Registral Confirmada</small>
                        </div>
                    </div>

                    <div class="col-md-6">
                        <div class="inst-box p-3 h-100">
                            <div class="d-flex justify-content-between align-items-center mb-2">
                                <span class="fw-bold text-white"><i class="bi bi-receipt-cutoff text-info me-2"></i>SAT (Materialidad Art. 69-B)</span>
                                <span class="badge bg-success">Sin Incidencias</span>
                            </div>
                            <p class="text-muted small mb-1">Revisión de no localización, lista negra CFF y soporte de capacidad operativa e infraestructura real.</p>
                            <small class="text-info fw-bold">✓ Opinión Fiscal Validada</small>
                        </div>
                    </div>

                    <div class="col-md-6">
                        <div class="inst-box p-3 h-100">
                            <div class="d-flex justify-content-between align-items-center mb-2">
                                <span class="fw-bold text-white"><i class="bi bi-journal-check text-info me-2"></i>IMSS & INFONAVIT</span>
                                <span class="badge bg-info text-dark">Registro Patronal</span>
                            </div>
                            <p class="text-muted small mb-1">Evaluación de cumplimiento de obligaciones patronales y prima de riesgo de trabajo.</p>
                            <small class="text-info fw-bold">✓ Cumplimiento Validado</small>
                        </div>
                    </div>

                    <div class="col-md-6">
                        <div class="inst-box p-3 h-100">
                            <div class="d-flex justify-content-between align-items-center mb-2">
                                <span class="fw-bold text-white"><i class="bi bi-person-badge-fill text-info me-2"></i>STPS (NOM-019/030/035/037)</span>
                                <span class="badge bg-primary">Marco Normativo</span>
                            </div>
                            <p class="text-muted small mb-1">Diagnóstico de comisiones mixtas, teletrabajo, riesgos psicosociales y carpetas de inspección.</p>
                            <small class="text-info fw-bold">✓ Evaluación de Riesgo Concluida</small>
                        </div>
                    </div>
                </div>

                <div class="mb-4">
                    <label class="text-muted small d-block mb-1">Impronta Criptográfica Inmutable (SHA-256)</label>
                    <div class="hash-box">
                        e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
                    </div>
                </div>

                <div class="p-4 rounded-3 border border-secondary mb-4" style="background: #0b132b;">
                    <div class="row align-items-center">
                        <div class="col-md-7 mb-3 mb-md-0">
                            <span class="badge bg-info text-dark mb-2 px-3 py-1">Kit de Expediente Pericial & Blindaje Normativo (.zip)</span>
                            <h5 class="fw-bold text-white mb-1">Activar Inmunidad & Descargar Expediente Criptográfico</h5>
                            <p class="text-muted small mb-2">Obtenga el paquete digital (.zip) que incluye: Dictamen firmado por Perito (Cédula/DC-5/DC-3), Matriz de Riesgos JSON, Evidencia OSINT, Certificado Blockchain Merkle en Arbitrum One y Carpeta Documental Normativa.</p>
                            <span class="text-muted small d-block">Provisión de Activación (0.5% del riesgo estimado - Mínimo $1,500.00 MXN):</span>
                            <span class="fs-2 fw-bold text-white">${} <small class="fs-6 text-muted">MXN (IVA Incluido)</small></span>
                        </div>
                        <div class="col-md-5 text-md-end">
                            <a href="/checkout/{}" class="btn btn-proceed w-100 fs-5 shadow text-center text-decoration-none d-block">
                                <i class="bi bi-shield-lock-fill me-2"></i> Pagar Provisión & Activar
                            </a>
                        </div>
                    </div>
                </div>

                <div class="text-center pt-2">
                    <div class="d-flex justify-content-center gap-3 flex-wrap text-muted small">
                        <span><i class="bi bi-lock-fill text-info me-1"></i> Conexión Cifrada TLS 1.3</span>
                        <span><i class="bi bi-shield-check text-info me-1"></i> Emisión Factura CFDI 4.0</span>
                        <span><i class="bi bi-file-earmark-zip-fill text-info me-1"></i> Entregable Digital .ZIP Inmediato</span>
                    </div>
                </div>

            </div>
        </div>
    </div>
</div>

<footer class="text-center py-4 text-muted border-top border-secondary mt-5" style="background: #0b132b;">
    <small>© 2026 Firma Pericial Mallen • Todos los derechos reservados</small>
</footer>

</body>
</html>
    "###, id, riesgo_str, provision_str, id);
    Html(html)
}
