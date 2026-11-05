#set page(
  paper: "a4",
  margin: (x: 1.2cm, y: 1.2cm),
  fill: rgb("#faf5ff")
)
#set text(font: "Linux Libertine", size: 8pt, fill: rgb("#0f172a"))

#let tax_id = sys.inputs.at("tax_id", default: "MAGV990623HDFLLC00")
#let empresa = sys.inputs.at("empresa", default: "FIRMA W16 MALLEN®")
#let fecha = sys.inputs.at("fecha", default: "17 de Octubre, 2026")

#grid(
  columns: (1fr, auto),
  gutter: 10pt,
  [
    #text(weight: "bold", size: 11pt, fill: rgb("#581c87"))[DICTAMEN PERICIAL DE AUDITORÍA LEGAL, FISCAL Y REGULATORIA] \
    #text(weight: "medium", size: 8pt, fill: rgb("#6b21a8"))[EVALUACIÓN INTEGRAL DE RIESGO NORMATIVO Y PROTECCIÓN PATRIMONIAL]
  ],
  [
    #rect(fill: rgb("#f3e8ff"), stroke: 0.5pt + rgb("#c084fc"), inset: 5pt, radius: 4pt)[
      #text(weight: "bold", fill: rgb("#6b21a8"))[SCORE: 85 PUNTOS]
    ]
  ]
)

#v(8pt)

#rect(width: 100%, fill: rgb("#ffffff"), stroke: 0.5pt + rgb("#e9d5ff"), inset: 8pt, radius: 4pt)[
  #grid(
    columns: (1fr, 1fr),
    row-gutter: 6pt,
    [ *EMPRESA AUDITADA:* #empresa ], [ *IDENTIFICADOR / TAX ID:* #tax_id ],
    [ *SECTOR / ACTIVIDAD:* Consultoría y Servicios Corporativos ], [ *FECHA DE DICTAMEN:* #fecha ],
    [ *UBICACIÓN:* Ciudad de México, México ], [ *ESTATUS:* #text(fill: rgb("#dc2626"), weight: "bold")[RIESGO CRÍTICO DETECTADO] ]
  )
]

#v(8pt)

#rect(width: 100%, fill: rgb("#ff2a2a0d"), stroke: 0.5pt + rgb("#fca5a5"), inset: 8pt, radius: 4pt)[
  #align(center)[
    #text(size: 8pt, weight: "bold", fill: rgb("#991b1b"))[EXPOSICIÓN TOTAL ESTIMADA A SANCIONES ACUMULADAS:] \
    #text(size: 14pt, weight: "extrabold", fill: rgb("#dc2626"))[\$ 1,188,000.00 MXN]
  ]
]

#v(10pt)

#text(weight: "bold", size: 9pt, fill: rgb("#581c87"))[3. DESGLOSE DETALLADO DE MULTAS POTENCIALES POR INSTITUCIÓN]
#v(4pt)
#table(
  columns: (1.5fr, 1.2fr, 2.3fr),
  fill: (x, y) => if y == 0 { rgb("#f3e8ff") } else { rgb("#ffffff") },
  stroke: 0.3pt + rgb("#e9d5ff"),
  align: (col, row) => if row == 0 { center } else { left },
  [*INSTITUCIÓN / NORMATIVA*], [*RANGO DE MULTA*], [*ORIGEN Y JUSTIFICACIÓN DEL PUNTAJE*],
  [STPS (NOM-035/037/019)], [\$162,855 - \$434,280 MXN], [12 pts: Omisión de Comisiones Mixtas y políticas psicosociales.],
  [SAT (CFF Art. 69-B)], [\$200,000 - \$500,000 MXN], [10 pts: Falta de entregables y expediente de materialidad.],
  [INAI (Ley LFPDPPP)], [\$21,714 - \$347,424 MXN], [10 pts: Deficiencia en tratamiento de datos personales.],
  [IMSS (LSS / Opinión 32-D)], [\$30,000 - \$180,000 MXN], [7 pts: Inconsistencias registradas y riesgo de opinión negativa.],
  [PROTECCIÓN CIVIL], [\$50,000 - \$250,000 MXN], [5 pts: Ausencia de Programa Interno PIPC y riesgo de clausura.],
  [PROFECO (LFPC Art. 76 BIS)], [\$50,000 - \$150,000 MXN], [5 pts: Términos comerciales y contratos no visibles.]
)

#v(10pt)

#rect(width: 100%, fill: rgb("#ffffff"), stroke: 0.5pt + rgb("#e9d5ff"), inset: 8pt, radius: 4pt)[
  #grid(
    columns: (1fr, 1fr),
    [
      #text(size: 7.5pt)[
        *EMISOR RESPONSABLE:* FIRMA W16 MALLEN® \
        *AUDITOR DESIGNADO:* FIRMA W16 MALLEN® \
        *FOLIO REGISTRAL:* W16-2026-CERTIFIED
      ]
    ],
    [
      #align(right)[
        #text(size: 7.5pt)[
          *OFERTA DE ACTIVACIÓN:* \$ 5,940.00 MXN \
          *SELLO DIGITAL:* Merkle Verified SHA-256 \
          #text(fill: rgb("#6b21a8"), weight: "bold")[https://mallen.lat/checkout?ref=#tax_id]
        ]
      ]
    ]
  )
]
