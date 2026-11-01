#set page(paper: "a4", flipped: true, margin: (x: 2cm, y: 1.8cm))
#set text(size: 10pt)
#set par(justify: true)

#rect(width: 100%, height: 100%, stroke: 2pt + rgb("#0f172a"), inset: 12pt)[
  #rect(width: 100%, height: 100%, stroke: 0.8pt + rgb("#cbd5e1"), inset: 15pt)[
    #align(center)[
      #v(0.5em)
      #text(fill: rgb("#0f172a"), weight: "bold", size: 18pt)[CERTIFICADO DE ACREDITACIÓN DE MATERIALIDAD Y CUMPLIMIENTO] \
      #v(0.3em)
      #text(fill: rgb("#dc2626"), weight: "bold", size: 12pt)[SAT / CFF ART. 69-B & OPINIÓN DE CUMPLIMIENTO 32-D IMSS] \
      #v(0.2em)
      #line(length: 60%, stroke: 1pt + rgb("#dc2626"))
      #v(1em)

      #text(size: 11pt)[Se otorga la presente Certificación Pericial Fiscal-Patronal a:] \
      #v(0.5em)
      #text(fill: rgb("#0f172a"), weight: "bold", size: 16pt)[{{EMPRESA_NOMBRE}}] \
      #text(fill: rgb("#475569"), size: 10pt)[RFC / TAX ID: {{EMPRESA_TAX_ID}}] \
      #v(1em)

      #block(width: 85%)[
        Por haber acreditado el *Expediente de Soporte y Entregables de Materialidad de Operaciones B2B*, desvirtuando presunciones de inexistencia contractual del Art. 69-B del Código Fiscal de la Federación y demostrando el estatus de Opinión Positiva de Cumplimiento de Obligaciones en Materia de Seguridad Social.
      ]

      #v(1.5em)

      #grid(
        columns: (1fr, 1fr, 1fr),
        gutter: 1em,
        [
          #text(weight: "bold", size: 9pt)[FECHA DE EMISIÓN] \
          #v(0.2em)
          #text(size: 9pt)[{{FECHA_EMISION}}]
        ],
        [
          #text(weight: "bold", size: 9pt)[VIGENCIA DE COBERTURA] \
          #v(0.2em)
          #text(size: 9pt)[1 Año Natural (2026 - 2027)]
        ],
        [
          #text(weight: "bold", size: 9pt)[FOLIO DE REGISTRO] \
          #v(0.2em)
          #text(size: 9pt)[SAT-69B-{{REF_TAX}}]
        ]
      )

      #v(2em)

      #grid(
        columns: (1fr, 1fr),
        gutter: 2em,
        [
          #line(length: 70%, stroke: 0.8pt + rgb("#0f172a"))
          #v(0.2em)
          #text(weight: "bold", size: 9.5pt)[C.P.C. Víctor Omar Malpica Guillen] \
          #text(size: 8pt)[Auditor Pericial Designado | Reg. CED-2026-B2B]
        ],
        [
          #line(length: 70%, stroke: 0.8pt + rgb("#0f172a"))
          #v(0.2em)
          #text(weight: "bold", size: 9.5pt)[Inmunidad B2B Consulting S.A.S.] \
          #text(size: 8pt)[Entidad Emisora de Certificación Corporativa]
        ]
      )

      #v(1.2em)
      #rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("#cbd5e1"), inset: 4pt)[
        #text(size: 6.5pt, fill: rgb("#64748b"))[SELLO DIGITAL DE AUTENTICIDAD SHA-256: raw("{{AUDIT_HASH}}")]
      ]
    ]
  ]
]
