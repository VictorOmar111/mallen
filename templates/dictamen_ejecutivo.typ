#set page(paper: "a4", margin: (x: 1.2cm, top: 1.2cm, bottom: 1.2cm))
#set text(font: "Liberation Serif", size: 8.5pt, fill: rgb("#0f172a"))

#show heading: it => block(below: 5pt, above: 7pt)[
  #text(fill: rgb("#0b132b"), weight: "bold", size: 8.5pt)[#it.body]
]

// ENCABEZADO PRINCIPAL
#block(fill: rgb("#0b132b"), inset: (x: 10pt, y: 8pt), radius: 2pt, width: 100%, align(center)[
  #text(size: 12pt, weight: "bold", fill: rgb("#ffffff"))[DICTAMEN PERICIAL DE AUDITORÍA LEGAL, FISCAL Y REGULATORIA]\
  #v(2pt)
  #text(size: 7.5pt, fill: rgb("#cbd5e1"), weight: "bold")[EVALUACIÓN INTEGRAL DE RIESGO NORMATIVO Y PROTECCIÓN PATRIMONIAL B2B]
])

#v(4pt)

// FICHA TÉCNICA Y SCORE CON DONA CONCÉNTRICA EXACTA
#grid(columns: (1fr, 1.2fr), gutter: 8pt, [
  #rect(width: 100%, stroke: 0.5pt + rgb("#cbd5e1"), inset: 6pt, radius: 2pt)[
    *EMPRESA AUDITADA:* Restaurante El Timón\
    *IDENTIFICADOR / TAX ID:* TAX_1\
    *SECTOR / ACTIVIDAD:* El Timon S.A. de C.V.\
    *UBICACIÓN / CONTACTO:* http://eltimon.com\
    *FECHA DE DICTAMEN:* 05 de Octubre, 2026\
    *ESTATUS AUDITORÍA:* #text(fill: rgb("#dc2626"), weight: "bold")[RIESGO CRÍTICO DETECTADO]
  ]
], [
  #rect(width: 100%, stroke: 0.5pt + rgb("#cbd5e1"), inset: 5pt, radius: 2pt)[
    #align(center)[#text(size: 7.5pt, weight: "bold")[SCORE DE RIESGO: 49 PUNTOS (ACUMULADO)]]
    #v(3pt)
    #grid(columns: (60pt, 1fr), align: horizon, [
      // DIBUJO DE ANILLOS CONCÉNTRICOS
      #place(center + horizon)[#circle(radius: 24pt, stroke: 2.5pt + rgb("#2563eb"))]
      #place(center + horizon)[#circle(radius: 20pt, stroke: 2.5pt + rgb("#dc2626"))]
      #place(center + horizon)[#circle(radius: 16pt, stroke: 2.5pt + rgb("#16a34a"))]
      #place(center + horizon)[#circle(radius: 12pt, stroke: 2.5pt + rgb("#ea580c"))]
      #place(center + horizon)[#circle(radius: 8pt, stroke: 2.5pt + rgb("#9333ea"))]
      #place(center + horizon)[#circle(radius: 5.5pt, fill: white)]
      #place(center + horizon)[#text(size: 7.5pt, weight: "bold")[49]#v(-3pt)#text(size: 4pt, fill: rgb("#64748b"))[PTS]]
    ], [
      #text(size: 6.5pt)[
        • #text(fill: rgb("#2563eb"))[*STPS:* 12 pts (Normativa Laboral)]\
        • #text(fill: rgb("#dc2626"))[*SAT:* 10 pts (Fiscal/Materialidad)]\
        • #text(fill: rgb("#16a34a"))[*INAI:* 10 pts (Privacidad Datos)]\
        • #text(fill: rgb("#16a34a"))[*IMSS:* 7 pts (Opinión 32-D)]\
        • #text(fill: rgb("#ea580c"))[*PROT. CIVIL:* 5 pts (Riesgo Operativo)]\
        • #text(fill: rgb("#9333ea"))[*PROFECO:* 5 pts (Comercial)]
      ]
    ])
  ]
])

#v(4pt)

// BANNER EXPOSICIÓN
#rect(width: 100%, fill: rgb("#fff5f5"), stroke: 0.6pt + rgb("#fca5a5"), inset: 5pt, radius: 2pt)[
  #grid(columns: (1fr, auto), align: horizon, [
    #text(size: 7.5pt, weight: "bold", fill: rgb("#7f1d1d"))[EXPOSICIÓN TOTAL ESTIMADA A SANCIONES ACUMULADAS:]\
    #v(1pt)
    #text(size: 11pt, weight: "bold", fill: rgb("#dc2626"))[\$ 738,276.00 MXN]
  ], [
    #rect(fill: rgb("#dc2626"), inset: 4pt, radius: 2pt)[#text(size: 7.5pt, fill: white, weight: "bold")[CRÍTICO: SANCIONABLE]]
  ])
]

