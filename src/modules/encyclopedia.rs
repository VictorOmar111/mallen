use axum::response::Html;

pub struct NormaEducativa {
    pub codigo: &'static str,
    pub titulo: &'static str,
    pub incumplimiento_pct: u8,
    pub fuente_estadistica: &'static str,
    pub rango_multa: &'static str,
    pub contexto_dof: &'static str,
    pub de_que_trata: &'static str,
    pub beneficio_productividad: &'static str,
    pub pasos_cumplimiento: &'static [&'static str],
    pub entregable_kit: &'static str,
}

fn get_category_data(
    category: &str,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    Vec<NormaEducativa>,
) {
    match category {
        "stps" => (
            "STPS • LEY FEDERAL DEL TRABAJO Y LAS 39 NORMAS OFICIALES MEXICANAS",
            "Secretaría del Trabajo y Previsión Social (Catálogo Completo de las 39 NOMs)",
            "Cobertura integral de las 39 NOMs divididas en Seguridad, Salud, Organización, Ergonomía/Psicosocial/Teletrabajo y Específicas.",
            "El marco inspectivo laboral mexicano emana del Artículo 123 Constitucional de 1917, la Ley Federal del Trabajo de 1970 y el Reglamento Federal de Seguridad y Salud en el Trabajo (DOF 13-nov-2014). Se estructura en 5 familias de Normas Oficiales Mexicanas: Seguridad (12 NOMs), Salud y Agentes (8 NOMs), Organización (6 NOMs), Ergonomía/Psicosocial/Teletrabajo (3 NOMs) y Específicas de Sector (7 NOMs + Ferroviario/Agrícola). Conforme al Artículo 992 de la LFT, las multas de 50 a 5,000 UMAs pueden imponerse por cada trabajador afectado.",
            "blue",
            "border-blue-600",
            vec![
                NormaEducativa {
                    codigo: "NOM-035-STPS-2018 • Art. 132 Fracc. XVI y Art. 994 Fracc. V LFT",
                    titulo: "Factores de Riesgo Psicosocial en el Trabajo — Identificación, Análisis y Prevención",
                    incumplimiento_pct: 71,
                    fuente_estadistica: "71% de los centros de trabajo inspeccionados presentan omisiones en la aplicación de las Guías de Referencia I, II o III, o carecen de evidencia firmada de su Política de Prevención Psicosocial.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Publicada en el Diario Oficial de la Federación (DOF) el 23 de octubre de 2018 tras la reforma al RFSST de 2014 para atender el estrés laboral crónico y la violencia organizacional. Cobró vigencia plena en dos etapas: octubre de 2019 (política escrita, medidas preventivas y Guía I) y octubre de 2020 (evaluación psicosocial Guía II y Entorno Organizacional Guía III).",
                    de_que_trata: "Obliga a todos los patrones en México a identificar, analizar y prevenir factores de riesgo psicosocial (cargas excesivas, jornadas superiores a la LFT, liderazgo negativo y violencia laboral), escalando las obligaciones para centros de 1 a 15, de 16 a 50 y de más de 50 trabajadores.",
                    beneficio_productividad: "Disminuye hasta en un 38% el ausentismo laboral y la rotación de personal clave, elimina el agotamiento (burnout), eleva la concentración diaria del equipo y previene demandas laborales por acoso o estrés.",
                    pasos_cumplimiento: &[
                        "Emitir, firmar y difundir físicamente la Política de Prevención de Riesgos Psicosociales (Guía de Referencia IV).",
                        "Aplicar al 100% de la plantilla la Guía de Referencia I para identificar trabajadores expuestos a acontecimientos traumáticos severos.",
                        "Aplicar cada 2 años la Guía de Referencia II (16 a 50 empleados) o Guía III (>50 empleados) y conservar cuestionarios firmados.",
                        "Habilitar un buzón confidencial de quejas por violencia laboral con bitácora de resolución."
                    ],
                    entregable_kit: "02_Politica_Prevencion_Psicosocial_NOM035_STPS.pdf y Certificado NOM-035",
                },
                NormaEducativa {
                    codigo: "NOM-037-STPS-2023 • Artículos 330-A al 330-K de la LFT",
                    titulo: "Teletrabajo — Condiciones de Seguridad y Salud en el Trabajo Remoto",
                    incumplimiento_pct: 78,
                    fuente_estadistica: "78% de las empresas con esquemas de trabajo remoto o híbrido (>40% del tiempo fuera del local) carecen del adendum contractual, de la lista de verificación ergonómica y del desglose de pago proporcional de internet y electricidad.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Emana de la reforma al Capítulo XII Bis de la LFT publicada en el DOF el 11 de enero de 2021. La norma técnica definitiva NOM-037-STPS-2023 se publicó en el DOF el 8 de junio de 2023 y entró en vigor obligatorio el 5 de diciembre de 2023.",
                    de_que_trata: "Regula las condiciones ergonómicas, psicosociales y de seguridad eléctrica para empleados que laboren más del 40% de su jornada en un domicilio distinto al centro de trabajo utilizando tecnologías de la información.",
                    beneficio_productividad: "Incrementa el cumplimiento de metas por objetivos, evita incapacidades por lesiones cervicales o lumbares en casa, protege la información confidencial de la empresa y atrae al mejor talento.",
                    pasos_cumplimiento: &[
                        "Integrar el listado de teletrabajadores y firmar el adendum contractual que estipule los insumos entregados (silla ergonómica, cómputo) y el pago mensual proporcional de electricidad e internet (Art. 330-E Fracc. III LFT).",
                        "Publicar la Política de Teletrabajo reconociendo expresamente el Derecho a la Desconexión Digital fuera de la jornada.",
                        "Aplicar la Lista de Verificación de Condiciones de Seguridad y Salud en el espacio remoto validada por la Comisión Mixta."
                    ],
                    entregable_kit: "04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.pdf y Certificado NOM-037",
                },
                NormaEducativa {
                    codigo: "NOM-036-1-STPS-2018 • Ergonomía y Manejo Manual de Cargas",
                    titulo: "Factores de Riesgo Ergonómico en el Trabajo — Manejo Manual de Cargas",
                    incumplimiento_pct: 68,
                    fuente_estadistica: "68% de los comercios, restaurantes, almacenes y plantas donde el personal levanta, empuja o transporta objetos mayores a 3 kg de forma cotidiana carecen del análisis ergonómico y de vigilancia médica.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Publicada en el DOF el 23 de noviembre de 2018 (en vigor desde enero de 2020 con ajuste de masas máximas en 2023/2024) para abatir los trastornos musculoesqueléticos lumbares, que representan la primera causa de incapacidad permanente en el IMSS.",
                    de_que_trata: "Aplica a todos los centros de trabajo donde existan trabajadores que realicen levantamiento, bajada, empuje, tracción o transporte manual de cargas superiores a 3 kg de manera cotidiana (más de una vez al día).",
                    beneficio_productividad: "Protege la columna y salud física de los trabajadores, evita incapacidades prolongadas (formatos ST-7 y ST-9) y previene que la Prima de Riesgo del IMSS suba un 1% anual cada mes de febrero.",
                    pasos_cumplimiento: &[
                        "Respetar las cargas máximas manuales (25 kg hombres, 20 kg mujeres, 7 kg menores y prohibición para embarazadas).",
                        "Elaborar el Análisis de Factores de Riesgo Ergonómico con los Apéndices I y II de la norma.",
                        "Proporcionar ayudas mecánicas (diablitos, patines hidráulicos, carretillas) y capacitar en higiene de columna con constancia DC-3."
                    ],
                    entregable_kit: "Diagnóstico Ergonómico NOM-036-1 en Dictamen Pericial Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-019-STPS-2011 y NOM-030-STPS-2009 • Organización de la Seguridad",
                    titulo: "Comisiones Mixtas de Seguridad e Higiene (NOM-019) y Servicios Preventivos SST (NOM-030)",
                    incumplimiento_pct: 62,
                    fuente_estadistica: "62% de los centros de trabajo reciben emplazamientos por no exhibir el Acta Constitutiva de la Comisión Mixta con Registro Patronal IMSS, las 4 actas de recorridos trimestrales o el Diagnóstico Integral NOM-030.",
                    rango_multa: "250 a 5,000 UMAs por cada norma ($28,285 a $565,700 MXN)",
                    contexto_dof: "La NOM-030-STPS-2009 se publicó en el DOF el 22 de diciembre de 2009 y la NOM-019-STPS-2011 el 13 de abril de 2011, consolidando el sistema de autogestión bipartita establecido desde los Artículos 509 y 510 de la LFT.",
                    de_que_trata: "La NOM-019 exige constituir una Comisión bipartita (Coordinador, Secretario y Vocales) que realiza recorridos trimestrales e investiga accidentes; la NOM-030 exige designar formalmente a un Responsable de Seguridad y Salud en el Trabajo (SST) y elaborar el Diagnóstico y Programa Anual.",
                    beneficio_productividad: "Involucra directamente a los trabajadores en el cuidado de las instalaciones, detecta fallas en equipos antes de que provoquen paros operativos y acredita el cumplimiento patronal ante cualquier inspección o demanda.",
                    pasos_cumplimiento: &[
                        "Firmar el Acta Constitutiva de la Comisión Mixta con todos los requisitos del Numeral 7.4 (Razón Social, RFC, Registro Patronal IMSS, domicilio, giro SCIAN, plantilla y turnos).",
                        "Designar por escrito al Responsable SST conforme al Numeral 4.1 de la NOM-030 y actualizar anualmente el Programa Preventivo.",
                        "Levantar y archivar cada 3 meses el Acta de Recorrido de Verificación de la Comisión Mixta."
                    ],
                    entregable_kit: "03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.pdf y Certificados",
                },
                NormaEducativa {
                    codigo: "NOM-017-STPS-2008, NOM-018-STPS-2015, NOM-026-STPS-2008 y NOM-028-STPS-2012",
                    titulo: "Organización: Equipo de Protección Personal (EPP), Sistema SGA Químico, Señalética y Procesos Críticos",
                    incumplimiento_pct: 55,
                    fuente_estadistica: "55% de las empresas entregan uniforme o equipo de protección sin haber elaborado la Matriz Técnica por Puesto (NOM-017) o utilizan productos químicos de limpieza/producción sin Hojas de Datos de Seguridad bajo el Sistema Globalmente Armonizado (NOM-018).",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "NOM-026 (DOF 25-nov-2008), NOM-017 (DOF 9-dic-2008), NOM-028 (DOF 6-sep-2012) y NOM-018-STPS-2015 (DOF 9-oct-2015, que sustituyó el antiguo rombo NFPA por los 9 pictogramas internacionales SGA de Naciones Unidas).",
                    de_que_trata: "Regula la determinación y entrega firmada de EPP por región anatómica (NOM-017); la comunicación de peligros por sustancias químicas con Hojas de Datos de Seguridad de 16 secciones (NOM-018); los colores y señales de seguridad e identificación de tuberías (NOM-026); y los procesos químicos mayores (NOM-028).",
                    beneficio_productividad: "Reduce drásticamente la gravedad de cualquier incidente laboral, ordena visualmente el flujo de trabajo en piso y exime de responsabilidad al patrón cuando el trabajador cuenta con acuse firmado de entrega y uso de EPP.",
                    pasos_cumplimiento: &[
                        "Elaborar la Matriz de EPP por puesto de trabajo (NOM-017) y conservar acuses de entrega con firma autógrafa de cada empleado.",
                        "Etiquetar todo envase o recipiente de productos químicos (incluyendo desengrasantes, solventes y sanitizantes) con pictogramas SGA y mantener sus HDS en español (NOM-018).",
                        "Señalizar extintores, rutas de evacuación, riesgos eléctricos y tuberías conforme a las formas y colores de la NOM-026."
                    ],
                    entregable_kit: "Cédulas de Organización (NOM-017, 018, 026 y 028) en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-001, NOM-002, NOM-004, NOM-005, NOM-006, NOM-009, NOM-020, NOM-022, NOM-027, NOM-029, NOM-033 y NOM-034",
                    titulo: "Las 12 NOMs de Seguridad Física, Incendios, Maquinaria, Electricidad y Trabajos Peligrosos",
                    incumplimiento_pct: 59,
                    fuente_estadistica: "59% de las visitas de inspección en materia de Condiciones Generales de Seguridad detectan falta de Clasificación de Grado de Riesgo de Incendio (NOM-002), ausencia de Diagrama Unifilar eléctrico (NOM-029) o falta de expediente de compresores/calderas (NOM-020).",
                    rango_multa: "250 a 5,000 UMAs por cada NOM incumplida ($28,285 a $565,700 MXN)",
                    contexto_dof: "Catálogo de Seguridad Industrial publicado en el DOF: NOM-001 (Edificios, 2008), NOM-002 (Incendios, 2010), NOM-004 (Maquinaria y bloqueo LOTO, 1999), NOM-005 (Sustancias químicas, 1998), NOM-006 (Almacenamiento y montacargas, 2014), NOM-009 (Trabajos en altura >1.80 m, 2011), NOM-020 (Recipientes sujetos a presión y calderas, 2011), NOM-022 (Electricidad estática y tierras físicas, 2015), NOM-027 (Soldadura y corte, 2008), NOM-029 (Mantenimiento eléctrico, 2011), NOM-033 (Espacios confinados, 2015) y NOM-034 (Trabajadores con discapacidad, 2016).",
                    de_que_trata: "Establecen los estándares físicos de infraestructura, protección contra incendios, guardas y candadeo en maquinaria, estiba en almacenes, permisos de trabajo peligroso (altura, corte/soldadura, espacios confinados), pararrayos/tierras físicas y accesibilidad.",
                    beneficio_productividad: "Previene incendios, cortocircuitos y paros de maquinaria que pueden destruir el patrimonio de la empresa, además de ser requisito indispensable para que las aseguradoras paguen pólizas de daños en caso de siniestro.",
                    pasos_cumplimiento: &[
                        "Contar siempre con los 4 documentos universales de seguridad física: (1) Verificación anual de instalaciones NOM-001; (2) Estudio de Grado de Riesgo de Incendio y bitácora de extintores NOM-002; (3) Diagrama Unifilar y cuadro de cargas NOM-029; y (4) Medición anual de resistencia de red de puesta a tierra NOM-022.",
                        "Si cuenta con compresores, hidroneumáticos o calderas (NOM-020), maquinaria (NOM-004), montacargas (NOM-006) o trabajos en altura/soldadura/espacios confinados (NOM-009/027/033), emitir permisos firmados por turno y certificar al personal con constancias DC-3."
                    ],
                    entregable_kit: "Matriz Completa de las 12 NOMs de Seguridad en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-010, NOM-011, NOM-012, NOM-013, NOM-014, NOM-015, NOM-024 y NOM-025",
                    titulo: "Las 8 NOMs de Salud e Higiene Industrial (Iluminación, Ruido, Temperaturas, Vibraciones y Radiaciones)",
                    incumplimiento_pct: 61,
                    fuente_estadistica: "61% de los establecimientos inspeccionados omiten el estudio de niveles de iluminación en luxes (NOM-025) o los estudios de ruido >85 dB(A) (NOM-011) y temperaturas elevadas/abatidas en cocinas, hornos o cámaras frías (NOM-015).",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Publicadas en el DOF para prevenir enfermedades de trabajo: NOM-010 (Agentes químicos contaminantes, 2014), NOM-011 (Ruido, 2001), NOM-012 (Radiaciones ionizantes/Rayos X, 2012), NOM-013 (Radiaciones no ionizantes, 1993), NOM-014 (Presiones ambientales anormales, 2000), NOM-015 (Condiciones térmicas elevadas o abatidas, 2001), NOM-024 (Vibraciones, 2001) y NOM-025 (Iluminación, 2008).",
                    de_que_trata: "Obligan a reconocer, evaluar mediante laboratorios acreditados ante la EMA (Entidad Mexicana de Acreditación) y controlar los agentes físicos y químicos presentes en el ambiente laboral para que no rebasen los Límites Máximos Permisibles de Exposición (LMPE).",
                    beneficio_productividad: "Un centro de trabajo bien iluminado (NOM-025), ventilado y térmicamente regulado (NOM-015) reduce hasta en un 28% los errores operativos, merma de insumos y fatiga visual, además de blindar a la empresa contra pensiones vitalicias por hipoacusia o enfermedades pulmonares.",
                    pasos_cumplimiento: &[
                        "Realizar cada 2 años la evaluación de niveles de iluminación en luxes (NOM-025) en oficinas, áreas de atención, cocinas, talleres y almacenes.",
                        "En cocinas industriales, panaderías, hornos o cámaras de refrigeración/congelación: evaluar el estrés térmico conforme a la NOM-015 y dar pausas de recuperación e hidratación.",
                        "En áreas con ruido igual o superior a 80 dB(A): implementar el Programa de Conservación de la Audición y dotación de protección auditiva NOM-011."
                    ],
                    entregable_kit: "Evaluación de las 8 NOMs de Salud e Higiene Industrial en Dictamen",
                },
                NormaEducativa {
                    codigo: "NOM-003, NOM-007, NOM-008, NOM-016, NOM-023, NOM-031 y NOM-032",
                    titulo: "Las 7 NOMs Específicas de Sector (Construcción NOM-031, Agrícola, Forestal, Minas y Ferrocarriles)",
                    incumplimiento_pct: 65,
                    fuente_estadistica: "65% de las obras de construcción y remodelación inspeccionadas bajo la NOM-031-STPS-2011 carecen del Análisis de Riesgos Potenciales por fase de obra y del Plan de Atención a Emergencias.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN) y suspensión inmediata de trabajos",
                    contexto_dof: "Emitidas para ramas de alta especialidad: NOM-003 (Insumos fitosanitarios/plaguicidas, 1999), NOM-007 (Maquinaria agrícola, 2000), NOM-008 (Aprovechamiento forestal y aserraderos, 2013), NOM-016 (Ferrocarriles, 2001), NOM-023 (Minas subterráneas y a cielo abierto, 2012), NOM-031 (Construcción y obras civiles, 2011) y NOM-032 (Minas de carbón, 2008).",
                    de_que_trata: "Activan protocolos especializados de seguridad industrial según los primeros dígitos del código SCIAN de la empresa (Construcción SCIAN 23, Agricultura/Forestal SCIAN 11, Minería SCIAN 21 y Transporte Ferroviario SCIAN 482).",
                    beneficio_productividad: "Evita la clausura o suspensión de obras y proyectos críticos, garantiza entregas en tiempo a clientes institucionales y protege a la dirección contra responsabilidades civiles y penales por accidentes en obra.",
                    pasos_cumplimiento: &[
                        "En obras de construcción o remodelación (NOM-031-STPS-2011): clasificar el tamaño de la obra (Pequeña, Mediana o Grande), integrar el Programa de Seguridad y Salud en la Obra y exigir permisos firmados para excavaciones, demoliciones, andamios y grúas.",
                        "Verificar en la sección SCIAN de su Dictamen Ejecutivo cuál de las 7 normas específicas resulta aplicable a su giro."
                    ],
                    entregable_kit: "Cruce Automático de las 7 NOMs Específicas por Código SCIAN en Dictamen",
                },
            ],
        ),
        "sat" => (
            "SAT • CÓDIGO FISCAL DE LA FEDERACIÓN (CFF), LISR Y LIVA",
            "Servicio de Administración Tributaria (Materialidad, Buzón, CFDI 4.0, CSD y Plataformas)",
            "Criterios de la SCJN, porcentajes reales de rechazo en auditoría, blindaje de flujo de efectivo y guía paso a paso por artículo.",
            "A partir de la Reforma Hacendaria de 2014 y las reformas de 2020-2024, el SAT fiscaliza mediante algoritmos que cruzan en tiempo real los CFDI 4.0 contra listas negras (Art. 69 y 69-B), declaraciones provisionales, Buzón Tributario y plataformas digitales. Contar únicamente con la factura XML y la transferencia bancaria ya no acredita la deducción.",
            "emerald",
            "border-emerald-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 69-B y Artículo 69 del CFF • Jurisprudencia SCJN 2a./J. 161/2019 (10a.)",
                    titulo: "Materialidad y Sustancia Económica (EFOS/EDOS), Fecha Cierta y Listas Negras",
                    incumplimiento_pct: 81,
                    fuente_estadistica: "81% de las deducciones por servicios intangibles, consultoría, marketing, mantenimiento o proveeduría revisadas en auditorías del SAT son rechazadas porque el contribuyente solo exhibe el CFDI y un contrato simple sin Fecha Cierta ni entregables.",
                    rango_multa: "Rechazo del 100% de la deducción (30% ISR + 16% IVA) + Multa del 55% al 75% (Art. 76 CFF)",
                    contexto_dof: "El Artículo 69-B fue publicado en el DOF el 9 de diciembre de 2013 (en vigor el 1 de enero de 2014). El 6 de diciembre de 2019, la Segunda Sala de la Suprema Corte de Justicia de la Nación publicó la Jurisprudencia obligatoria 2a./J. 161/2019 (10a.), determinando que todo contrato privado exhibido ante el SAT debe contar con Fecha Cierta para acreditar que no fue antedatado.",
                    de_que_trata: "Faculta al SAT para publicar en el DOF y en su portal a quienes emiten comprobantes sin contar con activos, personal o infraestructura (Art. 69-B) y a los contribuyentes con créditos firmes o no localizados (Art. 69). Quien dedujo esos CFDI (EDO) tiene 30 días para acreditar la materialidad real de la operación.",
                    beneficio_productividad: "Blinda el 100% de las deducciones de ISR y el acreditamiento de IVA de su empresa, acelera las devoluciones de saldos a favor y protege el patrimonio personal de socios y representantes legales.",
                    pasos_cumplimiento: &[
                        "Celebrar Contrato Marco de Prestación de Servicios con cláusulas de entregables y dotarlo de Fecha Cierta Digital inmutable (sello criptográfico SHA-256 / Merkle conforme al Código de Comercio y NOM-151) en la fecha de su firma.",
                        "Integrar por cada operación una Bitácora de Materialidad con: (1) Cotización y Orden de Compra; (2) Entregable verificable (reportes, planos, bitácoras, memoria fotográfica); (3) Opinión 32-D positiva del proveedor; y (4) CFDI 4.0 y REP.",
                        "Cruzar mensualmente el RFC de todos sus proveedores contra los listados completos del Art. 69 y Art. 69-B del SAT."
                    ],
                    entregable_kit: "05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.pdf y Certificado SAT-CFF69B",
                },
                NormaEducativa {
                    codigo: "Artículo 5-A del Código Fiscal de la Federación",
                    titulo: "Cláusula General Anti-Abuso: Razón de Negocios y Beneficio Económico Cuantificable",
                    incumplimiento_pct: 64,
                    fuente_estadistica: "64% de las empresas auditadas por gastos corporativos, regalías, comisiones o servicios especializados carecen de soporte documental previo que justifique la razón de negocios de la operación.",
                    rango_multa: "Recaracterización del acto jurídico y desconocimiento del beneficio fiscal de la deducción",
                    contexto_dof: "Publicado en el DOF el 9 de diciembre de 2019 (en vigor desde el 1 de enero de 2020), incorporando al derecho fiscal mexicano los estándares de la Acción 6 del proyecto BEPS de la OCDE.",
                    de_que_trata: "Establece que los actos jurídicos que carezcan de una razón de negocios y que generen un beneficio fiscal directo o indirecto tendrán los efectos fiscales que correspondan a los que se habrían realizado para la obtención del beneficio económico razonablemente esperado.",
                    beneficio_productividad: "Fortalece el gobierno corporativo al documentar el retorno de inversión (ROI) real de cada proyecto estratégico, facilitando auditorías financieras y obtención de créditos bancarios.",
                    pasos_cumplimiento: &[
                        "Estipular expresamente en las declaraciones y cláusulas de cada contrato el objetivo comercial, operativo o productivo que persigue la empresa al contratar el servicio.",
                        "Archivar junto con el contrato las cotizaciones comparativas, estudios o proyecciones de retorno de inversión que motivaron la contratación."
                    ],
                    entregable_kit: "Cláusula Segunda de Razón de Negocios en 05_Contrato_Marco_Materialidad_69B.pdf",
                },
                NormaEducativa {
                    codigo: "Artículo 17-K, Artículo 86-C y Artículo 86-D del CFF",
                    titulo: "Buzón Tributario Obligatorio y Medios de Contacto Actualizados",
                    incumplimiento_pct: 39,
                    fuente_estadistica: "39% de los contribuyentes presentan medios de contacto desactualizados (correos de ex-contadores o teléfonos dados de baja), ocasionando que los actos de fiscalización se notifiquen legalmente por estrados sin que la empresa se entere.",
                    rango_multa: "$3,850 a $11,540 MXN de multa directa (Art. 86-D CFF) + Restricción de CSD (Art. 17-H Bis)",
                    contexto_dof: "Incorporado en la Reforma Fiscal publicada en el DOF el 9 de diciembre de 2013 y vuelto obligatorio con sanción pecuniaria directa y causal de restricción de sellos en la reforma del 9 de diciembre de 2019.",
                    de_que_trata: "Obliga a todos los contribuyentes inscritos en el RFC a habilitar su Buzón Tributario y registrar y mantener actualizados al menos un correo electrónico (hasta cinco) y un número de teléfono móvil nacional.",
                    beneficio_productividad: "Garantiza que la dirección general conozca al instante cualquier requerimiento o invitación de la autoridad, evitando créditos fiscales firmes por falta de contestación oportuna.",
                    pasos_cumplimiento: &[
                        "Ingresar al Buzón Tributario con e.firma, verificar que aparezca el acuse de registro tanto del correo electrónico institucional de la dirección como del teléfono móvil, y atender cualquier aviso dentro de los 3 días hábiles siguientes."
                    ],
                    entregable_kit: "Verificación de Obligaciones Art. 17-K en Dictamen Pericial Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Artículos 29 y 29-A del CFF • Anexo 20 CFDI 4.0, Carta Porte 3.1 y Complemento de Pago (REP)",
                    titulo: "Facturación Electrónica CFDI 4.0, Complemento Carta Porte 3.1, REP y Nómina Digital",
                    incumplimiento_pct: 49,
                    fuente_estadistica: "49% de las empresas incurren en emisión extemporánea del Complemento de Recepción de Pagos (REP) después del día 5 del mes siguiente, o movilizan mercancías sin el Complemento Carta Porte 3.1.",
                    rango_multa: "$400 a $600 MXN por CFDI; $19,700 a $112,650 MXN por omisión de Carta Porte + No deducibilidad",
                    contexto_dof: "La versión 4.0 del Anexo 20 entró en vigor como única válida el 1 de abril de 2023, el plazo de 5 días naturales para el REP inició en 2022 y la versión 3.1 del Complemento Carta Porte cobró vigencia el 17 de julio de 2024.",
                    de_que_trata: "Exige validar en todo CFDI 4.0 el RFC, Nombre/Razón Social sin régimen societario, Código Postal y Régimen Fiscal del receptor; emitir el REP en operaciones en parcialidades (PPD) a más tardar el quinto día natural del mes siguiente al pago; y emitir Carta Porte 3.1 antes de trasladar bienes por tramos federales.",
                    beneficio_productividad: "Agiliza el cobro de facturas ante clientes corporativos sin rechazos administrativos, asegura la deducibilidad de toda la nómina y evita el decomiso o retención de mercancías en carretera.",
                    pasos_cumplimiento: &[
                        "Solicitar Constancia de Situación Fiscal actualizada a clientes y proveedores para timbrar con exactitud el Código Postal y Uso del CFDI.",
                        "Bloquear pagos a proveedores que no hayan entregado el Complemento de Recepción de Pagos (REP) de las facturas PPD del mes anterior antes del día 5.",
                        "Emitir CFDI de Traslado o de Ingreso con Complemento Carta Porte 3.1 antes de que cualquier vehículo de carga salga a carretera federal."
                    ],
                    entregable_kit: "Motor de Facturación CFDI 4.0 Anexo 20 y Protocolo Carta Porte en Expediente",
                },
                NormaEducativa {
                    codigo: "Artículo 28 del CFF y Artículos 32-B Ter a 32-B Quinquies • Anexo 24",
                    titulo: "Contabilidad Electrónica, Envío de Balanza de Comprobación y Beneficiario Controlador",
                    incumplimiento_pct: 74,
                    fuente_estadistica: "74% de las sociedades mercantiles presentan rezagos en el envío mensual de sus archivos XML de Catálogo de Cuentas y Balanza de Comprobación o carecen del expediente de Beneficiario Controlador que forma parte de la contabilidad.",
                    rango_multa: "$7,110 a $21,310 MXN por mes omitido; y $1,500,000 a $2,000,000 MXN por Beneficiario Controlador",
                    contexto_dof: "La Contabilidad Electrónica se incorporó en la Reforma Fiscal de 2014 (avalada por la Segunda Sala de la SCJN en julio de 2016) y el expediente de Beneficiario Controlador se integró al Art. 28 y 32-B Ter del CFF el 12 de noviembre de 2021.",
                    de_que_trata: "Obliga a llevar los registros contables en medios electrónicos conforme al Anexo 24, enviar mensualmente la Balanza de Comprobación por el Buzón Tributario y conservar como parte de la contabilidad los 22 datos fidedignos de cada Beneficiario Controlador.",
                    beneficio_productividad: "Permite tomar decisiones financieras con estados contables al día, facilita la aprobación inmediata de líneas de crédito bancarias y prepara a la empresa para auditorías o inversión externa.",
                    pasos_cumplimiento: &[
                        "Vincular cada póliza contable electrónica con el folio fiscal (UUID) de 36 caracteres del CFDI 4.0 que ampara la operación.",
                        "Mantener al día los acuses mensuales de envío de Balanza de Comprobación junto con el expediente firmado de Beneficiarios Controladores."
                    ],
                    entregable_kit: "Protocolo de Contabilidad Electrónica Art. 28 y Beneficiario Controlador en Dictamen",
                },
                NormaEducativa {
                    codigo: "Artículo 27, Artículo 17-H Bis, Artículo 32-D CFF y Artículos 113-A LISR / 18-J LIVA",
                    titulo: "Domicilio Fiscal Localizado, Opinión 32-D, Sellos Digitales (CSD) y Plataformas Digitales",
                    incumplimiento_pct: 42,
                    fuente_estadistica: "42% de las restricciones temporales de Sellos Digitales (Art. 17-H Bis) ocurren por aparecer el domicilio fiscal como No Localizado o por discrepancias automáticas entre los CFDI emitidos y los ingresos declarados.",
                    rango_multa: "Restricción inmediata del Certificado de Sello Digital (parálisis de facturación) + Multa de $4,480 a $13,430 MXN",
                    contexto_dof: "El Artículo 27 del CFF fue reestructurado el 9 de diciembre de 2019 junto con el procedimiento del Art. 17-H Bis y el régimen de Plataformas Digitales (Art. 113-A LISR).",
                    de_que_trata: "Obliga a mantener el domicilio fiscal en estatus de Localizado, presentar el Aviso de Socios y Accionistas en el RFC, mantener la Opinión 32-D en sentido Positivo y conciliar las retenciones de comercio electrónico.",
                    beneficio_productividad: "Asegura que la empresa jamás sufra una suspensión sorpresiva de su facturación y reduce la retención de ISR del 20% al 1% en ventas por plataformas digitales.",
                    pasos_cumplimiento: &[
                        "Verificar en la Constancia de Situación Fiscal que el estatus del domicilio diga expresamente Localizado.",
                        "Presentar el Aviso de Actualización de Socios o Accionistas dentro de los 30 días hábiles siguientes a cualquier modificación accionaria."
                    ],
                    entregable_kit: "Diagnóstico de Estatus RFC, CSD y Opinión 32-D en Dictamen Ejecutivo",
                },
            ],
        ),
        "imss" => (
            "IMSS E INFONAVIT • LEY DEL SEGURO SOCIAL (LSS), RACERF Y LINFONAVIT",
            "Instituto Mexicano del Seguro Social e INFONAVIT (Los 6 Vectores Patronales Completos)",
            "Conciliación de nómina vs. SUA, ahorro en Prima de Riesgo, blindaje REPSE y prevención de créditos fiscales.",
            "Regida por la Ley del Seguro Social vigente desde julio de 1997 y la reforma de subcontratación de abril de 2021, la seguridad social mexicana dota al IMSS y al INFONAVIT de facultades de Organismos Fiscales Autónomos. Hoy ambos institutos cruzan en tiempo real los XML de CFDI de Nómina 4.0 contra el SUA y el padrón REPSE.",
            "purple",
            "border-purple-600",
            vec![
                NormaEducativa {
                    codigo: "Artículos 15 y 27 a 30 de la LSS y Artículo 29 de la LINFONAVIT",
                    titulo: "Afiliación, Altas, Bajas y Conciliación de Salario Base de Cotización (SBC) Real vs. DENUE",
                    incumplimiento_pct: 57,
                    fuente_estadistica: "57% de las cartas invitación y auditorías del IMSS detectan discrepancias entre el personal ocupado físico o los conceptos timbrados en CFDI de Nómina (bonos, comisiones, premios, vales) y el Salario Base de Cotización declarado en el SUA.",
                    rango_multa: "20 a 350 UMAs + Capitales Constitutivos (Art. 77 LSS) y multa del 40% al 100% de cuotas omitidas",
                    contexto_dof: "Regulado en los Artículos 15 y 27 de la LSS (DOF 21-dic-1995) y el Artículo 29 de la Ley del INFONAVIT (DOF 24-abr-1972). El cruce algorítmico entre los CFDI de Nómina del SAT y el SUA del IMSS opera de forma automatizada en todo el país.",
                    de_que_trata: "Obliga a todo patrón a inscribir a sus trabajadores en el IMSS (preferentemente el día hábil anterior al inicio de labores para evitar Capitales Constitutivos millonarios en caso de accidente el primer día) e integrar al SBC todas las percepciones salvo las excluidas expresamente por el Artículo 27.",
                    beneficio_productividad: "Brinda seguridad médica y patrimonial completa a los trabajadores y sus familias, fideliza a la plantilla y elimina el riesgo de que un accidente de un empleado no afiliado genere un Capital Constitutivo que comprometa a la empresa.",
                    pasos_cumplimiento: &[
                        "Dar de alta en IDSE a todo trabajador un día hábil antes de su ingreso físico al centro de trabajo.",
                        "Conciliar cada mes el visor de nómina del SAT contra las cédulas EMA/EBA del SUA, vigilando que premios de puntualidad/asistencia no rebasen el 10% del SBC y vales de despensa no superen el 40% de la UMA.",
                        "Presentar los avisos bimestrales de modificación salarial por percepciones variables en los primeros 5 días hábiles de enero, marzo, mayo, julio, septiembre y noviembre."
                    ],
                    entregable_kit: "06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.pdf y Cédula SBC",
                },
                NormaEducativa {
                    codigo: "Artículo 15-A LSS • Artículos 12 a 15 LFT • Art. 29 Bis LINFONAVIT (ICSOE y SISSUB)",
                    titulo: "Reforma de Subcontratación, Padrón REPSE y Reportes Cuatrimestrales ICSOE / SISSUB",
                    incumplimiento_pct: 38,
                    fuente_estadistica: "38% de las empresas con registro REPSE o que contratan servicios especializados omiten o presentan fuera de plazo las informativas cuatrimestrales ICSOE (IMSS) y SISSUB (INFONAVIT) del día 17 de enero, mayo y septiembre.",
                    rango_multa: "500 a 2,000 UMAs por informativa ICSOE omitida; y 2,000 a 50,000 UMAs ($226,280 a $5,657,000 MXN — Art. 1004-C LFT)",
                    contexto_dof: "Publicado en el DOF el 23 de abril de 2021 mediante la reforma a 8 leyes federales que prohibió la subcontratación de personal y creó el registro público REPSE de la STPS con renovación obligatoria cada 3 años.",
                    de_que_trata: "Permite únicamente la subcontratación de servicios u obras especializadas que no formen parte del objeto social ni de la actividad económica preponderante de la beneficiaria, obligando al prestador a presentar cada cuatrimestre los informes en ICSOE y SISSUB.",
                    beneficio_productividad: "Convierte a la empresa en un proveedor elegible para grandes corporativos e industrias que exigen REPSE vigente, asegurando el cobro ágil de facturas y la deducibilidad total para sus clientes.",
                    pasos_cumplimiento: &[
                        "Si presta servicios especializados: mantener vigente su folio REPSE y presentar a más tardar el día 17 de enero, mayo y septiembre las informativas ICSOE y SISSUB (incluso informativa Sin Información).",
                        "Si contrata servicios especializados: recabar mensualmente del contratista los 7 comprobantes del Art. 27 Fracc. V LISR para garantizar la deducción de ISR y el acreditamiento de IVA."
                    ],
                    entregable_kit: "Checklist Mensual de 7 Comprobantes REPSE en 06_Protocolo_IMSS_REPSE_32D.pdf",
                },
                NormaEducativa {
                    codigo: "Artículos 71, 72, 73, 74 y 75 de la LSS y Artículos 18 a 38 y 196 del RACERF",
                    titulo: "Clasificación de Empresas y Revisión Anual de la Prima del Seguro de Riesgos de Trabajo",
                    incumplimiento_pct: 44,
                    fuente_estadistica: "44% de los patrones que sufrieron accidentes o enfermedades de trabajo omiten presentar la Determinación Anual de la Prima en febrero o excluyen indebidamente recaídas y formatos ST-7/ST-2.",
                    rango_multa: "Rectificación de oficio de Clase o Prima con cobro retroactivo y multa de 20 a 210 UMAs",
                    contexto_dof: "Establecido en el Capítulo III del Título Segundo de la LSS de 1997 y en el Catálogo de Actividades del Artículo 196 del RACERF (DOF 1-nov-2002).",
                    de_que_trata: "Obliga a autoclasificarse en una de las 5 Clases de Riesgo (Clase I: 0.54355% a Clase V: 7.58875%) según la actividad y maquinaria, y a recalcular cada mes de febrero la prima anual conforme a la siniestralidad del año anterior.",
                    beneficio_productividad: "Mantener cero accidentes laborales permite reducir legalmente la Prima de Riesgo hasta un 1% cada mes de febrero, generando un ahorro financiero directo y acumulativo sobre toda la nómina.",
                    pasos_cumplimiento: &[
                        "Integrar el expediente completo de riesgos de trabajo ocurridos del 1 de enero al 31 de diciembre (formatos ST-7, ST-2, ST-3 y ST-9).",
                        "Presentar durante el mes de febrero la Determinación Anual de la Prima de Riesgos de Trabajo vía IDSE."
                    ],
                    entregable_kit: "Certificado_Cumplimiento_IMSS32D.pdf y Protocolo de Prima de Riesgo",
                },
                NormaEducativa {
                    codigo: "Artículo 39 de la LSS y Artículo 32-D del CFF • Emisiones EMA y EBA",
                    titulo: "Pago Oportuno de Cuotas (EMA/EBA) y Opinión de Cumplimiento 32-D Positiva en Seguridad Social",
                    incumplimiento_pct: 35,
                    fuente_estadistica: "35% de las empresas proveedoras ven retenidos sus pagos o quedan descalificadas en licitaciones por aparecer su Opinión 32-D del IMSS como Negativa debido a diferencias menores en las cédulas EMA o EBA.",
                    rango_multa: "Recargos moratorios, actualización, multa del 40% al 100% del crédito (Art. 304 LSS) y bloqueo comercial",
                    contexto_dof: "Artículo 39 de la LSS (DOF 21-dic-1995), Reglas de Opinión de Cumplimiento (DOF 27-abr-2022) y Buzón IMSS obligatorio (DOF 5-ago-2024).",
                    de_que_trata: "Las cuotas obrero-patronales deben pagarse a más tardar el día 17 del mes inmediato siguiente. Para obtener Opinión 32-D Positiva se requiere no tener créditos firmes ni diferencias en los últimos 4 periodos.",
                    beneficio_productividad: "Acredita solvencia moral y fiscal inmediata ante bancos, licitaciones gubernamentales y cadenas de suministro, evitando retenciones de pagos.",
                    pasos_cumplimiento: &[
                        "Descargar el día 1 de cada mes las emisiones anticipadas EMA (mensual) y EBA (bimestral) desde IDSE para confrontarlas contra su archivo SUA antes de pagar el día 17.",
                        "Mantener habilitado el Buzón IMSS y descargar el día 18 las 3 opiniones positivas (SAT, IMSS e INFONAVIT)."
                    ],
                    entregable_kit: "Protocolo de Confronta EMA/EBA y Opinión 32-D en Kit 06",
                },
                NormaEducativa {
                    codigo: "Artículo 15-A párrafo tercero y Artículo 15 Fracción VI de la LSS (Responsabilidad Solidaria y SIROC)",
                    titulo: "Retención Documental, Responsabilidad Solidaria en Servicios y Registro de Obras SIROC",
                    incumplimiento_pct: 63,
                    fuente_estadistica: "63% de los propietarios o empresas que contratan obras de construcción, remodelación o mantenimiento en sus instalaciones no exigen el registro SIROC ni las constancias de pago SUA del contratista.",
                    rango_multa: "Exigibilidad del 100% de las cuotas obrero-patronales omitidas por el contratista directamente contra el cliente",
                    contexto_dof: "Regulado en la reforma del 23 de abril de 2021 (Art. 15-A LSS) y en el Acuerdo del sistema SIROC publicado en el DOF el 13 de julio de 2017.",
                    de_que_trata: "Hace al cliente o propietario del inmueble responsable solidario del pago de las cuotas al IMSS de los trabajadores utilizados por sus contratistas cuando éstos incumplen sus obligaciones patronales.",
                    beneficio_productividad: "Protege el presupuesto de cualquier remodelación, ampliación u operación tercerizada, impidiendo que deudas del contratista se transfieran a su empresa.",
                    pasos_cumplimiento: &[
                        "Establecer como condición contractual para liberar cualquier pago de obra o servicio especializado la entrega previa de la Opinión 32-D positiva del IMSS/INFONAVIT, el pago SUA del mes y, en caso de obra civil o remodelación, el Acuse de Registro SIROC."
                    ],
                    entregable_kit: "Cláusulas de Indemnidad y Responsabilidad Solidaria en Kit 05 y Kit 06",
                },
                NormaEducativa {
                    codigo: "Artículos 304, 304-A y 304-B de la Ley del Seguro Social",
                    titulo: "Catálogo de las 22 Infracciones Directas y Sanciones Administrativas de la LSS",
                    incumplimiento_pct: 48,
                    fuente_estadistica: "48% de las multas impuestas bajo el Artículo 304-A derivan de la presentación extemporánea de avisos afiliatorios o de no conservar la documentación patronal durante el plazo de 5 años.",
                    rango_multa: "20 a 350 UMAs ($2,262 a $39,599 MXN) en supuestos ordinarios y hasta 2,000 UMAs ($226,280 MXN) en REPSE",
                    contexto_dof: "Adicionado al Título Sexto de la LSS mediante decreto publicado en el DOF el 20 de diciembre de 2001 y ampliado en la reforma de abril de 2021.",
                    de_que_trata: "Tipifica las 22 conductas infractoras de los patrones ante el IMSS y prevé en el Artículo 304-C la no imposición de multas cuando las obligaciones se cumplen de manera espontánea antes de una notificación.",
                    beneficio_productividad: "Aplicar el beneficio de cumplimiento espontáneo del Artículo 304-C elimina el 100% de las multas por avisos extemporáneos antes de que llegue un requerimiento.",
                    pasos_cumplimiento: &[
                        "Realizar auditoría preventiva trimestral de avisos IDSE para corregir de forma espontánea (sin multa conforme al Art. 304-C LSS) cualquier omisión antes de que medie requerimiento en el Buzón IMSS."
                    ],
                    entregable_kit: "Diagnóstico Integral de Riesgo Art. 304-A LSS en Dictamen Ejecutivo",
                },
            ],
        ),
        "inai" => (
            "SABG / EX-INAI • LEY FEDERAL DE PROTECCIÓN DE DATOS PERSONALES (LFPDPPP)",
            "Autoridad Garante Federal (SABG) • Los 6 Pilares de Privacidad, ARCO y Biométricos",
            "Supervisión de la SABG, porcentajes reales de sanción, confianza digital del cliente y guía paso a paso por artículo.",
            "Publicada en el DOF el 5 de julio de 2010 y su Reglamento el 21 de diciembre de 2011, la LFPDPPP obliga a toda persona física o moral privada que trate datos de clientes, empleados o proveedores. Tras la reforma constitucional de diciembre de 2024 y la legislación secundaria de 2025, la autoridad verificadora y sancionadora es la Secretaría Anticorrupción y Buen Gobierno (SABG), manteniéndose vigentes todas las multas de 100 a 320,000 UMAs (que se duplican cuando involucran datos sensibles o biométricos).",
            "cyan",
            "border-cyan-600",
            vec![
                NormaEducativa {
                    codigo: "Artículos 8, 15 y 16 de la LFPDPPP y Lineamientos del Aviso de Privacidad (DOF 17/01/2013)",
                    titulo: "Aviso de Privacidad en sus 3 Modalidades: Integral, Simplificado y Corto",
                    incumplimiento_pct: 73,
                    fuente_estadistica: "73% de las empresas verificadas tienen un texto genérico en su página web que omite alguno de los 6 requisitos del Artículo 16 o no ofrece el mecanismo de 5 días para negar finalidades secundarias de mercadotecnia.",
                    rango_multa: "100 a 160,000 UMAs ($11,314 a $18,102,400 MXN — Art. 63 Fracc. II y 64 Fracc. II LFPDPPP)",
                    contexto_dof: "Establecido en la LFPDPPP (DOF 5-jul-2010) y estructurado técnicamente en los Lineamientos del Aviso de Privacidad publicados en el DOF el 17 de enero de 2013.",
                    de_que_trata: "Obliga a poner a disposición del titular el Aviso de Privacidad ANTES de recabar sus datos, utilizando el Aviso Integral en portales web y carpetas físicas, el Simplificado en formularios de contacto y el Corto en espacios reducidos como tickets o cámaras de videovigilancia (CCTV).",
                    beneficio_productividad: "Genera confianza inmediata en los visitantes de su sitio web y clientes potenciales, incrementando la conversión de prospectos comerciales y blindando el uso legal de su base de datos de marketing.",
                    pasos_cumplimiento: &[
                        "Publicar en el pie de página de su dominio web el Aviso de Privacidad Integral con los 6 requisitos del Art. 16: identidad y domicilio del responsable, datos tratados, finalidades primarias y secundarias separadas, opciones de limitación, medios ARCO y transferencias.",
                        "Colocar el Aviso de Privacidad Corto impreso a la vista en recepción y zonas con cámaras CCTV."
                    ],
                    entregable_kit: "01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.pdf y versión editable Word",
                },
                NormaEducativa {
                    codigo: "Artículos 22 al 35 de la LFPDPPP y Artículos 87 al 110 del RLFPDPPP",
                    titulo: "Mecanismos, Plazos y Procedimiento de Atención a los Derechos ARCO",
                    incumplimiento_pct: 64,
                    fuente_estadistica: "64% de los procedimientos de verificación se inician porque la empresa no responde al ciudadano dentro del plazo máximo legal de 20 días hábiles o exige requisitos adicionales no previstos en el Artículo 29.",
                    rango_multa: "200 a 320,000 UMAs ($22,628 a $36,204,800 MXN — Art. 63 Fracc. VII y 64 Fracc. III LFPDPPP)",
                    contexto_dof: "Capítulo IV de la LFPDPPP (DOF 5-jul-2010), que reglamenta en el sector privado el ejercicio de los derechos constitucionales de Acceso, Rectificación, Cancelación y Oposición del Artículo 16.",
                    de_que_trata: "Garantiza a todo titular (cliente, ex-empleado, prospecto o proveedor) el derecho de solicitar el Acceso a sus datos, su Rectificación, su Cancelación o su Oposición al tratamiento en plazos perentorios.",
                    beneficio_productividad: "Resuelve de manera interna y pacífica cualquier inconformidad de ex-empleados o clientes en menos de 20 días, evitando que acudan a presentar una denuncia formal ante la SABG.",
                    pasos_cumplimiento: &[
                        "Establecer en su Aviso de Privacidad un correo electrónico oficial activo y un domicilio físico para recibir solicitudes ARCO.",
                        "Comunicar la determinación adoptada en un plazo máximo de 20 días hábiles (Art. 32 LFPDPPP) y hacerla efectiva dentro de los 15 días hábiles siguientes."
                    ],
                    entregable_kit: "Procedimiento y Formulario Oficial de Derechos ARCO en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículos 3 Fracción VI, 9, 16 párrafo último y 64 Fracción IV de la LFPDPPP",
                    titulo: "Tratamiento de Datos Personales Sensibles (Salud, Biométricos de Asistencia y Financieros)",
                    incumplimiento_pct: 84,
                    fuente_estadistica: "84% de las empresas que utilizan huella dactilar o reconocimiento facial para control de asistencia o que resguardan certificados médicos y cuentas bancarias de nómina lo hacen sin contar con el consentimiento expreso y firma autógrafa del titular.",
                    rango_multa: "Hasta el DOBLE de las multas ordinarias: de 400 a 640,000 UMAs ($45,256 a $72,409,600 MXN)",
                    contexto_dof: "Artículo 9 de la LFPDPPP (DOF 5-jul-2010) y Guía para el Tratamiento de Datos Biométricos (2018).",
                    de_que_trata: "Exige obtener el consentimiento expreso y por escrito del titular (mediante su firma autógrafa o firma electrónica) para tratar cualquier dato sensible (estado de salud, estudios médicos, huella dactilar, iris, rostro, voz, afiliación sindical) y consentimiento expreso para datos financieros o patrimoniales (Art. 8).",
                    beneficio_productividad: "Permite utilizar relojes checadores biométricos con total validez probatoria en juicios laborales para acreditar puntualidad, asistencia o faltas injustificadas sin riesgo de nulidad.",
                    pasos_cumplimiento: &[
                        "Recabar en el expediente de cada trabajador y visitante el Formato de Consentimiento Expreso y por Escrito con firma autógrafa antes de capturar su huella dactilar, rostro o datos médicos."
                    ],
                    entregable_kit: "Acta de Consentimiento Expreso Biométrico y Patrimonial en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículos 19, 20 y 63 Fracción VIII de la LFPDPPP y Artículos 57 a 66 del RLFPDPPP",
                    titulo: "Medidas de Seguridad Técnicas, Físicas y Administrativas (Documento de Seguridad y SSL/TLS)",
                    incumplimiento_pct: 79,
                    fuente_estadistica: "79% de las organizaciones muestran un aviso de privacidad en su portal pero carecen del Documento de Seguridad interno con inventario de datos personales, análisis de brecha y cifrado en bases de datos.",
                    rango_multa: "200 a 320,000 UMAs ($22,628 a $36,204,800 MXN — Art. 63 Fracc. VIII y 64 Fracc. III LFPDPPP)",
                    contexto_dof: "Capítulo III del Reglamento de la LFPDPPP publicado en el DOF el 21 de diciembre de 2011.",
                    de_que_trata: "Obliga a mantener medidas de seguridad técnicas (cifrado TLS 1.3, control de accesos, respaldos), físicas (resguardo bajo llave de expedientes) y administrativas integradas en un Documento de Seguridad con los 7 rubros del Artículo 61 del Reglamento.",
                    beneficio_productividad: "Protege el activo más valioso de la empresa (su cartera de clientes, precios, proveedores y secretos comerciales) contra robo interno de empleados o ataques cibernéticos.",
                    pasos_cumplimiento: &[
                        "Mantener vigente el certificado SSL/TLS en todos sus dominios y subdominios web.",
                        "Elaborar y actualizar anualmente el inventario de datos personales, matriz de privilegios de usuarios y bitácora de incidentes de seguridad."
                    ],
                    entregable_kit: "Auditoría Técnica TLS/SSL y Custodia de Documento de Seguridad en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículos 36 y 37 de la LFPDPPP y Artículos 49 a 56 y 67 a 75 del RLFPDPPP",
                    titulo: "Cláusulas de Transferencia a Terceros y Contratos de Remisión con Encargados (Despachos y Nube)",
                    incumplimiento_pct: 67,
                    fuente_estadistica: "67% de las empresas envían bases de datos de nómina, clientes o facturación a despachos contables externos, agencias de marketing o proveedores de software sin haber firmado cláusulas contractuales de Encargado.",
                    rango_multa: "200 a 320,000 UMAs por transferir datos a terceros sin comunicarles el Aviso de Privacidad (Art. 63 Fracc. XII)",
                    contexto_dof: "Capítulo V de la LFPDPPP (DOF 5-jul-2010) y Capítulo II del RLFPDPPP (DOF 21-dic-2011), que regula la figura del Encargado que trata datos por cuenta del Responsable.",
                    de_que_trata: "Regula la comunicación de datos personales a terceros nacionales o extranjeros. Exige cláusula en el Aviso de Privacidad para transferencias y contratos con cláusulas de confidencialidad y seguridad para los Encargados (despachos, maquiladoras de nómina, CRM y hosting).",
                    beneficio_productividad: "Delimita legalmente la responsabilidad si un proveedor externo o despacho sufre una filtración de información, manteniendo indemne a su empresa.",
                    pasos_cumplimiento: &[
                        "Incluir en los contratos con su despacho contable, proveedor de nómina y proveedores tecnológicos las cláusulas de Encargado del Artículo 50 del Reglamento."
                    ],
                    entregable_kit: "Cláusulas de Transferencia y Remisión a Encargados en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículo 17 Fracción I y Artículo 30 de la LFPDPPP",
                    titulo: "Designación Formal del Oficial de Privacidad o Departamento de Datos Personales",
                    incumplimiento_pct: 61,
                    fuente_estadistica: "61% de las empresas inspeccionadas no cuentan con el acta interna que designa formalmente a la persona o departamento responsable de tramitar solicitudes ARCO y fomentar la protección de datos dentro de la organización.",
                    rango_multa: "100 a 160,000 UMAs por incumplir el principio de responsabilidad y gobernanza interna",
                    contexto_dof: "Establecido de manera obligatoria en el Artículo 30 de la LFPDPPP desde el 5 de julio de 2010.",
                    de_que_trata: "Todo responsable debe designar a una persona o departamento de datos personales, quien dará trámite a las solicitudes de los titulares y fomentará la protección de datos personales en el interior de la organización.",
                    beneficio_productividad: "Centraliza el control de acceso a la información confidencial en una figura responsable, profesionalizando el gobierno interno de la empresa.",
                    pasos_cumplimiento: &[
                        "Suscribir el Acta de Designación del Oficial de Privacidad e imprimir su nombre o cargo y correo oficial dentro del Aviso de Privacidad Integral."
                    ],
                    entregable_kit: "Nombramiento Formal del Oficial de Privacidad en 01_Aviso_de_Privacidad_Integral.pdf",
                },
            ],
        ),
        "pld" => (
            "LOS 9 MÓDULOS SECTORIALES DINÁMICOS Y PREVENCIÓN DE LAVADO DE DINERO (LFPIORPI)",
            "Regulación Sectorial por Giro SCIAN (UIF/CNBV, COFEPRIS, PIPC, SEMARNAT, SICT, REPSE, SSPC, SEP y SECTUR)",
            "Porcentaje real de empresas clausuradas o multadas, continuidad de negocio y guía paso a paso para los 9 Módulos Sectoriales.",
            "Además de las obligaciones generales laborales y fiscales, el motor pericial de FIRMA PERICIAL MALLEN® activa dinámicamente los 9 Módulos Sectoriales según el código SCIAN de 6 dígitos de la empresa, la superficie de sus instalaciones (>100 m²) y la naturaleza de sus operaciones financieras o comerciales.",
            "amber",
            "border-amber-600",
            vec![
                NormaEducativa {
                    codigo: "Módulo Sectorial 01 • CNBV / CONDUSEF / LFPIORPI (Artículos 17, 18, 21, 32, 53 y 54)",
                    titulo: "Fintech, Sector Financiero y las 16 Actividades Vulnerables Antilavado (SHCP / UIF / SAT)",
                    incumplimiento_pct: 69,
                    fuente_estadistica: "69% de las empresas no financieras que otorgan préstamos intercompañía (mutuos), arrendan inmuebles, desarrollan obras inmobiliarias o comercializan vehículos/activos virtuales rebasando los umbrales en UMAs omiten su alta en el portal SPPLD y los avisos del día 17.",
                    rango_multa: "10,000 a 65,000 UMAs ($1,131,400 a $7,354,100 MXN) o del 10% al 100% del valor del acto por aviso omitido",
                    contexto_dof: "La Ley Federal para la Prevención e Identificación de Operaciones con Recursos de Procedencia Ilícita (LFPIORPI) fue publicada en el DOF el 17 de octubre de 2012 (en vigor el 17 de julio de 2013), complementada por la Ley Fintech (DOF 9-mar-2018).",
                    de_que_trata: "Obliga a quienes realicen cualquiera de las 16 Actividades Vulnerables del Artículo 17 (préstamos/mutuos, construcción o venta de inmuebles, arrendamiento, vehículos, blindaje, joyería, traslado de valores, servicios profesionales independientes, fe pública, comercio exterior y activos virtuales) a identificar al cliente y Dueño Beneficiario, restringir efectivo (Art. 32) y enviar Avisos a la UIF.",
                    beneficio_productividad: "Evita el congelamiento de cuentas bancarias por alertas de la UIF, blinda los préstamos entre empresas del mismo grupo y protege la reputación financiera de los accionistas.",
                    pasos_cumplimiento: &[
                        "Darse de alta en el Portal SPPLD del SAT, designar Oficial de Cumplimiento (Art. 20) y contar con su Manual de Cumplimiento PLD.",
                        "Integrar el Expediente Único de Identificación de cada cliente o contraparte ANTES del acto y presentar los Avisos o Informes en Cero a más tardar el día 17 de cada mes."
                    ],
                    entregable_kit: "Dictamen de Actividades Vulnerables LFPIORPI en Expediente Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 02 • COFEPRIS (Arts. 198, 200 Bis y 202 LGS • NOM-251-SSA1 y NOM-051-SCFI/SSA1)",
                    titulo: "Salud, Alimentos/Restaurantes, Cosméticos, Suplementos y Farmacia (DIGIPRIS)",
                    incumplimiento_pct: 52,
                    fuente_estadistica: "52% de los restaurantes, pizzerías, cafeterías, clínicas, consultorios, farmacias y comercializadoras de alimentos/cosméticos inspeccionados presentan omisiones en su Aviso de Funcionamiento DIGIPRIS, bitácoras NOM-251 o etiquetado NOM-051.",
                    rango_multa: "Aseguramiento de mercancía, suspensión de servicios y multas de 2,000 a 16,000 UMAs ($226,280 a $1,810,240 MXN)",
                    contexto_dof: "Ley General de Salud (DOF 7-feb-1984), decreto de creación de COFEPRIS (DOF 5-jul-2001), NOM-251-SSA1-2009 para higiene en alimentos y bebidas (DOF 1-mar-2010) y NOM-051 de etiquetado frontal (DOF 27-mar-2020).",
                    de_que_trata: "Exige contar con Licencia Sanitaria o Aviso de Funcionamiento y Responsable Sanitario en DIGIPRIS antes de operar, así como cumplir las bitácoras de higiene en preparación de alimentos (NOM-251) y etiquetado frontal (NOM-051).",
                    beneficio_productividad: "Estandariza la frescura, cadena de frío e higiene en cocinas, consultorios y almacenes, reduciendo mermas por descomposición y garantizando la salud y preferencia de los clientes.",
                    pasos_cumplimiento: &[
                        "Mantener exhibido el Acuse del Aviso de Funcionamiento DIGIPRIS con la clave SCIAN exacta de la actividad del establecimiento.",
                        "En restaurantes, pizzerías y servicios de alimentos (NOM-251-SSA1-2009): llevar bitácoras firmadas de temperaturas de refrigeración (<4°C) y congelación (-18°C), desinfección de insumos, limpieza por áreas y certificado vigente de fumigación expedido por empresa con Licencia Sanitaria COFEPRIS."
                    ],
                    entregable_kit: "Módulo Sectorial COFEPRIS y Checklist NOM-251 / NOM-051 en Dictamen",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 03 • PROTECCIÓN CIVIL ESTATAL Y MUNICIPAL (Arts. 39, 40 y 74 a 78 LGPC)",
                    titulo: "Programa Interno de Protección Civil (PIPC >100 m² o Afluencia) y Dictámenes Estructural, Eléctrico y Gas",
                    incumplimiento_pct: 66,
                    fuente_estadistica: "66% de los establecimientos comerciales, restaurantes, oficinas e industrias con superficie mayor a 100 m² o afluencia de público operan con su Programa Interno de Protección Civil (PIPC) vencido o sin visto bueno anual.",
                    rango_multa: "Clausura temporal o definitiva inmediata del inmueble con colocación de sellos + Multas de 1,000 a 20,000 UMAs",
                    contexto_dof: "Sistema Nacional de Protección Civil (1986), Ley General de Protección Civil vigente publicada en el DOF el 6 de junio de 2012 y su Reglamento del 13 de mayo de 2014.",
                    de_que_trata: "Obliga a todo centro de trabajo con superficie superior a 100 m², manejo de gas/químicos o afluencia de público a contar con su PIPC vigente elaborado por Tercer Acreditado / Consultor Registrado, acompañado de dictámenes técnicos de instalaciones.",
                    beneficio_productividad: "Elimina el riesgo número uno de clausura municipal o de alcaldía en comercios y restaurantes, protege la vida de clientes y empleados y valida las pólizas de seguro contra incendio o explosión.",
                    pasos_cumplimiento: &[
                        "Integrar el Acta Constitutiva de la Unidad Interna de Protección Civil y las 4 Brigadas (Evacuación, Primeros Auxilios, Prevención y Combate de Incendios, y Comunicación).",
                        "Contar con Póliza de Seguro de Responsabilidad Civil vigente, Dictamen de Instalaciones Eléctricas (UVIE), Dictamen de Instalaciones de Gas LP o Natural (obligatorio en restaurantes y plantas) y Constancia de Seguridad Estructural (DRO)."
                    ],
                    entregable_kit: "Certificado_Seguridad_ProtCivil.pdf y Guía de Unidad Interna PIPC",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 04 • SEMARNAT / ASEA / PROFEPA (LGEEPA Arts. 109 Bis y 150 • LGPGIR Arts. 40 a 48)",
                    titulo: "Impacto Ambiental, Licencia Ambiental Única (LAU), COA y Generador de Residuos Peligrosos",
                    incumplimiento_pct: 59,
                    fuente_estadistica: "59% de los talleres, plantas, imprentas, clínicas y restaurantes generadores de residuos peligrosos o de manejo especial (aceites, grasas, solventes, estopas, RPBI) carecen de su registro de generador y manifiestos de recolección autorizada.",
                    rango_multa: "Clausura total o parcial por PROFEPA y multas de 30 a 50,000 UMAs ($3,394 a $5,657,000 MXN — Art. 171 LGEEPA)",
                    contexto_dof: "LGEEPA (DOF 28-ene-1988), LGPGIR (DOF 8-oct-2003) y Ley de la Agencia de Seguridad, Energía y Ambiente ASEA (DOF 11-ago-2014).",
                    de_que_trata: "Regula las emisiones a la atmósfera, descargas de aguas residuales y el registro y disposición de Residuos Peligrosos (Microgenerador <400 kg/año; Pequeño 400 kg a 10 ton/año; Gran Generador >10 ton/año) y Residuos Peligrosos Biológico-Infecciosos (NOM-087-SEMARNAT-SSA1-2002).",
                    beneficio_productividad: "Posiciona a la empresa dentro de los estándares internacionales ESG (Ambiental, Social y Gobernanza) exigidos por corporativos globales y evita sanciones ambientales.",
                    pasos_cumplimiento: &[
                        "Obtener el Número de Registro Ambiental (NRA) como Generador de Residuos Peligrosos o de Manejo Especial y entregar los residuos únicamente a transportistas autorizados conservando los Manifiestos sellados por 5 años."
                    ],
                    entregable_kit: "Módulo Sectorial SEMARNAT / PROFEPA en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 05 • SICT / CAPUFE (Ley de Caminos, Puentes y Autotransporte Federal y NOM-087-SCT)",
                    titulo: "Transporte Federal de Carga, Fletes, Logística, Bitácoras de Horas y Pesos/Dimensiones (NOM-012-SCT)",
                    incumplimiento_pct: 48,
                    fuente_estadistica: "48% de las unidades de carga y flotillas corporativas inspeccionadas en carreteras federales presentan incumplimientos en la Bitácora de Horas de Servicio del conductor (NOM-087-SCT-2-2017) o en la verificación físico-mecánica (NOM-068-SCT).",
                    rango_multa: "Detención y remisión de la unidad al corralón federal + Multas de hasta 500 UMAs por infracción",
                    contexto_dof: "Ley de Caminos, Puentes y Autotransporte Federal (DOF 22-dic-1993), NOM-068-SCT-2-2014 (DOF 19-ene-2015), NOM-012-SCT-2-2017 (DOF 26-dic-2017) y NOM-087-SCT-2-2017 (DOF 28-jun-2018).",
                    de_que_trata: "Obliga a contar con permiso de la SICT para autotransporte federal o transporte privado de carga, póliza de seguro de responsabilidad civil por al menos 19,000 UMAs, dictamen anual físico-mecánico y de bajas emisiones, bitácora de pausas de conducción y Carta Porte 3.1.",
                    beneficio_productividad: "Previene accidentes viales por fatiga del operador, reduce costos de mantenimiento correctivo de la flotilla y garantiza entregas puntuales sin detenciones en básculas federales.",
                    pasos_cumplimiento: &[
                        "Portar en cada vehículo de carga o reparto foráneo: Tarjeta de Circulación SICT, Póliza de Seguro vigente, Dictamen Físico-Mecánico NOM-068, Bitácora de Horas de Servicio NOM-087 y CFDI con Complemento Carta Porte 3.1."
                    ],
                    entregable_kit: "Módulo Sectorial SICT / Logística en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Módulos Sectoriales 06, 07, 08 y 09 • REPSE • SSPC (Seguridad Privada) • SEP (RVOE) • SECTUR (RNT)",
                    titulo: "Servicios Especializados REPSE, Seguridad Privada (DGSP), Educación (RVOE) y Turismo/Hospitalidad (RNT)",
                    incumplimiento_pct: 54,
                    fuente_estadistica: "54% de las empresas de seguridad privada, agencias/hoteles turísticos y centros de capacitación operan sin revalidación anual de su permiso SSPC, sin inscripción en el Registro Nacional de Turismo (RNT) o sin mención expresa del estatus de RVOE.",
                    rango_multa: "Clausura y multas de hasta 5,000 UMAs en Seguridad Privada; 7,000 UMAs en Educación; y suspensión en Turismo",
                    contexto_dof: "Ley Federal de Seguridad Privada (DOF 6-jul-2006), Ley General de Turismo (DOF 17-jun-2009), Ley General de Educación (DOF 30-sep-2019) y Reforma de Subcontratación REPSE (DOF 23-abr-2021).",
                    de_que_trata: "Regula los 4 regímenes de autorización especial de servicios: (6) Padrón REPSE e informativas ICSOE/SISSUB; (7) Autorización federal de la Dirección General de Seguridad Privada (SSPC/DGSP) y registro en el RNPSP; (8) Reconocimiento de Validez Oficial de Estudios (RVOE) ante la SEP; y (9) Registro Nacional de Turismo (RNT) para hoteles, restaurantes turísticos y agencias.",
                    beneficio_productividad: "Otorga el sello oficial de autorización federal en su sector, permitiendo firmar contratos de alto valor con clientes institucionales y plataformas internacionales.",
                    pasos_cumplimiento: &[
                        "Seguridad Privada (SSPC): revalidar anualmente la autorización federal/estatal y consultar antecedentes del personal operativo en el RNPSP.",
                        "Turismo y Hospitalidad (SECTUR): tramitar y exhibir el certificado del Registro Nacional de Turismo (RNT) vigente por 2 años y cumplir la NOM-010-TUR-2001.",
                        "Educación y Capacitación (SEP/STPS): exhibir el número de acuerdo RVOE en toda publicidad o indicar expresamente cuando los cursos sean de educación continua sin RVOE (contando con registro DC-5 ante la STPS)."
                    ],
                    entregable_kit: "Evaluación de los Módulos 06, 07, 08 y 09 por Giro SCIAN en Dictamen Ejecutivo",
                },
            ],
        ),
        _ => (
            "PROFECO Y SECRETARÍA DE ECONOMÍA • LEY FEDERAL DE PROTECCIÓN AL CONSUMIDOR (LFPC)",
            "Procuraduría Federal del Consumidor y Secretaría de Economía (Comercio Digital, RRPAC y NOM-151)",
            "Monitoreo de tiendas virtuales, estadísticas reales de sanción, reputación comercial y guía paso a paso por artículo.",
            "Regida por la Ley Federal de Protección al Consumidor (LFPC) de orden público e interés social, PROFECO supervisa tanto establecimientos físicos (restaurantes, comercios, exhibición de precios totales con IVA y prohibición de propina obligatoria) como tiendas en línea y contratos estandarizados.",
            "teal",
            "border-teal-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 76 BIS y 76 BIS 1 de la LFPC y Norma Mexicana NMX-COE-001-SCFI-2018",
                    titulo: "Comercio Electrónico, Portales Digitales y Transparencia de Precios y Contacto",
                    incumplimiento_pct: 56,
                    fuente_estadistica: "56% de los sitios web comerciales revisados por el Monitoreo de Tiendas Virtuales de PROFECO omiten publicar su domicilio físico real en territorio nacional, sus teléfonos de atención o las características detalladas del bien o servicio.",
                    rango_multa: "$701.15 a $3,995,249.26 MXN (Artículos 127 y 128 LFPC) y sellos de advertencia digital",
                    contexto_dof: "El Capítulo VIII Bis (Artículo 76 BIS) fue adicionado a la LFPC mediante decreto publicado en el DOF el 29 de mayo de 2000 y complementado el 30 de abril de 2019 con la norma mexicana de comercio electrónico NMX-COE-001-SCFI-2018.",
                    de_que_trata: "Obliga a todo proveedor que ofrezca, comercialice o venda bienes o servicios por medios electrónicos a informar su domicilio físico en México, teléfonos, correo, características reales de los productos y monto total a pagar en Moneda Nacional con IVA incluido (Art. 7 BIS).",
                    beneficio_productividad: "Aumenta la tasa de cierre de ventas en línea al brindar certeza total al comprador y evita la colocación de alertas públicas de PROFECO sobre su dominio web.",
                    pasos_cumplimiento: &[
                        "Publicar de forma visible en el pie de página de su sitio web su Razón Social, RFC, domicilio físico completo en México, teléfono fijo y correo electrónico de atención.",
                        "Mostrar siempre precios totales en Pesos Mexicanos (MXN) con IVA e impuestos incluidos. En restaurantes y servicios (Art. 7 BIS y Art. 10 LFPC): exhibir carta con precios totales a la entrada y no incluir ni cargar propina sin consentimiento expreso del cliente."
                    ],
                    entregable_kit: "07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.pdf y versión editable Word",
                },
                NormaEducativa {
                    codigo: "Artículo 76 BIS Fracciones III y VII, Artículo 56 y Artículos 77 a 82 de la LFPC",
                    titulo: "Políticas de Devolución, Cancelación, Revocación en 5 Días Hábiles y Garantía Mínima de 90 Días",
                    incumplimiento_pct: 63,
                    fuente_estadistica: "63% de los comercios y portales web exhiben leyendas de Garantía de 30 días o Sin cambios ni devoluciones, las cuales violan directamente el mínimo legal de 90 días del Artículo 77 y el derecho de revocación del Artículo 56 de la LFPC.",
                    rango_multa: "$980.78 a $3,839,580.66 MXN (Art. 128 LFPC) + reposición y bonificación mínima del 20% (Art. 92 TER)",
                    contexto_dof: "El plazo mínimo de garantía comercial fue incrementado de 60 a 90 días naturales mediante reforma al Artículo 77 publicada en el DOF el 11 de enero de 2018.",
                    de_que_trata: "Otorga 5 días hábiles para revocar sin penalización compras celebradas a distancia o fuera del local comercial (Art. 56 LFPC) y obliga a que toda póliza de garantía tenga una vigencia mínima de 90 días naturales y se redacte en español con los datos del Artículo 78.",
                    beneficio_productividad: "Disminuye hasta en un 70% las disputas bancarias (contracargos en Stripe/terminales) y las quejas en el Buró Comercial de PROFECO al resolver garantías con reglas claras.",
                    pasos_cumplimiento: &[
                        "Actualizar sus pólizas de garantía y términos web a un plazo mínimo de 90 días naturales, indicando claramente el domicilio en México donde el consumidor puede hacerla efectiva.",
                        "Publicar el procedimiento claro para devoluciones, cancelaciones, reembolsos en un máximo de 15 días y atención de reclamaciones."
                    ],
                    entregable_kit: "Póliza de Garantía de 90 Días y Procedimiento de Reembolso en Kit 07 PROFECO",
                },
                NormaEducativa {
                    codigo: "Artículos 85, 86, 86 BIS, 87 y 90 de la LFPC",
                    titulo: "Contratos de Adhesión, Registro Público (RRPAC) y Prohibición de Cláusulas Abusivas",
                    incumplimiento_pct: 72,
                    fuente_estadistica: "72% de las empresas de servicios que emplean formatos estandarizados de contratación carecen de número de registro RRPAC ante PROFECO o incluyen cláusulas unilaterales nulas conforme al Artículo 90.",
                    rango_multa: "Ineficacia jurídica del contrato no registrado (Art. 86 TER LFPC) y multas de hasta $3,839,580.66 MXN",
                    contexto_dof: "Capítulo X de la LFPC (DOF 24-dic-1992) y catálogo electrónico del Registro Público de Contratos de Adhesión (RRPAC).",
                    de_que_trata: "Define como contrato de adhesión todo documento elaborado unilateralmente por el proveedor en formatos uniformes. Cuando una NOM exige su registro previo (inmobiliarias NOM-247, eventos sociales, talleres, tiempos compartidos, telecomunicaciones), no surte efectos contra el consumidor si no ostenta su folio RRPAC.",
                    beneficio_productividad: "Asegura que los contratos firmados con sus clientes sean 100% exigibles en tribunales sin riesgo de que un juez o PROFECO declare nulas sus cláusulas de cobro o penalización.",
                    pasos_cumplimiento: &[
                        "Purgar de todos sus contratos y términos web las cláusulas prohibidas por el Artículo 90 de la LFPC (modificación unilateral de términos, liberación de responsabilidad civil del proveedor o penas convencionales no equitativas)."
                    ],
                    entregable_kit: "Certificado_Comercial_PROFECO.pdf y Términos Alineados al Art. 85-90 LFPC",
                },
                NormaEducativa {
                    codigo: "Artículos 32, 32 BIS y 128 de la Ley Federal de Protección al Consumidor",
                    titulo: "Publicidad Veraz, Comprobable y Prohibición de Prácticas Comerciales Engañosas",
                    incumplimiento_pct: 47,
                    fuente_estadistica: "47% de los procedimientos sancionadores por publicidad en redes sociales y sitios web derivan del uso de afirmaciones absolutas no comprobables técnicamente o promociones sin vigencia ni restricciones claras.",
                    rango_multa: "Orden precautoria de suspensión inmediata de la publicidad (Art. 35 LFPC) y multa de hasta $3,839,580.66 MXN",
                    contexto_dof: "Reformado mediante decretos publicados en el DOF el 4 de febrero de 2004 y el 11 de enero de 2018, y complementado en 2023 con la Guía de Publicidad para Medios Digitales de PROFECO.",
                    de_que_trata: "Establece que toda información o publicidad relativa a bienes, productos o servicios que se difunda por cualquier medio deberá ser veraz, comprobable, clara y exenta de textos o imágenes que induzcan a error o confusión.",
                    beneficio_productividad: "Protege las campañas publicitarias en redes sociales y buscadores contra órdenes de suspensión de PROFECO y fortalece el prestigio de la marca.",
                    pasos_cumplimiento: &[
                        "En toda promoción, descuento u oferta publicada en web o redes sociales (Art. 46 LFPC): indicar siempre las condiciones, el plazo de vigencia o el volumen exacto de piezas disponibles.",
                        "Conservar el soporte técnico o documental que acredite cualquier atributo cuantitativo prometido en su publicidad comercial."
                    ],
                    entregable_kit: "Lineamientos de Publicidad Veraz Art. 32 en 07_Terminos_Ecommerce_PROFECO_76BIS.pdf",
                },
                NormaEducativa {
                    codigo: "Artículo 128 BIS y Artículo 76 BIS Fracción II de la LFPC • NOM-151-SCFI-2016",
                    titulo: "Seguridad en Transacciones Financieras Digitales, Pasarelas de Pago y Conservación NOM-151",
                    incumplimiento_pct: 53,
                    fuente_estadistica: "53% de las denuncias graves en comercio digital involucran cobros no reconocidos o portales que procesan pagos sin estándares de autenticación 3D Secure / TLS ni sellado de integridad NOM-151.",
                    rango_multa: "Caso particularmente grave: Multa de $196,634.08 a $5,505,754.46 MXN y clausura hasta por 90 días (Art. 128 BIS)",
                    contexto_dof: "El Artículo 128 BIS fue adicionado en el DOF el 4 de febrero de 2004 y actualizado el 11 de enero de 2018; la NOM-151-SCFI-2016 se publicó en el DOF el 30 de marzo de 2017.",
                    de_que_trata: "Exige utilizar elementos técnicos confiables para brindar seguridad y confidencialidad a la información financiera proporcionada por el consumidor (tokenización PCI-DSS, cifrado TLS 1.3 y autenticación 3D Secure) y conservar los mensajes de datos íntegros conforme al Código de Comercio y la NOM-151.",
                    beneficio_productividad: "Otorga prueba criptográfica irrefutable ante bancos emisores para ganar el 100% de las aclaraciones por cargos no reconocidos y dota de Fecha Cierta a cada contrato digital.",
                    pasos_cumplimiento: &[
                        "Operar el cobro con tarjeta exclusivamente mediante pasarelas tokenizadas certificadas PCI-DSS Nivel 1 con regla obligatoria de 3D Secure.",
                        "Sellar cada transacción y contrato electrónico con huella criptográfica SHA-256 e incluirlo en el Árbol de Merkle de Fecha Cierta."
                    ],
                    entregable_kit: "Certificado_Digital_NOM151.pdf y Manifiesto de Fecha Cierta SCJN",
                },
            ],
        ),
    }
}

