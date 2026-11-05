#set page(
  paper: "a4",
  margin: (x: 1.2cm, top: 1.1cm, bottom: 1.1cm),
  header: align(right)[#text(size: 6.6pt, fill: rgb("#475569"), weight: "bold")[FIRMA W16 MALLEN® (MALLEN.LAT) • CERTIFICADO DE ACREDITACIÓN TÉCNICA Y NORMATIVA 360°]],
  footer: [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(2pt)
    #grid(
      columns: (1.2fr, 1fr),
      align(left)[#text(size: 6.5pt, fill: rgb("#475569"))[Folio Pericial: #text(weight: "bold", fill: rgb("#0f172a"))[{{NORMA_FOLIO_PREFIX}}-{{REF_TAX}}] • Giro: #text(weight: "bold")[{{EMPRESA_SCIAN_CODE}}]]],
      align(right)[#text(size: 6.5pt, fill: rgb("#475569"))[Sello SHA-256 / Merkle L2: #text(weight: "bold", fill: rgb("#0f172a"))[mallen.lat/verify/{{REF_TAX}}]]]
    )
  ]
)
#set text(size: 7.4pt, fill: rgb("#1e293b"))

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.8pt + rgb("{{NORMA_COLOR}}"), inset: (x: 9pt, y: 7pt), radius: 3pt)[
  #grid(
    columns: (1fr, auto),
    align: horizon,
    [
      #text(size: 9.8pt, weight: "bold", fill: rgb("#0f172a"))[CERTIFICADO DE INSTRUMENTACIÓN Y BLINDAJE NORMATIVO]\
      #v(1.5pt)
      #text(size: 7.4pt, fill: rgb("{{NORMA_COLOR}}"), weight: "bold")[{{NORMA_FOLIO_PREFIX}} — {{NORMA_TITULO}}]
    ],
    [
      #box(fill: rgb("#0f172a"), inset: (x: 7pt, y: 3.5pt), radius: 2pt)[
        #text(size: 6.8pt, fill: white, weight: "bold")[EXPEDIENTE TÉCNICO INTEGRADO]
      ]
    ]
  )
]

#v(0.2em)

#rect(width: 100%, fill: rgb("#ffffff"), stroke: 0.5pt + rgb("#cbd5e1"), inset: 6pt, radius: 2.5pt)[
  #grid(
    columns: (1.1fr, 0.9fr),
    row-gutter: 3.8pt,
    column-gutter: 8pt,
    [*Ente Auditado:* {{EMPRESA_NOMBRE}}],
    [*RFC / Clave Registral:* {{EMPRESA_TAX_ID}}],
    [*Giro SCIAN Oficial:* {{EMPRESA_ACTIVIDAD}}],
    [*Plantilla y Prima IMSS:* {{EMPRESA_EMPLEADOS}} colaboradores • {{EMPRESA_IMSS_CLASE}}],
    [*Domicilio del Centro de Trabajo:* {{EMPRESA_DIRECCION}}],
    [*Jurisdicción Aplicable:* {{EMPRESA_JURISDICCION}} / Fuero Federal],
    [*Representación Legal:* {{REPRESENTANTE_LEGAL}}],
    [*Contingencia Total Mitigada:* \$ {{CONTINGENCIA_MITIGADA}} MXN (Score Base: {{RISK_SCORE}}/100)],
    [*Órgano Certificador:* {{AUDITOR_NOMBRE}} ({{AUDITOR_CEDULA}})],
    [*Sello Criptográfico SHA-256:* #raw("{{AUDIT_HASH}}")]
  )
]

#v(0.2em)

#text(size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[1. FUNDAMENTACIÓN TÉCNICA, PARAMETRIZACIÓN POR GIRO SCIAN Y ALCANCE PERICIAL]
#v(0.1em)
#text(size: 6.8pt)[El Consejo Técnico de *FIRMA W16 MALLEN®* certifica que para la entidad *{{EMPRESA_NOMBRE}}* (*{{EMPRESA_TAX_ID}}*), con actividad económica registrada en el catálogo SCIAN *{{EMPRESA_ACTIVIDAD}}* y una plantilla operativa parametrizada de *{{EMPRESA_EMPLEADOS}} personas*, se han generado, personalizado y sellado criptográficamente los instrumentos documentales, actas constitutivas y matrices de control exigibles para acreditar el cumplimiento preventivo del vector *{{NORMA_FOLIO_PREFIX}}*.]

#v(0.15em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("{{NORMA_COLOR}}"), inset: 5pt, radius: 2.5pt)[
  #text(size: 6.6pt)[*Preceptos Legales y Alcance Cubierto:* {{NORMA_DESCRIPCION}}\
  *Parametrización Específica del Giro ({{EMPRESA_SCIAN_CODE}}):* {{SECTOR_ESPECIFICO}}]
]

