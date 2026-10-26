#set page(
  paper: "a4",
  margin: (x: 1.2cm, top: 1.2cm, bottom: 1.2cm),
  header: align(right)[#text(size: 6.8pt, fill: rgb("#64748b"), weight: "bold")[FIRMA PERICIAL MALLEN | ACREDITACIÓN TÉCNICA Y NORMATIVA B2B]],
  footer: [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(1.5pt)
    #grid(
      columns: (1fr, 1fr),
      align(left)[#text(size: 6.8pt, fill: rgb("#64748b"))[Folio Registral: #text(weight: "bold", fill: rgb("#334155"))[{{NORMA_FOLIO_PREFIX}}-{{REF_TAX}}]]],
      align(right)[#text(size: 6.8pt, fill: rgb("#64748b"))[Evidencia Criptográfica SHA-256 / Merkle L2 Validated — mallen.lat/verify]]
    )
  ]
)
#set text(size: 7.5pt, fill: rgb("#1e293b"))

#rect(width: 100%, fill: rgb("#faf5ff"), stroke: 0.5pt + rgb("#d8b4fe"), inset: (x: 8pt, y: 7pt), radius: 2.5pt)[
  #grid(
    columns: (1fr, auto),
    align: horizon,
    [
      #text(size: 10pt, weight: "bold", fill: rgb("#1e293b"))[CERTIFICADO DE INSTRUMENTACIÓN Y BLINDAJE TÉCNICO]\
      #v(1.5pt)
      #text(size: 7.5pt, fill: rgb("#7c3aed"), weight: "bold")[{{NORMA_FOLIO_PREFIX}} — {{NORMA_TITULO}}]
    ],
    [
      #box(fill: rgb("#16a34a"), inset: (x: 6pt, y: 2.5pt), radius: 2pt)[
        #text(size: 7pt, fill: white, weight: "bold")[EXPEDIENTE INTEGRADO]
      ]
    ]
  )
]

#v(0.2em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("#cbd5e1"), inset: 5pt, radius: 2.5pt)[
  #grid(
    columns: (1fr, 1fr),
    row-gutter: 3pt,
    [*Entidad Auditada:* {{EMPRESA_NOMBRE}}],
    [*RFC / Tax ID:* {{EMPRESA_TAX_ID}}],
    [*Giro / SCIAN:* {{EMPRESA_ACTIVIDAD}}],
    [*Fecha de Emisión:* {{FECHA_EMISION}}],
    [*Auditor Dictaminador:* C.P.C. Víctor Omar Malpica Guillén],
    [*Hash SHA-256:* #raw("{{AUDIT_HASH}}")]
  )
]

#v(0.2em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[1. MARCO JURÍDICO, ALCANCE & FUNDAMENTACIÓN PERICIAL]
#v(0.1em)
#text(size: 6.8pt)[El Consejo Pericial de Firma Pericial Mallen hace constar que para la entidad *{{EMPRESA_NOMBRE}}* ({{EMPRESA_TAX_ID}}) se han compilado y estructurado los instrumentos documentales, protocolos de control y guías de cumplimiento correspondientes al vector normativo *{{NORMA_FOLIO_PREFIX}}*.]

#v(0.15em)

#rect(width: 100%, fill: rgb("#ff2a2a0d"), stroke: 0.5pt + rgb("#ff2a2a40"), inset: 4.5pt, radius: 2.5pt)[
  #text(size: 6.6pt)[*Alcance Normativo Cubierto:* {{NORMA_DESCRIPCION}}]
]

#v(0.2em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[2. MATRIZ PARAMÉTRICA DE DEFENDABILIDAD DOCUMENTAL]
#v(0.1em)