pub fn render_norma_category(category: &str) -> Html<String> {
    let (siglas, titulo, subtitulo, resumen_ejecutivo, color_tw, border_top, normas) =
        get_category_data(category);

    let mut modulos_html = String::from(r#"<div class="grid grid-cols-1 md:grid-cols-2 gap-5 mb-8">"#);
    for (idx, n) in normas.iter().enumerate() {
        let num_exp = idx + 1;
        let pasos_count = n.pasos_cumplimiento.len();

        modulos_html.push_str(&format!(
            r##"<div class="quantum-card rounded-xl p-5 border border-{color_tw}-500/30 flex flex-col justify-between group">
                <div>
                    <div class="flex items-center justify-between gap-2 mb-2">
                        <span class="inline-block text-[10px] font-bold font-mono-code text-{color_tw}-400 bg-{color_tw}-950/60 border border-{color_tw}-500/30 px-2.5 py-0.5 rounded">
                            {codigo}
                        </span>
                        <span class="text-[10px] font-mono-code text-slate-400">Expediente #{num_exp:02}</span>
                    </div>

                    <h2 class="text-base sm:text-lg font-extrabold text-white tracking-tight leading-snug mb-3 group-hover:text-{color_tw}-300 transition">
                        {titulo_norma}
                    </h2>

                    <div class="w-full bg-slate-950/90 border border-red-500/30 rounded-lg p-3 mb-3.5">
                        <div class="flex items-center justify-between text-[10px] font-mono-code mb-1">
                            <span class="uppercase text-slate-300 font-semibold">Incumplimiento en México:</span>
                            <span class="text-xs font-extrabold text-red-400">{pct}% de empresas inspeccionadas</span>
                        </div>
                        <div class="w-full bg-slate-800 h-2.5 rounded-full overflow-hidden border border-white/10">
                            <div class="bg-gradient-to-r from-amber-500 via-red-500 to-rose-600 h-full rounded-full" style="width: {pct}% !important;"></div>
                        </div>
                        <div class="flex justify-between text-[8px] font-mono-code text-slate-500 mt-1">
                            <span>0%</span><span>25%</span><span>50%</span><span>75%</span><span>100%</span>
                        </div>
                        <div class="flex items-center justify-between text-[9px] font-mono-code text-amber-300 mt-1.5 pt-1.5 border-t border-white/5">
                            <span>Riesgo de Sanción:</span>
                            <strong class="text-amber-200">{multa}</strong>
                        </div>
                    </div>

                    <p class="text-xs text-slate-300 font-mono-code leading-relaxed mb-4 line-clamp-2">
                        {estadistica}
                    </p>

                    <div class="grid grid-cols-3 gap-2 mb-4 text-center font-mono-code">
                        <div class="bg-slate-950/60 border border-white/10 rounded-lg py-1.5 px-2">
                            <span class="text-[9px] text-emerald-400 font-bold block">ROI & Salud</span>
                            <span class="text-[9px] text-slate-400">Ventaja Operativa</span>
                        </div>
                        <div class="bg-slate-950/60 border border-white/10 rounded-lg py-1.5 px-2">
                            <span class="text-[9px] text-{color_tw}-400 font-bold block">Contexto DOF</span>
                            <span class="text-[9px] text-slate-400">Marco y Sujetos</span>
                        </div>
                        <div class="bg-slate-950/60 border border-white/10 rounded-lg py-1.5 px-2">
                            <span class="text-[9px] text-amber-400 font-bold block">{pasos_count} Pasos Clave</span>
                            <span class="text-[9px] text-slate-400">Guía de Blindaje</span>
                        </div>
                    </div>
                </div>

                <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-2.5 pt-3 border-t border-white/10 text-[11px] font-mono-code">
                    <a href="/normas/{cat_slug}/{idx}" class="flex-1 bg-gradient-to-r from-{color_tw}-600/30 to-slate-900 hover:from-{color_tw}-600/50 hover:to-slate-800 text-white border border-{color_tw}-500/50 font-bold px-3.5 py-2.5 rounded-lg transition flex items-center justify-between group/btn">
                        <span>Descubrir cómo cumplir y proteger a su equipo</span>
                        <span class="text-{color_tw}-300 group-hover/btn:translate-x-1 transition font-extrabold ml-2">→</span>
                    </a>
                    <a href="/#express-check" class="shrink-0 bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white border border-white/15 font-bold px-3 py-2.5 rounded-lg transition text-center">
                        Auditar RFC
                    </a>
                </div>
            </div>"##,
            color_tw = color_tw,
            codigo = n.codigo,
            num_exp = num_exp,
            titulo_norma = n.titulo,
            pct = n.incumplimiento_pct,
            multa = n.rango_multa,
            estadistica = n.fuente_estadistica,
            pasos_count = pasos_count,
            cat_slug = category,
            idx = idx
        ));
    }
    modulos_html.push_str("</div>");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{titulo} — Cobertura Normativa | FIRMA PERICIAL MALLEN®</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.25">
    <script src="/js/quantum-lights.js?v=2026.25" defer></script>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-4 sm:p-8 border-t-4 {border_top}">
    <div class="max-w-7xl mx-auto">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-5 mb-6 border-b border-white/15 gap-4">
            <div>
                <span class="text-xs font-bold text-{color_tw}-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN® • CENTRO DE CUMPLIMIENTO Y RENDIMIENTO EMPRESARIAL</span>
                <span class="text-xs text-slate-400 font-mono-code">{siglas}</span>
            </div>
            <div class="flex flex-wrap gap-2 text-[11px] font-mono-code">
                <a href="/normas/stps" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-blue-400 text-slate-300 transition">STPS (39 NOMs)</a>
                <a href="/normas/sat" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-emerald-400 text-slate-300 transition">SAT / CFF</a>
                <a href="/normas/imss" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-purple-400 text-slate-300 transition">IMSS / INFONAVIT</a>
                <a href="/normas/inai" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-cyan-400 text-slate-300 transition">LFPDPPP / SABG</a>
                <a href="/normas/pld" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-amber-400 text-slate-300 transition">9 Sectoriales & PLD</a>
                <a href="/normas/comercio" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-teal-400 text-slate-300 transition">PROFECO & NOM-151</a>
                <a href="/" class="px-3 py-1.5 rounded-lg bg-{color_tw}-600 text-white font-bold transition">Portal Principal</a>
            </div>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 mb-8">
            <span class="text-xs font-bold font-mono-code text-{color_tw}-400 uppercase tracking-widest block mb-1.5">{siglas}</span>
            <h1 class="text-xl sm:text-3xl font-extrabold text-white mb-2 tracking-tight">{titulo}</h1>
            <p class="text-xs sm:text-sm text-slate-300 font-mono-code mb-3">{subtitulo}</p>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed border-t border-white/10 pt-3">
                {resumen_ejecutivo} — <strong>Seleccione cualquier expediente inferior para abrir su desglose profundo, criterios de inspección y guía práctica paso a paso.</strong>
            </p>
        </div>

        {modulos_html}

        <div class="quantum-card rounded-2xl p-6 text-center border border-emerald-500/30 mt-6">
            <h3 class="text-lg font-extrabold text-white mb-1.5">Proteja a su equipo de trabajo, eleve su productividad y blinde el patrimonio de su empresa</h3>
            <p class="text-xs text-slate-300 font-mono-code max-w-2xl mx-auto mb-5">
                Ingrese su RFC o Nombre Comercial y sitio web en nuestro motor pericial para cruzar sus datos contra los padrones nacionales y obtener sus 10 Certificados y 7 Instrumentos Jurídicos listos para firma.
            </p>
            <a href="/#express-check" class="inline-block bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white font-bold text-xs font-mono-code px-7 py-3.5 rounded-xl transition uppercase tracking-wider shadow-lg shadow-emerald-600/20">
                Ejecutar Diagnóstico Pericial 360° con mi RFC
            </a>
        </div>
    </div>
</body>
</html>"##,
        titulo = titulo,
        border_top = border_top,
        color_tw = color_tw,
        siglas = siglas,
        subtitulo = subtitulo,
        resumen_ejecutivo = resumen_ejecutivo,
        modulos_html = modulos_html
    );

    Html(html)
}

