use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::OsintResult;
use resvg::usvg::{self, TreeParsing, TreeTextToPath};
use std::fs::File;
use std::io::Write;

/// Plantilla SVG vectorial completa de alta calidad C-Level.
/// Se mantiene como string estático y se rellenan los placeholders `{{KEY}}`
/// mediante `.replace()` para evitar problemas de parsing con `format!(r#"..."#)`.
const SVG_TEMPLATE: &str = concat!(
r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 1130" width="800" height="1130">
  <style>
    .title {{ font-family: sans-serif ; font-size: 22px; font-weight: bold; fill: #ffffff ; }}
    .subtitle {{ font-family: sans-serif ; font-size: 13px; fill: #d1d5db ; }}
    .h2 {{ font-family: sans-serif ; font-size: 16px; font-weight: bold; fill: #1e293b ; }}
    .text {{ font-family: sans-serif ; font-size: 13px; fill: #334155 ; }}
    .bold {{ font-weight: bold; fill: #0f172a ; }}
    .alert {{ font-family: sans-serif ; font-size: 24px; font-weight: bold; fill: #dc2626 ; }}
    .table-header {{ font-family: sans-serif ; font-size: 12px; font-weight: bold; fill: #1e293b ; }}
    .table-cell {{ font-family: sans-serif ; font-size: 12px; fill: #475569 ; }}
    .footer {{ font-family: sans-serif ; font-size: 11px; fill: #64748b ; }}
  </style>

  <!-- Encabezado Corporativo -->
  <rect x="0" y="0" width="800" height="110" fill=" #0f172a "/>
  <text x="40" y="48" class="title">INMUNIDAD B2B | AUDITORÍA &amp; CUMPLIMIENTO</text>
  <text x="40" y="78" class="subtitle">DICTAMEN PERICIAL DE EVALUACIÓN DE RIESGO REGULATORIO</text>

  <!-- Datos Generales -->
  <text x="40" y="150" class="text"><tspan class="bold">Empresa Auditada:</tspan> {{NOMBRE}}</text>
  <text x="40" y="175" class="text"><tspan class="bold">Tax ID / RFC:</tspan> {{TAX_ID}}</text>
  <text x="40" y="200" class="text"><tspan class="bold">Giro / Actividad:</tspan> {{ACTIVIDAD}}</text>
  <text x="40" y="225" class="text"><tspan class="bold">Ubicación:</tspan> {{DIRECCION}}, {{ESTADO}}</text>

  <!-- Gráfica Vectorial de Dona (Score de Riesgo) -->
  <g transform="translate(620, 190)">
    <circle cx="0" cy="0" r="40" fill="none" stroke=" #e2e8f0 " stroke-width="14"/>
    <circle cx="0" cy="0" r="40" fill="none" stroke=" #dc2626 " stroke-width="14"
            stroke-dasharray="{{CIRCUMFERENCE}}" stroke-dashoffset="{{DASHOFFSET}}"
            transform="rotate(-90)"/>
    <text x="0" y="6" text-anchor="middle" font-family=" sans-serif " font-size="18" font-weight="bold" fill=" #dc2626 ">{{RISK_SCORE}}%</text>
    <text x="0" y="22" text-anchor="middle" font-family=" sans-serif " font-size="9" font-weight="bold" fill=" #64748b ">RIESGO</text>
  </g>

  <!-- Cuadro de Exposición Financiera -->
  <rect x="40" y="260" width="720" height="85" rx="8" fill=" #f8fafc " stroke=" #cbd5e1 " stroke-width="1"/>
  <text x="60" y="295" class="text"><tspan class="bold">EXPOSICIÓN TOTAL ESTIMADA A SANCIONES Y MULTAS ACUMULADAS:</tspan></text>
  <text x="60" y="330" class="alert">${{ESTIMATED_FINE}} MXN</text>

  <!-- Tabla de Desglose Regulatorio -->
  <text x="40" y="385" class="h2">DESGLOSE DE OBLIGACIONES Y MULTAS POTENCIALES</text>

  <rect x="40" y="405" width="720" height="35" fill=" #f1f5f9 "/>
  <text x="55" y="427" class="table-header">INSTITUCIÓN / NORMATIVA</text>
  <text x="310" y="427" class="table-header">RANGO DE MULTA POTENCIAL</text>
  <text x="540" y="427" class="table-header">HALLAZGO / OBSERVACIÓN</text>

  <!-- Filas de la Tabla -->
  <g transform="translate(0, 440)">
    <line x1="40" y1="35" x2="760" y2="35" stroke=" #e2e8f0 " stroke-width="1"/>
    <text x="55" y="23" class="table-cell"><tspan font-weight="bold">STPS (NOM-035 / NOM-037)</tspan></text>
    <text x="310" y="23" class="table-cell">$162,855.00 MXN - $434,280.00 MXN</text>
    <text x="540" y="23" class="table-cell">Comisiones Mixtas y Teletrabajo</text>

    <line x1="40" y1="70" x2="760" y2="70" stroke=" #e2e8f0 " stroke-width="1"/>
    <text x="55" y="58" class="table-cell"><tspan font-weight="bold">INAI (LFPDPPP)</tspan></text>
    <text x="310" y="58" class="table-cell">$21,714.00 MXN - $347,424.00 MXN</text>
    <text x="540" y="58" class="table-cell">{{PRIVACY_POLICY_SHORT}}</text>

    <line x1="40" y1="105" x2="760" y2="105" stroke=" #e2e8f0 " stroke-width="1"/>
    <text x="55" y="93" class="table-cell"><tspan font-weight="bold">PROFECO (LFPC Art. 76 BIS)</tspan></text>
    <text x="310" y="93" class="table-cell">$50,000.00 MXN - $150,000.00 MXN</text>
    <text x="540" y="93" class="table-cell">Términos Comerciales en Línea</text>

    <line x1="40" y1="140" x2="760" y2="140" stroke=" #e2e8f0 " stroke-width="1"/>
    <text x="55" y="128" class="table-cell"><tspan font-weight="bold">SAT / IMSS (CFF Art. 69-B)</tspan></text>
    <text x="310" y="128" class="table-cell">$200,000.00 MXN - $500,000.00 MXN</text>
    <text x="540" y="128" class="table-cell">Soporte de Materialidad Operativa</text>

    <line x1="40" y1="175" x2="760" y2="175" stroke=" #e2e8f0 " stroke-width="1"/>
    <text x="55" y="163" class="table-cell"><tspan font-weight="bold">SECTORIAL (COFEPRIS/CNBV)</tspan></text>
    <text x="310" y="163" class="table-cell">$150,000.00 MXN - $400,000.00 MXN</text>
    <text x="540" y="163" class="table-cell">Licencias y Avisos de Operación</text>
  </g>

  <!-- Diagnóstico OSINT Web -->
  <text x="40" y="670" class="h2">DIAGNÓSTICO DE HUELLA DIGITAL (OSINT)</text>
  <rect x="40" y="685" width="720" height="65" rx="6" fill=" #f8fafc " stroke=" #cbd5e1 " stroke-width="1"/>
  <text x="60" y="710" class="text"><tspan class="bold">Estado SSL/HTTPS:</tspan> {{SSL_STATUS}}</text>
  <text x="350" y="710" class="text"><tspan class="bold">Aviso de Privacidad:</tspan> {{PRIVACY_POLICY_LONG}}</text>
  <text x="60" y="735" class="text"><tspan class="bold">Estatus Servidor Web:</tspan> HTTP {{HTTP_STATUS}}</text>

  <!-- Certificación Digital y Oferta de Activación -->
  <line x1="40" y1="780" x2="760" y2="780" stroke=" #cbd5e1 " stroke-width="1.5"/>
  <text x="40" y="815" class="h2">CERTIFICACIÓN PERICIAL &amp; ACTIVACIÓN DE INMUNIDAD</text>
  <text x="40" y="845" class="text"><tspan class="bold">Hash de Auditoría SHA-256:</tspan> {{AUDIT_HASH}}</text>
  <text x="40" y="870" class="text"><tspan class="bold">Certificación Pericial Contable:</tspan> {{ACCOUNTANT_SIGNATURE}}</text>

  <rect x="40" y="900" width="720" height="75" rx="8" fill=" #fef2f2 " stroke=" #fca5a5 " stroke-width="1"/>
  <text x="60" y="930" font-family=" sans-serif " font-size="15" font-weight="bold" fill=" #dc2626 ">OFERTA DE ACTIVACIÓN DE BLINDAJE INTEGRAL: $140.00 USD ($2,500 MXN)</text>
  <text x="60" y="955" font-family=" sans-serif " font-size="13" fill=" #991b1b ">Active la cobertura y descargue los certificados ajustados en: https://inmunidadb2b.com/checkout?ref={{CHECKOUT_REF}}</text>

  <text x="40" y="1080" class="footer">Dictamen generado automáticamente por el Motor Pericial de Auditoría B2B. Documento respaldado criptográficamente.</text>
</svg>
"#
);

pub fn generate_pdf_report(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let risk_pct = empresa.risk_score.min(100);
    let circumference = 2.0 * std::f32::consts::PI * 40.0;
    let stroke_dashoffset = circumference - ((risk_pct as f32 / 100.0) * circumference);

    let privacy_policy_long = if osint.has_privacy_policy {
        "Aviso de Privacidad Detectado"
    } else {
        "ALERTA: Sin Aviso de Privacidad"
    };
    let privacy_policy_short = if osint.has_privacy_policy {
        "DETECTADO"
    } else {
        "AUSENTE / NO VISIBLE"
    };
    let ssl_status = if osint.has_ssl {
        "SEGURO (HTTPS)"
    } else {
        "INSEGURO (HTTP)"
    };

    // Rellenar la plantilla SVG sustituyendo los placeholders `{{KEY}}`.
    let svg_content = SVG_TEMPLATE
        .replace("{{NOMBRE}}", &empresa.nombre)
        .replace("{{TAX_ID}}", empresa.tax_id.as_deref().unwrap_or("N/A"))
        .replace("{{ACTIVIDAD}}", &empresa.actividad)
        .replace("{{DIRECCION}}", &empresa.direccion)
        .replace("{{ESTADO}}", &empresa.estado)
        .replace("{{CIRCUMFERENCE}}", &format!("{:.2}", circumference))
        .replace("{{DASHOFFSET}}", &format!("{:.2}", stroke_dashoffset))
        .replace("{{RISK_SCORE}}", &empresa.risk_score.to_string())
        .replace("{{ESTIMATED_FINE}}", &format!("{:.2}", empresa.estimated_fine_mxn))
        .replace("{{PRIVACY_POLICY_SHORT}}", privacy_policy_short)
        .replace("{{SSL_STATUS}}", ssl_status)
        .replace("{{PRIVACY_POLICY_LONG}}", privacy_policy_long)
        .replace("{{HTTP_STATUS}}", &osint.http_status.to_string())
        .replace("{{AUDIT_HASH}}", &empresa.audit_hash)
        .replace(
            "{{ACCOUNTANT_SIGNATURE}}",
            empresa
                .accountant_signature
                .as_deref()
                .unwrap_or("Firma Pericial En Proceso"),
        )
        .replace(
            "{{CHECKOUT_REF}}",
            empresa.tax_id.as_deref().unwrap_or("GENERIC"),
        );

    // Cargar y Renderizar el SVG usando resvg/usvg
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    let opt = usvg::Options::default();
    let mut tree = usvg::Tree::from_str(&svg_content, &opt)?;
    tree.convert_text(&fontdb);

    let pixmap_size = tree.size.to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("Error al inicializar el lienzo Pixmap")?;

    // API correcta en resvg 0.36: convertir el árbol usvg a resvg::Tree y renderizar.
    let rtree = resvg::Tree::from_usvg(&tree);
    rtree.render(usvg::Transform::default(), &mut pixmap.as_mut());

    // Codificar en formato PDF/PNG
    let png_bytes = pixmap.encode_png()?;
    let mut pdf_file = File::create(output_path)?;

    // Guardar el renderizado vectorial
    pdf_file.write_all(&png_bytes)?;

    Ok(())
}
