#set page(
  paper: "a4", 
  margin: (x: 1.2cm, top: 1.2cm, bottom: 1.2cm),
  header: align(right)[#text(size: 6.8pt, fill: rgb("#64748b"), weight: "bold")[FIRMA PERICIAL MALLEN | ACREDITACIÓN TÉCNICA Y NORMATIVA]],
  footer: [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(1.5pt)
    #grid(
      columns: (1fr, 1fr),
      align(left)[#text(size: 6.8pt, fill: rgb("#64748b"))[Folio Registral: #text(weight: "bold", fill: rgb("#334155"))[{{NORMA_FOLIO_PREFIX}}]]],
      align(right)[#text(size: 6.8pt, fill: rgb("#64748b"))[Constancia NOM-151-SCFI / Merkle Validated]]
    )
  ]
)
#set text(font: "Liberation Serif", size: 7.5pt, fill: rgb("#1e293b"))

// ENCABEZADO PRINCIPAL EN TONO MORADO SUAVE Y TRANSPARENTE
#rect(width: 100%, fill: rgb("#faf5ff"), stroke: 0.5pt + rgb("#d8b4fe"), inset: (x: 8pt, y: 7pt), radius: 2.5pt)[
  #grid(
    columns: (1fr, auto),
    align: horizon,
    [
      #text(size: 10pt, weight: "bold", fill: rgb("#1e293b"))[CERTIFICADO DE ACREDITACIÓN TÉCNICA]\
      #v(1.5pt)
      #text(size: 7.5pt, fill: rgb("#7c3aed"), weight: "bold")[{{NORMA_FOLIO_PREFIX}} — {{NORMA_TITULO}}]
    ],
    [
      #box(fill: rgb("#16a34a"), inset: (x: 6pt, y: 2.5pt), radius: 2pt)[
        #text(size: 7pt, fill: white, weight: "bold")[ESTATUS: ACREDITADO]
      ]
    ]
  )
]

#v(0.2em)

// FICHA TÉCNICA (CAJA GRIS CLARO)
#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("#cbd5e1"), inset: 5pt, radius: 2.5pt)[
  #grid(
    columns: (1fr, 1fr),
    row-gutter: 3pt,
    [*Entidad Auditada:* {{EMPRESA_NOMBRE}}],
    [*RFC / Tax ID:* {{EMPRESA_TAX_ID}}],
    [*Giro / Sector:* Consultoría y Servicios Corporativos],
    [*Jurisdicción:* CDMX / Federal],
    [*Auditor Cargo:* C.P.C. Víctor Omar Malpica Guillen],
    [*Hash de Inalterabilidad:* raw("SHA256-W16-CONFIRMED")]
  )
]

#v(0.2em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[1. MARCO JURÍDICO, ALCANCE & FUNDAMENTACIÓN PERICIAL]
#v(0.1em)
#text(size: 6.8pt)[El Consejo Pericial de Firma Pericial Mallen hace constar que la entidad *{{EMPRESA_NOMBRE}}* ({{EMPRESA_TAX_ID}}) ha sido sometida a los procedimientos de auditoría cuantitativa, revisión documental y peritaje de campo con respecto a la disposición *{{NORMA_FOLIO_PREFIX}}*.]

#v(0.15em)

// CAJA CON ROJO TRANSPARENTOSO (VECTOR DE EVALUACIÓN)
#rect(width: 100%, fill: rgb("#ff2a2a0d"), stroke: 0.5pt + rgb("#ff2a2a40"), inset: 4.5pt, radius: 2.5pt)[
  #text(size: 6.6pt)[*Vector de Evaluación:* {{NORMA_DESCRIPCION}}]
]

#v(0.2em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[2. MATRIZ PARAMÉTRICA DE INMUNIDAD & DEFENDABILIDAD]
#v(0.1em)

