#set page(paper: "a4", flipped: true, margin: (x: 2cm, y: 1.8cm))
#set text(size: 10pt)
#set par(justify: true)

// Borde Perimetral Institucional Tipo Diploma
#rect(width: 100%, height: 100%, stroke: 2pt + rgb("#0f172a"), inset: 12pt)[
  #rect(width: 100%, height: 100%, stroke: 0.8pt + rgb("#cbd5e1"), inset: 15pt)[
    #align(center)[
      #v(0.5em)
      #text(fill: rgb("#0f172a"), weight: "bold", size: 18pt)[CONSTANCIA DE ACREDITACIÓN Y CUMPLIMIENTO NORMATIVO] \
      #v(0.3em)
      #text(fill: rgb("{{NORMA_COLOR}}"), weight: "bold", size: 11.5pt)[{{NORMA_TITULO}}] \
      #v(0.2em)
      #line(length: 60%, stroke: 1.2pt + rgb("{{NORMA_COLOR}}"))
      #v(0.8em)

      #text(size: 10.5pt)[Se otorga el presente Certificado Inmunológico de Conformidad a:] \
      #v(0.4em)
      #text(fill: rgb("#0f172a"), weight: "bold", size: 16pt)[{{EMPRESA_NOMBRE}}] \
      #text(fill: rgb("#475569"), size: 10pt)[RFC / TAX ID: {{EMPRESA_TAX_ID}}] \
      #v(0.8em)

      #block(width: 85%)[
        {{NORMA_DESCRIPCION}}
      ]

      #v(1.2em)

      #grid(
        columns: (1fr, 1fr, 1fr),
        gutter: 1em,
        [
          #text(weight: "bold", size: 8.5pt)[FECHA DE EMISIÓN] \
          #v(0.2em)
          #text(size: 8.5pt)[05 de Octubre, 2026]
        ],
        [
          #text(weight: "bold", size: 8.5pt)[VIGENCIA DE COBERTURA] \
          #v(0.2em)
          #text(size: 8.5pt)[1 Año Natural (2026 - 2027)]
        ],
        [
          #text(weight: "bold", size: 8.5pt)[FOLIO DE REGISTRO] \
          #v(0.2em)
          #text(size: 8.5pt)[{{NORMA_FOLIO_PREFIX}}-{{REF_TAX}}]
        ]
      )

      #v(1.6em)

      #grid(
        columns: (1fr, 1fr),
        gutter: 2em,
        [
          #line(length: 70%, stroke: 0.8pt + rgb("#0f172a"))
          #v(0.2em)
          #text(weight: "bold", size: 9pt)[C.P.C. Víctor Omar Malpica Guillen] \
          #text(size: 7.5pt)[Auditor Pericial Designado | Reg. CED-2026-B2B]
        ],
        [
          #line(length: 70%, stroke: 0.8pt + rgb("#0f172a"))
          #v(0.2em)
          #text(weight: "bold", size: 9pt)[Inmunidad B2B Consulting S.A.S. de C.V.] \
          #text(size: 7.5pt)[Entidad Emisora de Certificación Corporativa]
        ]
      )

      #v(1em)
      #rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("#cbd5e1"), inset: 4pt)[
        #text(size: 6pt, fill: rgb("#64748b"))[SELLO DIGITAL DE AUTENTICIDAD SHA-256: raw("{{AUDIT_HASH}}")]
      ]
    ]
  ]
]