pub fn render_subnorma_detail(category: &str, code_idx: &str) -> Html<String> {
    let (siglas, titulo, _sub, _res, color_tw, border_top, normas) =
        get_category_data(category);

    let idx = code_idx.parse::<usize>().unwrap_or(0);
    let n = normas.get(idx).unwrap_or(&normas[0]);

    let mut pasos_li = String::new();
    for (p_i, paso) in n.pasos_cumplimiento.iter().enumerate() {
        pasos_li.push_str(&format!(
            r##"<li class="flex items-start gap-3 text-xs sm:text-sm text-slate-200 leading-relaxed bg-slate-900/75 border border-white/10 rounded-xl p-3.5">
                <span class="shrink-0 w-6 h-6 rounded-full bg-{color_tw}-500/20 border border-{color_tw}-400/50 text-{color_tw}-300 font-bold flex items-center justify-center text-xs">{num}</span>
                <span>{paso}</span>
            </li>"##,
            color_tw = color_tw,
            num = p_i + 1,
            paso = paso
        ));
    }

    let mut otros_html = String::new();
    for (o_idx, o_norm) in normas.iter().enumerate() {
        if o_idx == idx {
            continue;
        }
        otros_html.push_str(&format!(
            r##"<a href="/normas/{cat}/{o_idx}" class="quantum-card rounded-xl p-4 border border-white/10 hover:border-{color_tw}-400/60 transition flex flex-col justify-between group">
                <div>
                    <div class="flex items-center justify-between gap-2 mb-1.5">
                        <span class="text-[10px] font-mono-code font-bold text-{color_tw}-400">{cod}</span>
                        <span class="text-[10px] font-mono-code font-bold text-red-400">{pct}% Incumplimiento</span>
                    </div>
                    <h4 class="text-xs sm:text-sm font-bold text-white group-hover:text-{color_tw}-300 transition leading-snug mb-2">{tit}</h4>
                </div>
                <span class="text-[10px] font-mono-code text-{color_tw}-400 font-bold flex items-center justify-between pt-2 border-t border-white/10">
                    <span>Abrir expediente técnico completo</span>
                    <span class="group-hover:translate-x-1 transition">→</span>
                </span>
            </a>"##,
            cat = category,
            o_idx = o_idx,
            color_tw = color_tw,
            cod = o_norm.codigo,
            pct = o_norm.incumplimiento_pct,
            tit = o_norm.titulo
        ));
    }

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{titulo_norma} — Expediente Técnico | FIRMA PERICIAL MALLEN®</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.25">
    <script src="/js/quantum-lights.js?v=2026.25" defer></script>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-4 sm:p-8 border-t-4 {border_top}">
    <div class="max-w-5xl mx-auto">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-5 mb-6 border-b border-white/15 gap-4">
            <div>
                <a href="/normas/{cat}" class="inline-flex items-center gap-2 text-xs font-bold text-{color_tw}-400 hover:text-{color_tw}-300 font-mono-code mb-1 transition">
                    <span>← Volver al Tablero de {titulo_cat}</span>
                </a>
                <span class="text-[11px] text-slate-400 font-mono-code block">{siglas}</span>
            </div>
            <div class="flex flex-wrap gap-2 text-[11px] font-mono-code">
                <a href="/normas/stps" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-blue-400 text-slate-300 transition">STPS</a>
                <a href="/normas/sat" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-emerald-400 text-slate-300 transition">SAT</a>
                <a href="/normas/imss" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-purple-400 text-slate-300 transition">IMSS</a>
                <a href="/normas/inai" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-cyan-400 text-slate-300 transition">LFPDPPP</a>
                <a href="/normas/pld" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-amber-400 text-slate-300 transition">Sectoriales</a>
                <a href="/normas/comercio" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-teal-400 text-slate-300 transition">PROFECO</a>
                <a href="/" class="px-3 py-1 rounded-lg bg-{color_tw}-600 text-white font-bold transition">Portal Principal</a>
            </div>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 sm:p-8 mb-8">
            <div class="flex flex-wrap items-center justify-between gap-2 mb-3">
                <span class="inline-block text-xs font-bold font-mono-code text-{color_tw}-400 bg-{color_tw}-950/70 border border-{color_tw}-500/40 px-3 py-1 rounded-lg">
                    {codigo}
                </span>
                <span class="text-xs font-mono-code text-amber-300 bg-amber-950/50 border border-amber-500/40 px-3 py-1 rounded-lg font-bold">
                    Sanción Oficial: {multa}
                </span>
            </div>

            <h1 class="text-xl sm:text-3xl font-extrabold text-white tracking-tight leading-snug mb-5">
                {titulo_norma}
            </h1>

            <div class="w-full bg-slate-950/90 border border-red-500/35 rounded-xl p-4 mb-6">
                <div class="flex items-center justify-between text-xs font-mono-code mb-2">
                    <span class="uppercase text-slate-300 font-bold">Índice Real de Incumplimiento en México:</span>
                    <span class="text-sm font-extrabold text-red-400">{pct}% de los centros inspeccionados</span>
                </div>
                <div class="w-full bg-slate-800 h-3 rounded-full overflow-hidden border border-white/10">
                    <div class="bg-gradient-to-r from-amber-500 via-red-500 to-rose-600 h-full rounded-full" style="width: {pct}% !important;"></div>
                </div>
                <div class="flex justify-between text-[9px] font-mono-code text-slate-500 mt-1.5">
                    <span>0%</span><span>25%</span><span>50%</span><span>75%</span><span>100%</span>
                </div>
            </div>

            <div class="bg-red-950/25 border border-red-500/35 rounded-xl p-4 mb-5 font-mono-code">
                <strong class="text-red-400 uppercase block text-[11px] mb-1 tracking-wider">DIAGNÓSTICO DE INSPECCIÓN FEDERAL:</strong>
                <p class="text-xs sm:text-sm text-slate-200 leading-relaxed">{estadistica}</p>
            </div>

            <div class="grid sm:grid-cols-2 gap-4 mb-5 font-mono-code">
                <div class="bg-slate-950/70 border border-white/15 rounded-xl p-5">
                    <span class="text-[11px] font-bold text-{color_tw}-400 uppercase tracking-wider block mb-2">
                        MARCO LEGAL Y CONTEXTO (DOF)
                    </span>
                    <p class="text-xs text-slate-300 leading-relaxed">{contexto}</p>
                </div>
                <div class="bg-slate-950/70 border border-white/15 rounded-xl p-5">
                    <span class="text-[11px] font-bold text-sky-400 uppercase tracking-wider block mb-2">
                        ALCANCE Y SUJETOS OBLIGADOS
                    </span>
                    <p class="text-xs text-slate-300 leading-relaxed">{trata}</p>
                </div>
            </div>

            <div class="bg-emerald-950/30 border border-emerald-500/40 rounded-xl p-5 mb-6 font-mono-code">
                <strong class="text-emerald-400 uppercase block text-[11px] mb-1.5 tracking-wider">
                    VENTAJA COMPETITIVA, SEGURIDAD DEL PERSONAL Y RENDIMIENTO:
                </strong>
                <p class="text-xs sm:text-sm text-emerald-100 leading-relaxed">{beneficio}</p>
            </div>

            <div class="bg-slate-950/85 border border-{color_tw}-500/35 rounded-xl p-5 mb-6 font-mono-code">
                <span class="text-xs font-bold text-white uppercase tracking-wider block mb-3.5">
                    GUÍA PRÁCTICA PARA CUMPLIR AL 100%:
                </span>
                <ul class="space-y-2.5">{pasos_li}</ul>
            </div>

            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pt-5 border-t border-white/15 font-mono-code text-xs">
                <div>
                    <span class="text-slate-400 block text-[10px] uppercase">Instrumento incluido en su Kit Mallen 360°:</span>
                    <strong class="text-emerald-400 text-sm">{entregable}</strong>
                </div>
                <div class="flex flex-wrap gap-3">
                    <a href="/normas/{cat}" class="px-4 py-3 rounded-xl bg-slate-900 hover:bg-slate-800 text-slate-200 border border-white/15 font-bold transition">
                        ← Ver más normas de {cat_upper}
                    </a>
                    <a href="/#express-check" class="px-6 py-3 rounded-xl bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white font-bold uppercase shadow-lg shadow-emerald-600/25 transition">
                        Auditar este precepto con mi RFC →
                    </a>
                </div>
            </div>
        </div>

        <div class="mb-8">
            <div class="flex items-center justify-between mb-4">
                <div>
                    <span class="text-[10px] font-mono-code font-bold text-{color_tw}-400 uppercase tracking-widest block">Exploración Continua de Cumplimiento</span>
                    <h3 class="text-lg font-extrabold text-white">Otros Expedientes Críticos en {titulo_cat}</h3>
                </div>
                <a href="/normas/{cat}" class="text-xs font-mono-code text-{color_tw}-400 hover:underline">Ver todos →</a>
            </div>
            <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">{otros_html}</div>
        </div>
    </div>
</body>
</html>"##,
        titulo_norma = n.titulo,
        border_top = border_top,
        cat = category,
        color_tw = color_tw,
        titulo_cat = titulo,
        siglas = siglas,
        codigo = n.codigo,
        multa = n.rango_multa,
        pct = n.incumplimiento_pct,
        estadistica = n.fuente_estadistica,
        contexto = n.contexto_dof,
        trata = n.de_que_trata,
        beneficio = n.beneficio_productividad,
        pasos_li = pasos_li,
        entregable = n.entregable_kit,
        cat_upper = category.to_uppercase(),
        otros_html = otros_html
    );

    Html(html)
}