#v(4pt)

== 1. RESUMEN EJECUTIVO Y METODOLOGÍA DE EVALUACIÓN
#text(size: 7.2pt)[El presente dictamen analiza el nivel de exposición legal, fiscal y administrativa de la entidad *"Restaurante El Timón"*. La puntuación acumulada de *49 Puntos* se determina ponderando 3 vectores: *Sanción Económica (\$MXN)*, *Probabilidad de Inspección por Rastreo OSINT* y *Riesgo de Medidas Cautelares Operativas*.]

== 2. MARCO NORMATIVO Y FUNDAMENTOS DE AUDITORÍA DIRECTA
#text(size: 7pt)[
  La auditoría evalúa el grado de cumplimiento de las facultades de inspección vigentes:\
  • *STPS:* LFT Arts. 540 y 1002 (NOM-035, NOM-037, Comisiones Mixtas CSH).\
  • *SAT:* CFF Art. 69-B (Soporte documental de materialidad de operaciones B2B).\
  • *INAI:* Ley LFPDPPP Arts. 60-64 (Auditoría de _Avisos de Privacidad_ y transferencia de datos).\
  • *IMSS e INFONAVIT:* LSS Arts. 15 y 304 (Opinión de Cumplimiento 32-D, Primas de Riesgo y Subcontratación REPSE).\
  • *PROTECCIÓN CIVIL:* Ley General de Protección Civil (Programa Interno PIPC y Dictamen de Seguridad Operativa).\
  • *PROFECO:* LFPC Art. 76 BIS (Términos comerciales e información en plataformas digitales).
]

== 3. DESGLOSE DETALLADO DE MULTAS POTENCIALES POR INSTITUCIÓN
#table(
  columns: (1.4fr, 1.3fr, 2.3fr),
  fill: (x, y) => if y == 0 { rgb("#0b132b") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  stroke: 0.4pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.8pt)[INSTITUCIÓN / NORMATIVA]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[RANGO DE MULTA]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[ORIGEN Y JUSTIFICACIÓN DEL PUNTAJE]],
  [STPS (NOM-035/037/019)], [\$162,855 - \$434,280 MXN], [12 pts: Omisión de Comisiones Mixtas y políticas psicosociales.],
  [SAT (CFF Art. 69-B)], [\$200,000 - \$500,000 MXN], [10 pts: Falta de entregables y expediente de materialidad.],
  [INAI (Ley LFPDPPP)], [\$21,714 - \$347,424 MXN], [10 pts: ALERTA: Ausencia de Aviso de Privacidad. Alto riesgo por el INAI. Deficiencia en tratamiento de datos.],
  [IMSS (LSS / Opinión 32-D)], [\$30,000 - \$180,000 MXN], [7 pts: Inconsistencias registradas y riesgo de opinión negativa.],
  [PROTECCIÓN CIVIL], [\$50,000 - \$250,000 MXN], [5 pts: Ausencia de Programa Interno PIPC y riesgo de clausura.],
  [PROFECO (LFPC Art. 76 BIS)], [\$50,000 - \$150,000 MXN], [5 pts: Términos comerciales y contratos de adhesión no visibles.]
)

#pagebreak()

// PÁGINA 2
== 4. MATRIZ DE IMPACTO OPERATIVO Y SANCIONES ACCESORIAS
#table(
  columns: (1fr, 1.3fr, 2.2fr),
  fill: (x, y) => if y == 0 { rgb("#0b132b") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  stroke: 0.4pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.8pt)[GRAVEDAD]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[MEDIDA CAUTELAR]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[IMPACTO EN LA OPERACIÓN B2B]],
  [Riesgo Crítico], [Cancelación CSD (SAT)], [Incapacidad inmediata para facturar a clientes comerciales.],
  [Riesgo Alto], [Clausura Física (Prot. Civil)], [Colocación de sellos y suspensión inmediata del centro de trabajo.],
  [Riesgo Moderado], [Opinión 32-D Negativa (IMSS)], [Bloqueo de cobros y retención de pagos por clientes B2B.]
)