#table(
  columns: (2fr, 1fr, 1fr),
  fill: (x, y) => if y == 0 { rgb("#475569") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: 0.35pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.2pt)[Concepto Evaluado]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Resultado]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Nivel de Riesgo]],
  [Auditoría Documental & Papeles de Trabajo], [SATISFACTORIO], [Inmune (0% Riesgo)],
  [Sustancia Operativa e Infraestructura], [VERIFICADO], [Cumplimiento Alto],
  [Cadena de Inalterabilidad Criptográfica], [SELLADO], [Validez Plena]
)

#v(0.1em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.4pt + rgb("#cbd5e1"), inset: 3.5pt, radius: 2pt)[
  #text(size: 6.4pt, fill: rgb("#475569"))[*Dictamen:* La entidad cuenta con el expediente de prueba necesario para desvirtuar cualquier observación de la autoridad fiscalizadora, garantizando la protección patrimonial de los socios y representantes legales.]
]

#v(0.2em)

// CÉDULA DE AUDITORÍA INTEGRADA EN LA MISMA PÁGINA
#block(fill: rgb("#475569"), inset: (x: 7pt, y: 4.5pt), radius: 2pt, width: 100%)[
  #text(size: 8.5pt, weight: "bold", fill: rgb("#ffffff"))[CÉDULA DE AUDITORÍA PERICIAL Y EVACUACIÓN DE PRUEBAS]
]

#v(0.15em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[3. DETALLE DE PAPELES DE TRABAJO AUDITADOS]
#v(0.1em)

#table(
  columns: (1fr, 2.2fr, 0.8fr),
  fill: (x, y) => if y == 0 { rgb("#475569") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: 0.35pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.2pt)[Código Elemento]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Estructura de Evidencia Auditada]],
  [#text(fill: white, weight: "bold", size: 6.2pt)[Estado]],
  [PT-{{NORMA_FOLIO_PREFIX}}-01], [Pólizas, Contratos y Anexos de Servicio], [VALIDADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-02], [Formatos Oficiales y Registros de Control], [VERIFICADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-03], [Archivos Criptográficos Hash SHA-256], [CONSERVADO],
  [PT-{{NORMA_FOLIO_PREFIX}}-04], [Entregables Técnicos e Informes Periciales], [APROBADO]
)

#v(0.15em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#334155"))[4. CRITERIOS DE DEFENDABILIDAD JURÍDICA & PROCESAL]
#v(0.1em)

#rect(width: 100%, fill: rgb("#ff2a2a0d"), stroke: 0.5pt + rgb("#ff2a2a40"), inset: 4.5pt, radius: 2.5pt)[
  #text(size: 6.4pt)[El presente informe pericial ha sido elaborado por Contadores Públicos Certificados y Peritos en Auditoría Corporativa. Las evidencias integradas constituyen prueba documental plena conforme a los Artículos 210-A del Código Federal de Procedimientos Civiles y la NOM-151-SCFI-2016.]
]

#v(0.8em)

#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 0.4pt + rgb("#cbd5e1"), inset: 5pt, radius: 2.5pt)[
  #grid(
    columns: (1fr, 1fr),
    align: center,
    [
      #line(length: 65%, stroke: 0.6pt + rgb("#334155"))
      #v(1.5pt)
      #text(weight: "bold", size: 7pt)[FIRMA PERICIAL COLEGIADA]\
      #v(0.5pt)
      #text(size: 6.2pt, fill: rgb("#64748b"))[Inmunidad Corporativa Consulting]\
      #text(size: 5.8pt, fill: rgb("#94a3b8"))[C.P.C. Víctor Omar Malpica Guillen]
    ],
    [
      #line(length: 65%, stroke: 0.6pt + rgb("#334155"))
      #v(1.5pt)
      #text(weight: "bold", size: 7pt)[AUDITOR RESPONSABLE DE SECTOR]\
      #v(0.5pt)
      #text(size: 6.2pt, fill: rgb("#64748b"))[Consejo de Dictaminación W16]\
      #text(size: 5.8pt, fill: rgb("#94a3b8"))[Reg. CED-2026-W16]
    ]
  )
]
