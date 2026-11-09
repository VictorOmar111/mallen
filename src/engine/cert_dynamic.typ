#set text(hyphenate: false)
#set par(justify: false)
#show heading: set block(sticky: true)
#set page(
  paper: "a4",
  margin: (x: 1.2cm, top: 1.1cm, bottom: 1.1cm),
  background: place(
    center + horizon,
    rotate(
      -28deg,
      block(width: 18cm)[
        #align(center)[
          #text(size: 21pt, weight: "bold", fill: rgb(15, 23, 42, 4%))[
            FIRMA W16 MALLEN® • SELLO CRIPTOGRÁFICO INMUTABLE\
            #text(size: 11pt, font: ("Liberation Mono", "DejaVu Sans Mono"))[SHA-256: {{AUDIT_HASH}} • eIDAS ART. 41 / CCOM ART. 89]
          ]
        ]
      ]
    )
  ),
  header: align(right)[#text(size: 6.6pt, fill: rgb("#475569"), weight: "bold")[FIRMA W16 MALLEN® (MALLEN.LAT) • CERTIFICADO DE ACREDITACIÓN TÉCNICA Y NORMATIVA 360°]],
  footer: [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(2pt)
    #grid(
      columns: (1.2fr, 1fr),
      align(left)[#text(size: 6.5pt, fill: rgb("#475569"))[Folio Criptográfico: #text(font: ("Liberation Mono", "DejaVu Sans Mono"), weight: "bold", fill: rgb("#0f172a"))[{{NORMA_FOLIO_PREFIX}}-{{REF_TAX}}] • Giro SCIAN: #text(weight: "bold")[{{EMPRESA_SCIAN_CODE}}]]],
      align(right)[#text(size: 6.5pt, fill: rgb("#475569"))[Verificación: #text(font: ("Liberation Mono", "DejaVu Sans Mono"), weight: "bold", fill: rgb("#0f172a"))[mallen.lat/verify/{{REF_TAX}}] • #text(weight: "bold")[mallen.lat/#faq]]]
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
    [#box(width: 100%, clip: false)[#set par(leading: 0.42em); *Ente Auditado:* #text(size: 6.9pt, weight: "bold")[{{EMPRESA_NOMBRE}}]]],
    [#box(width: 100%, clip: false)[*RFC / Clave Registral:* #text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6.9pt, weight: "bold")[{{EMPRESA_TAX_ID}}]]],
    [#box(width: 100%, clip: false)[#set par(leading: 0.42em); *Giro SCIAN Oficial:* #text(size: 6.8pt)[{{EMPRESA_ACTIVIDAD}}]]],
    [#box(width: 100%, clip: false)[*Plantilla y Prima IMSS:* #text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6.8pt, weight: "bold")[{{EMPRESA_EMPLEADOS}}] colaboradores • {{EMPRESA_IMSS_CLASE}}]],
    [#box(width: 100%, clip: false)[#set par(leading: 0.42em); *Domicilio del Centro de Trabajo:* #text(size: 6.7pt)[{{EMPRESA_DIRECCION}}]]],
    [#box(width: 100%, clip: false)[*Jurisdicción Aplicable:* {{EMPRESA_JURISDICCION}} / Fuero Federal]],
    [#box(width: 100%, clip: false)[#set par(leading: 0.42em); *Representación Legal:* #text(size: 6.8pt)[{{REPRESENTANTE_LEGAL}}]]],
    [#box(width: 100%, clip: false)[*Contingencia Total Mitigada:* #text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 7pt, weight: "bold", fill: rgb("#047857"))[\$ {{CONTINGENCIA_MITIGADA}} MXN] #text(size: 6.4pt)[(Score: {{RISK_SCORE}}/100)]]],
    [#box(width: 100%, clip: false)[#set par(leading: 0.42em); *Infraestructura Emisora:* #text(size: 6.6pt)[{{AUDITOR_NOMBRE}}]]],
    [#box(width: 100%, clip: false)[*Sello Criptográfico SHA-256:* #text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6.6pt, weight: "bold", fill: rgb("#0369a1"))[#raw("{{AUDIT_HASH}}")]]]
  )
]

#v(0.2em)

#text(size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[1. FUNDAMENTACIÓN TÉCNICA, PARAMETRIZACIÓN POR GIRO SCIAN Y ALCANCE DOCUMENTAL]
#v(0.1em)
#text(size: 6.8pt)[La División de Ingeniería Regulatoria de *FIRMA W16 MALLEN®* hace constar que para la entidad *{{EMPRESA_NOMBRE}}* (*{{EMPRESA_TAX_ID}}*), con actividad económica registrada en el catálogo SCIAN *{{EMPRESA_ACTIVIDAD}}* y una plantilla operativa parametrizada de *{{EMPRESA_EMPLEADOS}} personas*, se han compilado, estructurado y sellado criptográficamente los instrumentos documentales, actas constitutivas y matrices de control preventivo correspondientes al vector *{{NORMA_FOLIO_PREFIX}}*.]

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
  [Alineación al Giro SCIAN ({{EMPRESA_SCIAN_CODE}}) y Capacidad ({{EMPRESA_EMPLEADOS}} pers.)], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6.1pt, weight: "bold")[PARAMETRIZADO OFICIAL]], [Art. 992 LFT y Art. 75 CFF],
  [Clasificación de Riesgo Patronal ({{EMPRESA_IMSS_CLASE}})], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6.1pt, weight: "bold")[CONCILIADO RACERF]], [Arts. 71-75 LSS y Art. 196 RACERF],
  [Inalterabilidad y Fecha Cierta Digital (Hash `{{AUDIT_HASH}}`)], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6.1pt, weight: "bold", fill: rgb("#047857"))[SELLADO SHA-256]], [Arts. 89-93 CCom • eIDAS Art. 41]
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
  [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[PT-{{NORMA_FOLIO_PREFIX}}-01]], [{{NORMA_PT_1}}], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[INSTRUMENTADO]],
  [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[PT-{{NORMA_FOLIO_PREFIX}}-02]], [{{NORMA_PT_2}}], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[INSTRUMENTADO]],
  [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[PT-{{NORMA_FOLIO_PREFIX}}-03]], [{{NORMA_PT_3}}], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[VERIFICADO]],
  [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold")[PT-{{NORMA_FOLIO_PREFIX}}-04]], [Dictamen Determinístico 37 Vectores y Sello Criptográfico (`{{AUDIT_HASH}}`)], [#text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 6pt, weight: "bold", fill: rgb("#047857"))[SELLADO SHA-256]]
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
      #text(weight: "bold", size: 6.3pt)[INFRAESTRUCTURA DE AUDITORÍA]\
      #box(width: 100%, clip: false)[#text(size: 5.8pt, fill: rgb("#475569"))[{{AUDITOR_NOMBRE}}]]\
      #box(width: 100%, clip: false)[#text(size: 5.4pt, fill: rgb("#64748b"))[{{AUDITOR_CEDULA}} • {{AUDITOR_DC5}}]]
    ],
    [
      #v(14pt)
      #line(length: 85%, stroke: 0.6pt + rgb("#334155"))
      #v(2pt)
      #text(weight: "bold", size: 6.3pt)[REPRESENTACIÓN LEGAL]\
      #box(width: 100%, clip: false)[#text(size: 5.8pt, fill: rgb("#475569"))[{{REPRESENTANTE_LEGAL}}]]\
      #box(width: 100%, clip: false)[#text(size: 5.4pt, fill: rgb("#64748b"))[{{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}})]]
    ],
    [
      #rect(width: 100%, fill: rgb("#ffffff"), stroke: 0.4pt + rgb("#94a3b8"), inset: 4pt, radius: 2pt)[
        #grid(
          columns: (auto, 1fr),
          column-gutter: 5pt,
          align: horizon,
          [{{VECTOR_QR_SEAL}}],
          [
            #align(left)[
              #text(size: 5.6pt, weight: "bold", fill: rgb("#0f172a"))[VERIFICACIÓN QR INMUTABLE]\
              #text(size: 5.2pt, fill: rgb("#475569"))[UTC: {{FECHA_EMISION}}]\
              #text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 5.0pt, fill: rgb("#475569"))[#raw("{{AUDIT_HASH}}")]\
              #text(font: ("Liberation Mono", "DejaVu Sans Mono"), size: 5.1pt, weight: "bold", fill: rgb("#2563eb"))[mallen.lat/verify/{{REF_TAX}}]
            ]
          ]
        )
      ]
    ]
  )
]