#v(0.2em)

#text(size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[2. MATRIZ DE DEFENDABILIDAD JURÍDICA Y TRAZABILIDAD CRIPTOGRÁFICA]
#v(0.1em)

#table(
  columns: (1.8fr, 1.1fr, 1.1fr),
  inset: (x: 5pt, y: 3.5pt),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  stroke: 0.4pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { left } else { left },
  [#text(fill: white, weight: "bold", size: 6.3pt)[Parámetro de Auditoría Evaluado]],
  [#text(fill: white, weight: "bold", size: 6.3pt)[Estatus de Instrumentación]],
  [#text(fill: white, weight: "bold", size: 6.3pt)[Cobertura Probatoria]],
  [Alineación al Giro SCIAN ({{EMPRESA_SCIAN_CODE}}) y Capacidad ({{EMPRESA_EMPLEADOS}} pers.)], [PARAMETRIZADO POR PADRÓN OFICIAL], [Art. 992 LFT y Art. 75 CFF],
  [Clasificación de Riesgo Patronal ({{EMPRESA_IMSS_CLASE}})], [CONCILIADO], [Arts. 71-75 LSS y Art. 196 RACERF],
  [Inalterabilidad y Fecha Cierta Digital (Hash `{{AUDIT_HASH}}`)], [SELLADO SHA-256], [Arts. 89-93 CCom y Jurisprudencia SCJN]
)

#v(0.2em)

#block(fill: rgb("#0f172a"), inset: (x: 7pt, y: 4pt), radius: 2pt, width: 100%)[
  #text(size: 7.8pt, weight: "bold", fill: rgb("#ffffff"))[3. CÉDULA DE PAPELES DE TRABAJO E INSTRUMENTOS ESPECÍFICOS DEL EXPEDIENTE ({{NORMA_FOLIO_PREFIX}})]
]

#v(0.1em)

#table(
  columns: (0.9fr, 2.3fr, 0.8fr),
  inset: (x: 5pt, y: 3.5pt),
  fill: (x, y) => if y == 0 { rgb("#334155") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  stroke: 0.4pt + rgb("#cbd5e1"),
  [#text(fill: white, weight: "bold", size: 6.2pt)[Código Papel]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Instrumento Jurídico / Evidencia Técnica Integrada en el Kit]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Estado]],
  [PT-{{NORMA_FOLIO_PREFIX}}-01], [{{NORMA_PT_1}}], [INSTRUMENTADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-02], [{{NORMA_PT_2}}], [INSTRUMENTADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-03], [{{NORMA_PT_3}}], [VERIFICADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-04], [Dictamen Pericial 37 Vectores y Sello Criptográfico (`{{AUDIT_HASH}}`)], [SELLADO SHA-256]
)

#v(0.45em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.4pt + rgb("#cbd5e1"), inset: 6pt, radius: 2.5pt)[
  #grid(
    columns: (1fr, 1fr, 1fr),
    column-gutter: 8pt,
    align: center + bottom,
    [
      #v(14pt)
      #line(length: 85%, stroke: 0.6pt + rgb("#334155"))
      #v(2pt)
      #text(weight: "bold", size: 6.5pt)[DIRECCIÓN PERICIAL CORPORATIVA]\
      #text(size: 6pt, fill: rgb("#475569"))[{{AUDITOR_NOMBRE}}]\
      #text(size: 5.6pt, fill: rgb("#64748b"))[Folio: {{AUDITOR_CEDULA}} • {{AUDITOR_DC5}}]
    ],
    [
      #v(14pt)
      #line(length: 85%, stroke: 0.6pt + rgb("#334155"))
      #v(2pt)
      #text(weight: "bold", size: 6.5pt)[REPRESENTACIÓN LEGAL]\
      #text(size: 6pt, fill: rgb("#475569"))[{{REPRESENTANTE_LEGAL}}]\
      #text(size: 5.6pt, fill: rgb("#64748b"))[{{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}})]
    ],
    [
      #rect(width: 100%, fill: rgb("#ffffff"), stroke: 0.4pt + rgb("#94a3b8"), inset: 4pt, radius: 2pt)[
        #align(left)[
          #text(size: 5.8pt, weight: "bold", fill: rgb("#0f172a"))[VERIFICACIÓN PÚBLICA INMUTABLE]\
          #text(size: 5.4pt, fill: rgb("#475569"))[Emisión UTC: {{FECHA_EMISION}}]\
          #text(size: 5.4pt, fill: rgb("#475569"))[Hash: #raw("{{AUDIT_HASH}}")]\
          #text(size: 5.4pt, weight: "bold", fill: rgb("#2563eb"))[https://mallen.lat/verify/{{REF_TAX}}]
        ]
      ]
    ]
  )
]