#table(
  columns: (2fr, 1fr, 1fr),
  fill: (x, y) => if y == 0 { rgb("#475569") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: 0.35pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.2pt)[Concepto Evaluado]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Estatus de Instrumento]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Nivel de Cobertura]],
  [Instrumentos Jurídicos y Políticas Base], [ENTREGADO EN KIT ZIP], [Cobertura Normativa Alta],
  [Trazabilidad y Cruce Registral Público (OSINT)], [VERIFICADO], [Alineado a SCIAN / RFC],
  [Sello de Tiempo e Inalterabilidad Criptográfica], [SELLADO SHA-256], [Fecha Cierta Digital]
)

#v(0.1em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.4pt + rgb("#cbd5e1"), inset: 3.5pt, radius: 2pt)[
  #text(size: 6.4pt, fill: rgb("#475569"))[*Observación Pericial:* Una vez impresos y firmados autógrafamente por el Representante Legal y las comisiones correspondientes, los instrumentos de este expediente constituyen soporte documental de defensa ante requerimientos de la autoridad.]
]

#v(0.2em)

#block(fill: rgb("#475569"), inset: (x: 7pt, y: 4.5pt), radius: 2pt, width: 100%)[
  #text(size: 8.5pt, weight: "bold", fill: rgb("#ffffff"))[CÉDULA DE PAPELES DE TRABAJO E INSTRUMENTOS DEL KIT]
]

#v(0.15em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[3. DETALLE DE ELEMENTOS INTEGRADOS EN EL PAQUETE CORPORATIVO]
#v(0.1em)

#table(
  columns: (1fr, 2.2fr, 0.8fr),
  fill: (x, y) => if y == 0 { rgb("#475569") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: 0.35pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.2pt)[Código Elemento]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Estructura de Evidencia Integrada]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Estado]],
  [PT-{{NORMA_FOLIO_PREFIX}}-01], [Pólizas, Políticas, Contratos Marco y Anexos Normativos], [GENERADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-02], [Listas de Verificación, Actas Constitutivas y Bitácoras], [PRE-LLENADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-03], [Registro Criptográfico SHA-256 y Árbol de Merkle L2], [SELLADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-04], [Dictamen Pericial de Exposición y Remediación], [APROBADO]
)

#v(0.15em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[4. CRITERIOS DE DEFENDABILIDAD JURÍDICA & ALCANCE]
#v(0.1em)

#rect(width: 100%, fill: rgb("#ff2a2a0d"), stroke: 0.5pt + rgb("#ff2a2a40"), inset: 4.5pt, radius: 2.5pt)[
  #text(size: 6.2pt)[El presente certificado acredita la emisión de los instrumentos técnicos elaborados por Firma Pericial Mallen conforme al Artículo 210-A del Código Federal de Procedimientos Civiles y Artículos 89 a 94 del Código de Comercio. La ejecución física en sitio recae bajo responsabilidad de la administración de la empresa.]
]

#v(0.6em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.4pt + rgb("#cbd5e1"), inset: 5pt, radius: 2.5pt)[
  #grid(
    columns: (1fr, 1fr),
    align: center,
    [
      #line(length: 65%, stroke: 0.6pt + rgb("#334155"))
      #v(1.5pt)
      #text(weight: "bold", size: 7pt)[FIRMA PERICIAL COLEGIADA]\
      #v(0.5pt)
      #text(size: 6.2pt, fill: rgb("#64748b"))[Firma Pericial Mallen (`mallen.lat`)]\
      #text(size: 5.8pt, fill: rgb("#94a3b8"))[C.P.C. Víctor Omar Malpica Guillén]
    ],
    [
      #line(length: 65%, stroke: 0.6pt + rgb("#334155"))
      #v(1.5pt)
      #text(weight: "bold", size: 7pt)[AUDITORÍA ALGORÍTMICA Y FECHA CIERTA]\
      #v(0.5pt)
      #text(size: 6.2pt, fill: rgb("#64748b"))[Consejo de Dictaminación Pericial]\
      #text(size: 5.8pt, fill: rgb("#94a3b8"))[Reg. CED-2026-MALLEN]
    ]
  )
]