#v(6pt)
== 5. COMPARATIVA DE COSTOS: SERVICIOS INDEPENDIENTES VS. BLINDAJE B2B
#table(
  columns: (2.2fr, 1fr, 0.8fr),
  fill: (x, y) => if y == 0 { rgb("#0b132b") } else if y == 7 { rgb("#f1f5f9") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  stroke: 0.4pt + rgb("#cbd5e1"),
  align: (col, row) => if row == 0 { center } else { left },
  [#text(fill: white, weight: "bold", size: 6.8pt)[CONTRATACIÓN DE SERVICIOS POR SEPARADO]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[COSTO MERCADO]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[INCLUIDO]],
  [Implementación y Carpetas NOM-035 / NOM-037 (Despacho Laboral)], [\$25,000.00 MXN], [SÍ],
  [Aviso de Privacidad e Implementación LFPDPPP (Consultor INAI)], [\$12,000.00 MXN], [SÍ],
  [Auditoría de Materialidad Fiscal CFF 69-B (Despacho Fiscal)], [\$35,000.00 MXN], [SÍ],
  [Programa Interno y Carpeta de Protección Civil (Tercero Acreditado)], [\$30,000.00 MXN], [SÍ],
  [Expediente Patrón-IMSS y Opinión 32-D (Gestoría Social)], [\$15,000.00 MXN], [SÍ],
  [Auditoría de Términos Comerciales LFPC (Abogado Comercial)], [\$10,000.00 MXN], [SÍ],
  [*COSTO TOTAL ESTIMADO CON DESPACHOS INDEPENDIENTES:*], [*\$127,000.00 MXN*], [#text(weight: "bold", fill: rgb("#16a34a"))[AHORRO 95%]]
)

#v(6pt)
== 6. DIAGNÓSTICO DE HUELLA DIGITAL Y AUDITORÍA OSINT DETECTADA
#rect(width: 100%, stroke: 0.4pt + rgb("#cbd5e1"), inset: 5pt, radius: 2pt)[
  #grid(columns: (1fr, 1fr, 1fr), row-gutter: 3pt, [*Certificado SSL:*\ SEGURO (HTTPS)], [*Aviso Privacidad:*\ AUSENTE / NO VISIBLE], [*Estatus Servidor:*\ Código 0])
  #v(2pt)
  #text(size: 6.5pt)[*Banderas OSINT:* STPS_NOM_035, INAI_LFPDPPP_PRIVACY, SAT_CFF_69B_CHECK, IMSS_32D_MONITOR, PROT_CIVIL_PIPC, PROFECO_76BIS]
]

#v(6pt)
== 7. KITS Y CERTIFICADOS INCLUIDOS EN ESTE PAQUETE DE ACTIVACIÓN
#text(size: 7pt)[
  • [X] *Certificado y Kit 01:* Política de Prevención NOM-035 y Protocolo NOM-037 (STPS).\
  • [X] *Certificado y Kit 02:* Aviso de Privacidad Integral Ajustado con Cláusulas ARCO (INAI).\
  • [X] *Certificado y Kit 03:* Expediente de Soporte y Materialidad Fiscal B2B (SAT Art. 69-B).\
  • [X] *Certificado y Kit 04:* Carpeta de Seguridad Operativa y Programa Interno PIPC (Protección Civil).\
  • [X] *Certificado y Kit 05:* Protocolo de Cumplimiento Patrón-IMSS y Expediente REPSE.
]

#v(6pt)
== 8. FIRMA E INSTITUCIONALIDAD PERICIAL CORPORATIVA
#text(size: 7pt)[
  *EMISOR RESPONSABLE:* Inmunidad B2B Consulting S.A.S. de C.V.\
  *AUDITOR DESIGNADO:* C.P.C. Víctor Omar Malpica Guillen (Reg. CED-2026-B2B)\
  *SELLO DIGITAL CORPORATIVO (SHA-256):* raw("0x67961ce4b285")
]

#v(6pt)
#rect(width: 100%, fill: rgb("#fff5f5"), stroke: 0.8pt + rgb("#fca5a5"), inset: 6pt, radius: 3pt)[
  #align(center)[
    #text(size: 9.5pt, weight: "bold", fill: rgb("#dc2626"))[OFERTA DE ACTIVACIÓN DE BLINDAJE INTEGRAL: \$ 5,940.00 MXN]\
    #v(2pt)
    #text(size: 6.8pt, fill: rgb("#7f1d1d"))[(Cuota fija correspondiente al 0.5% de la exposición promedio calculada: \$ 1,188,000.00 MXN)]\
    #v(2pt)
    #text(size: 7pt, weight: "bold")[Liga de Activación Inmediata: raw("https://inmunidadb2b.com/checkout?ref=TAX_1")]
  ]
]

#v(4pt)
#align(center)[#text(size: 5pt, fill: rgb("#94a3b8"))[CLÁUSULA DE CADUCIDAD: Este dictamen pericial tiene una validez de 15 días naturales a partir de su emisión. Transcurrido dicho plazo, las sanciones expuestas y los costos de regularización quedan sujetos a actualización fiscal.]]
