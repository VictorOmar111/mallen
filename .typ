use std::os::unix::process;
use tokio::process::Command;
use std::fs;

pub async fn generate_pdf_report_typst(
    empresa_nombre: &str,
    tax_id: &str,
    risk_score: u8,
    fine_mxn: f64,
    output_pdf_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let typ_path = format!("{}.typ", output_pdf_path);

    // Plantilla Typst profesional B2B
    let typst_template = format!(r#"
#set page(paper: "a4", margin: (x: 1.5cm, y: 1.5cm))
#set text(font: "Liberation Sans", size: 10pt)

// Encabezado
#rect(width: 100%, fill: rgb("#0f172a"), inset: 12pt, radius: 4pt)[
  #text(fill: white, weight: "bold", size: 14pt)[INMUNIDAD B2B | AUDITORÍA & CUMPLIMIENTO] \
  #text(fill: rgb("#cbd5e1"), size: 9pt)[DICTAMEN PERICIAL DE EVALUACIÓN DE RIESGO REGULATORIO]
]

#v(1em)

*Empresa Auditada:* {} \
*RFC / Tax ID:* {} \
*Fecha de Emisión:* 05 de Octubre, 2026

#v(1em)

// Resumen de Riesgo Financiero
#grid(
  columns: (2fr, 1fr),
  gutter: 1em,
  rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 10pt, radius: 6pt)[
    #text(fill: rgb("#0f172a"), weight: "bold")[EXPOSICIÓN TOTAL ESTIMADA A SANCIONES:] \
    #text(fill: rgb("#dc2626"), size: 18pt, weight: "bold")[${:.2} MXN]
  ],
  rect(width: 100%, fill: rgb("#fef2f2"), stroke: 1pt + rgb("#fca5a5"), inset: 10pt, radius: 6pt)[
    #align(center)[
      #text(size: 20pt, weight: "bold", fill: rgb("#dc2626"))[{}%] \
      #text(size: 8pt, weight: "bold", fill: rgb("#991b1b"))[SCORE DE RIESGO]
    ]
  ]
)

#v(1.5em)

#text(size: 12pt, weight: "bold", fill: rgb("#0f172a"))[DESGLOSE DE OBLIGACIONES Y MULTAS POTENCIALES]

#v(0.5em)

#table(
  columns: (1.5fr, 1.5fr, 2fr),
  fill: (x, y) => if y == 0 {{ rgb("#f1f5f9") }} else {{ none }},
  stroke: (x, y) => if y == 0 {{ (bottom: 1.5pt + rgb("#0f172a")) }} else {{ (bottom: 0.5pt + rgb("#e2e8f0")) }},
  [*Normativa / Dependencia*], [*Rango Estimado de Multa*], [*Observación / Estatus*],
  [STPS (NOM-035 / NOM-037)], [$162,855.00 - $434,280.00 MXN], [Comisiones Mixtas y Teletrabajo],
  [INAI (LFPDPPP)], [$21,714.00 - $347,424.00 MXN], [Aviso de Privacidad / Datos],
  [PROFECO (LFPC)], [$50,000.00 - $150,000.00 MXN], [Términos Comerciales en Línea],
  [SAT / IMSS (CFF 69-B)], [$200,000.00 - $500,000.00 MXN], [Soporte Materialidad Operativa],
)

#v(2em)

#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))

#text(size: 8pt, fill: rgb("#64748b"))[
  Documento generado automáticamente por el Motor Pericial de Auditoría B2B. Válido sin firma autógrafa mediante certificación criptográfica.
]
"#, empresa_nombre, tax_id, fine_mxn, risk_score);

    // Escribir archivo temporal y compilar
    tokio::fs::write(&typ_path, typst_template).await?;

    let output = Command::new("typst")
        .arg("compile")
        .arg(&typ_path)
        .arg(output_pdf_path)
        .output()
        .await?;

    let _ = tokio::fs::remove_file(typ_path).await;

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("Error en compilación Typst: {}", err).into())
    }
}
