#set page(paper: "a4", margin: (x: 1.8cm, y: 2.2cm), header: align(right)[#text(size: 8pt, fill: rgb("#64748b"))[FIRMA PERICIAL MALLEN | DICTAMEN DE AUDITORÍA CORPORATIVA W16]], footer: [#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1")) #grid(columns: (1fr, 1fr), align(left)[#text(size: 7pt, fill: rgb("#94a3b8"))[Expediente Fiscal: #text(weight: "bold")[DEMO_NUEVO2026]]], align(right)[#text(size: 7pt, fill: rgb("#94a3b8"))[Validez Probatoria Plena — NOM-151-SCFI]])])
#set text(size: 9.5pt, fill: rgb("#0f172a"))

#block(fill: rgb("#0b0f19"), inset: 16pt, radius: 6pt, width: 100%, [#grid(columns: (1fr, auto), align: horizon, [#text(size: 15pt, weight: "bold", fill: rgb("#ffffff"))[DICTAMEN PERICIAL DE AUDITORÍA INTEGRAL] #v(4pt) #text(size: 9.5pt, fill: rgb("#38bdf8"), weight: "bold")[Sustento Legal, Fiscal y Cumplimiento Normativo B2B]], [#rect(fill: rgb("#1e293b"), inset: 8pt, radius: 4pt)[#text(size: 9pt, fill: rgb("#34d399"), weight: "bold")[INMUNIDAD: 98.4%]]])])

#v(10pt)
#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#e2e8f0"), inset: 12pt, radius: 4pt)[#grid(columns: (1fr, 1fr), row-gutter: 8pt, [*Razon Social / Entidad:* Empresa Auditada (DEMO_NUEVO2026)], [*RFC / Tax ID:* DEMO_NUEVO2026], [*Giro / Sector Operativo:* Consultoría y Servicios Corporativos], [*Jurisdiccion / Estado:* CDMX], [*Seguridad Informatica:* SEGURO (Cifrado TLS 1.3 Active)], [*Diagnostico INAI / ARCO:* Aviso de Privacidad Validado en Dominio Institucional])]

#v(10pt)
== 1. Resumen Pericial de Inmunidad & Contingencia Fiscal
#text(size: 9pt)[El Consejo Pericial de Auditoría Corporativa de Firma Pericial Mallen emite el presente Dictamen de Evaluación Cuantitativa de Riesgos. Mediante el algoritmo de peritaje W16, se evaluaron los vectores de inspección crítica de la STPS, SAT, IMSS, INAI, PROFECO y la UIF (Ley Anti-Lavado), determinando la suficiencia probatoria y el índice de blindaje patrimonial de la entidad auditada.]

#v(8pt)
#table(columns: (2.2fr, 1fr, 1fr), fill: (x, y) => if y == 0 { rgb("#1e293b") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") }, stroke: 0.5pt + rgb("#cbd5e1"), align: (col, row) => if row == 0 { center } else { left }, [#text(fill: white, weight: "bold")[Vector Regulatorio & Organo Fiscalizador]], [#text(fill: white, weight: "bold")[Estatus Pericial]], [#text(fill: white, weight: "bold")[Contingencia Finera]], [Seguridad & Salud Laboral (STPS NOM-035 / NOM-037)], [ACREDITADO], [Inmune (0% Riesgo)], [Materialidad & Razon de Negocios (SAT Art. 69-B / 5-A)], [VERIFICADO], [Sustancia Aprobada], [Seguridad Social & Integracion SBC (IMSS Art. 72)], [VALIDADO], [Inmune (Sin Omisiones)], [Gobernanza de Datos & Derechos ARCO (INAI LFPDPPP)], [AUDITADO], [Bajo Protocolo], [Mensajes de Datos & Conservacion Criptografica (NOM-151)], [SELLADO], [Validez Plena (TFJA)])

#v(14pt)
#align(center)[#line(length: 40%, stroke: 1pt + rgb("#0f172a")) #v(2pt) #text(weight: "bold", size: 10pt)[CONSEJO PERICIAL COLEGIADO C.P.C.] #v(1pt) #text(size: 8.5pt, fill: rgb("#64748b"))[Firma Pericial Mallen — Audit & Risk Management Services] #v(1pt) #text(size: 8pt, fill: rgb("#94a3b8"))[Cedula Profesional Reg. 948201 — Sistema Pericial W16]]
