#set page(paper: "a4", flipped: true, margin: (x: 2cm, y: 1.8cm))
#set text(size: 10pt)
#set par(justify: true)

#rect(width: 100%, height: 100%, stroke: 2pt + rgb("#0f172a"), inset: 12pt)[
  #rect(width: 100%, height: 100%, stroke: 0.8pt + rgb("#cbd5e1"), inset: 15pt)[
    #align(center)[
      #v(0.5em)
      #text(fill: rgb("#0f172a"), weight: "bold", size: 18pt)[CONSTANCIA DE ACREDITACIÓN Y CUMPLIMIENTO NORMATIVO] \
      #v(0.3em)
      #text(fill: rgb("#2563eb"), weight: "bold", size: 12pt)[SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-037-STPS-2023] \
      #v(0.2em)
      #line(length: 60%, stroke: 1pt + rgb("#2563eb"))
      #v(1em)

      #text(size: 11pt)[Se otorga el presente Certificado Inmunológico de Conformidad a:] \
      #v(0.5em)
      #text(fill: rgb("#0f172a"), weight: "bold", size: 16pt)[{{EMPRESA_NOMBRE}}] \
      #text(fill: rgb("#475569"), size: 10pt)[RFC / TAX ID: {{EMPRESA_TAX_ID}}] \
      #v(1em)

      #block(width: 85%)[
        Por haber acreditado la validación de la *Política de Teletrabajo, Listas de Verificación de Condiciones Ergonométricas y de Seguridad en Lugares de Trabajo Remoto*, en cumplimiento con la Norma Oficial Mexicana *NOM-037-STPS-2023* y el artículo 330-J de la Ley Federal del Trabajo.
      ]

      #v(1.5em)

      #grid(
        columns: (1fr, 1fr, 1fr),
        gutter: 1em,
        [
          #text(weight: "bold", size: 9pt)[FECHA DE EMISIÓN] \
          #v(0.2em)
          #text(size: 9pt)[05 de Octubre, 2026]
        ],
        [
          #text(weight: "bold", size: 9pt)[VIGENCIA DE COBERTURA] \
          #v(0.2em)
          #text(size: 9pt)[1 Año Natural (2026 - 2027)]
        ],
        [
          #text(weight: "bold", size: 9pt)[FOLIO DE REGISTRO] \
          #v(0.2em)
          #text(size: 9pt)[STPS-NOM037-{{REF_TAX}}]
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
