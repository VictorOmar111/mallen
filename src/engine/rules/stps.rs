//! Motor de Reglas STPS (39 Normas Oficiales Mexicanas divididas en 5 categorías):
//! 1. Seguridad (12 NOMs): 001, 002, 004, 005, 006, 009, 020, 022, 027, 029, 033, 034
//! 2. Salud y Agentes (8 NOMs): 010, 011, 012, 013, 014, 015, 024, 025
//! 3. Organización (6 NOMs): 017, 018, 019, 026, 028, 030
//! 4. Ergonomía, Psicosocial y Teletrabajo (3 NOMs): 035, 036-1, 037
//! 5. Específicas de Sector por SCIAN (7 NOMs): 003, 007, 008, 016, 023, 031, 032

use crate::engine::models::{
    CalibrationAnswers, EmpresaB2B, RegulatoryAgency, RegulatoryFinding, RiskSeverity,
    UmaCalculator,
};
use phf::phf_map;

/// Diccionario estático O(1) en tiempo de compilación: Prefijo SCIAN (2-3 dígitos) -> Perfil Industrial STPS
/// Formato tupla: (Requiere Maquinaria/Químicos, Es Construcción, Es Agro/Forestal, Es Minería, Es Ferroviario)
static SCIAN_STPS_PROFILE: phf::Map<&'static str, (bool, bool, bool, bool, bool)> = phf_map! {
    "11" => (true, false, true, false, false),  // Agricultura, cría y explotación de animales, aprovechamiento forestal (NOM-003, NOM-007, NOM-008)
    "21" => (true, false, false, true, false),  // Minería (NOM-023, NOM-032, NOM-014)
    "22" => (true, false, false, false, false), // Generación y distribución de energía eléctrica, agua y gas (NOM-020, NOM-022, NOM-028, NOM-029)
    "23" => (true, true, false, false, false),  // Construcción (NOM-031, NOM-009, NOM-027, NOM-033)
    "31" => (true, false, false, false, false), // Industrias manufactureras (Alimentos, textil)
    "32" => (true, false, false, false, false), // Industrias manufactureras (Química, plástico, papel - NOM-005, NOM-010, NOM-018, NOM-028)
    "33" => (true, false, false, false, false), // Industrias manufactureras (Metalmecánica, maquinaria, electrónica - NOM-004, NOM-022, NOM-027)
    "43" => (true, false, false, false, false), // Comercio al por mayor (Bodegas, montacargas - NOM-006, NOM-036-1)
    "46" => (false, false, false, false, false),// Comercio al por menor
    "48" => (true, false, false, false, true),  // Transporte, correos y almacenamiento (NOM-016 ferroviario, NOM-005, NOM-006)
    "49" => (true, false, false, false, false), // Almacenamiento y logística
    "51" => (false, false, false, false, false),// Información en medios masivos y software (NOM-037 Teletrabajo)
    "52" => (false, false, false, false, false),// Servicios financieros y de seguros (NOM-035, NOM-037)
    "54" => (false, false, false, false, false),// Servicios profesionales, científicos y técnicos (NOM-035, NOM-037)
    "62" => (true, false, false, false, false), // Servicios de salud y de asistencia social (NOM-012 Radiaciones, NOM-010, NOM-018)
    "72" => (true, false, false, false, false), // Servicios de alojamiento temporal y preparación de alimentos y bebidas (NOM-002, NOM-020, NOM-015)
};

/// Extrae el número estimado de trabajadores desde el estrato del DENUE o la respuesta explícita del cuestionario.
pub fn parse_employee_count(denue_str: &str, explicit: Option<u32>) -> u32 {
    if let Some(exact) = explicit {
        return exact;
    }
    let s = denue_str.to_lowercase();
    if s.contains("251") {
        300
    } else if s.contains("101 a 250") {
        150
    } else if s.contains("51 a 100") {
        75
    } else if s.contains("31 a 50") {
        40
    } else if s.contains("11 a 30") {
        20
    } else if s.contains("6 a 10") {
        8
    } else {
        5
    }
}

/// Función original 100% retrocompatible con `EmpresaB2B`, ahora respaldada por la matriz completa de 39 NOMs.
pub fn evaluate_stps(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let calc = UmaCalculator {
        uma_daily_mxn: valor_uma,
        fiscal_year: 2026,
    };
    let emp_count = parse_employee_count(&empresa.direccion, None);
    let default_cal = CalibrationAnswers::default();
    let findings = evaluate_stps_detailed(
        &empresa.actividad,
        emp_count,
        &default_cal,
        &calc,
    );

    let mut total_multa = 0.0;
    for f in findings {
        empresa.audit_flags.push(format!("STPS_{}", f.code.replace('-', "_")));
        total_multa += f.estimated_fine_mxn;
    }
    total_multa
}

/// Evaluación Pericial Completa de las 39 NOMs de la STPS (Cruza SCIAN + Plantilla + Cuestionario de Certeza).
pub fn evaluate_stps_detailed(
    scian: &str,
    employee_count: u32,
    cal: &CalibrationAnswers,
    uma: &UmaCalculator,
) -> Vec<RegulatoryFinding> {
    let mut findings = Vec::new();
    let prefix2 = if scian.len() >= 2 { &scian[..2] } else { "46" };
    let (is_industrial, is_construction, is_agro, is_mining, is_rail_transport) =
        SCIAN_STPS_PROFILE
            .get(prefix2)
            .copied()
            .unwrap_or((false, false, false, false, false));

    let is_office_or_tech = matches!(prefix2, "51" | "52" | "54" | "55" | "56");
    let is_chemical_or_health = matches!(prefix2, "32" | "62" | "21");
    let is_hospitality_or_food = matches!(prefix2, "72" | "31");

    let mut push_nom = |code: &str,
                        title: &str,
                        basis: &str,
                        severity: RiskSeverity,
                        desc: &str,
                        remedy: &str,
                        min_u: u32,
                        max_u: u32| {
        let (min_m, est_m, max_m) = uma.compute_fine_bands(min_u, max_u, employee_count, true);
        findings.push(RegulatoryFinding {
            code: code.to_string(),
            agency: RegulatoryAgency::STPS,
            title: title.to_string(),
            legal_basis: basis.to_string(),
            severity,
            description: desc.to_string(),
            remediation_action: remedy.to_string(),
            min_uma: min_u,
            max_uma: max_u,
            min_fine_mxn: min_m,
            estimated_fine_mxn: est_m,
            max_fine_mxn: max_m,
            verified_by_osint: false,
        });
    };

    // =========================================================================
    // I. SEGURIDAD (12 NOMs: 001, 002, 004, 005, 006, 009, 020, 022, 027, 029, 033, 034)
    // =========================================================================

    // 1. NOM-001-STPS-2008 (Obligatoria para el 100% de centros de trabajo)
    push_nom(
        "NOM-001-STPS-2008",
        "Condiciones de seguridad en edificios, locales e instalaciones",
        "Art. 132 Fracc. XVI LFT / Art. 994 Fracc. V LFT / NOM-001-STPS-2008",
        RiskSeverity::Medium,
        "Omisión de constancias documentales de verificación anual de techos, paredes, pisos, escaleras y ventilación.",
        "Integrar bitácora de verificación ocular anual y dictamen de conservación de áreas del centro de trabajo.",
        250,
        5000,
    );

    // 2. NOM-002-STPS-2010 (Obligatoria para el 100% de centros de trabajo)
    push_nom(
        "NOM-002-STPS-2010",
        "Prevención y protección contra incendios en los centros de trabajo",
        "Art. 994 Fracc. V LFT / NOM-002-STPS-2010",
        RiskSeverity::High,
        "Falta de estudio de clasificación de grado de riesgo de incendio (Ordinario/Alto), croquis, brigadas y bitácora de extintores.",
        "Emitir Estudio de Clasificación de Riesgo de Incendio, actas de simulacros e integración de brigadas.",
        250,
        5000,
    );

    // 3. NOM-004-STPS-1999 (Maquinaria y Equipo)
    if CalibrationAnswers::resolve_flag(cal.has_heavy_machinery, is_industrial || is_construction) {
        push_nom(
            "NOM-004-STPS-1999",
            "Sistemas de protección y dispositivos de seguridad en la maquinaria y equipo",
            "Art. 994 Fracc. V LFT / NOM-004-STPS-1999",
            RiskSeverity::High,
            "Ausencia de estudio de riesgo potencial en maquinaria, procedimientos LOTO (bloqueo y etiquetado) y guardas físicas.",
            "Implementar programa específico de seguridad para operación y mantenimiento de maquinaria y bloqueo de energía.",
            250,
            5000,
        );
    }

    // 4. NOM-005-STPS-1998 (Sustancias Químicas Peligrosas)
    if CalibrationAnswers::resolve_flag(
        cal.handles_hazardous_chemicals,
        is_industrial || is_chemical_or_health,
    ) {
        push_nom(
            "NOM-005-STPS-1998",
            "Manejo, transporte y almacenamiento de sustancias químicas peligrosas",
            "Art. 994 Fracc. V LFT / NOM-005-STPS-1998",
            RiskSeverity::High,
            "Falta de estudio para analizar el riesgo potencial de sustancias químicas inflamables, corrosivas o tóxicas.",
            "Elaborar estudio de riesgo químico, instalar regaderas/lavaojos neutralizadores y manual de primeros auxilios.",
            250,
            5000,
        );
    }

    // 5. NOM-006-STPS-2014 (Manejo y almacenamiento de materiales / Montacargas)
    if CalibrationAnswers::resolve_flag(
        cal.has_forklifts_or_manual_loads,
        is_industrial || is_construction || prefix2 == "43" || prefix2 == "48" || prefix2 == "49",
    ) {
        push_nom(
            "NOM-006-STPS-2014",
            "Manejo y almacenamiento de materiales — Condiciones de seguridad y salud",
            "Art. 994 Fracc. V LFT / NOM-006-STPS-2014",
            RiskSeverity::Medium,
            "Falta de manuales de seguridad para operación de montacargas, patines hidráulicos, polipastos y estiba máxima.",
            "Documentar procedimientos de seguridad en almacén, constancias DC-3 de operadores y bitácoras de revisión.",
            250,
            5000,
        );
    }

    // 6. NOM-009-STPS-2011 (Trabajos en altura > 1.80m)
    if CalibrationAnswers::resolve_flag(cal.performs_height_work, is_construction || is_industrial) {
        push_nom(
            "NOM-009-STPS-2011",
            "Condiciones de seguridad para realizar trabajos en altura",
            "Art. 994 Fracc. V LFT / NOM-009-STPS-2011",
            RiskSeverity::High,
            "Omisión de permisos de trabajo en altura (>1.80 m), revisión de sistemas anticaídas y exámenes médicos especiales.",
            "Implementar formato de autorización por escrito para trabajos en altura y bitácora de arneses/líneas de vida.",
            250,
            5000,
        );
    }

    // 7. NOM-020-STPS-2011 (Recipientes sujetos a presión y calderas)
    if CalibrationAnswers::resolve_flag(
        cal.has_pressure_vessels_boilers,
        is_industrial || is_hospitality_or_food,
    ) {
        push_nom(
            "NOM-020-STPS-2011",
            "Recipientes sujetos a presión, recipientes criogénicos y generadores de vapor o calderas",
            "Art. 994 Fracc. V LFT / NOM-020-STPS-2011",
            RiskSeverity::CriticalClosure,
            "Falta de listado clasificado (Categorías I, II y III), expediente de integridad mecánica y aviso a la STPS.",
            "Integrar listado de compresores/calderas, planos, pruebas hidrostáticas/exámenes no destructivos y número de control STPS.",
            250,
            5000,
        );
    }

    // 8. NOM-022-STPS-2015 (Electricidad estática y tierras físicas)
    if CalibrationAnswers::resolve_flag(
        cal.performs_electrical_maintenance,
        is_industrial || is_construction || prefix2 == "43",
    ) {
        push_nom(
            "NOM-022-STPS-2015",
            "Electricidad estática en los centros de trabajo — Condiciones de seguridad",
            "Art. 994 Fracc. V LFT / NOM-022-STPS-2015",
            RiskSeverity::Medium,
            "Ausencia de informe anual de medición de resistencia de la red de puesta a tierra (<10 ohms en pararrayos, <25 ohms general).",
            "Realizar medición anual de tierras físicas por laboratorio acreditado por EMA y registrar continuidad eléctrica.",
            250,
            5000,
        );
    }

    // 9. NOM-027-STPS-2008 (Soldadura y corte)
    if CalibrationAnswers::resolve_flag(cal.performs_welding_work, is_construction || prefix2 == "33")
    {
        push_nom(
            "NOM-027-STPS-2008",
            "Actividades de soldadura y corte — Condiciones de seguridad e higiene",
            "Art. 994 Fracc. V LFT / NOM-027-STPS-2008",
            RiskSeverity::High,
            "Falta de análisis de riesgos potenciales para procesos de soldadura, mamparas y autorización escrita en áreas no designadas.",
            "Implementar permisos de trabajo en caliente, hojas de seguridad de gases y capacitación especializada.",
            250,
            5000,
        );
    }

    // 10. NOM-029-STPS-2011 (Mantenimiento de instalaciones eléctricas)
    push_nom(
        "NOM-029-STPS-2011",
        "Mantenimiento de las instalaciones eléctricas en los centros de trabajo",
        "Art. 994 Fracc. V LFT / NOM-029-STPS-2011",
        RiskSeverity::High,
        "Falta de diagrama unifilar actualizado con cuadro general de cargas, etiquetado de tableros y procedimientos de seguridad.",
        "Integrar diagrama unifilar firmado, programa anual de mantenimiento eléctrico y autorización al personal capacitado.",
        250,
        5000,
    );

    // 11. NOM-033-STPS-2015 (Espacios confinados)
    if CalibrationAnswers::resolve_flag(cal.has_confined_spaces, is_construction || is_mining) {
        push_nom(
            "NOM-033-STPS-2015",
            "Condiciones de seguridad para realizar trabajos en espacios confinados",
            "Art. 994 Fracc. V LFT / NOM-033-STPS-2015",
            RiskSeverity::CriticalClosure,
            "Ingreso a cisternas, tanques, fosas o ductos sin clasificación (Tipo I/II), monitoreo de atmósfera ni plan de rescate.",
            "Establecer muestreo previo de oxígeno/explosividad, vigía externo y permisos de entrada firmados.",
            250,
            5000,
        );
    }

    // 12. NOM-034-STPS-2016 (Trabajadores con discapacidad)
    if CalibrationAnswers::resolve_flag(cal.has_disabled_workers, employee_count >= 50) {
        push_nom(
            "NOM-034-STPS-2016",
            "Condiciones de seguridad para el acceso y desarrollo de actividades de trabajadores con discapacidad",
            "Art. 994 Fracc. V LFT / NOM-034-STPS-2016",
            RiskSeverity::Low,
            "Falta de análisis de compatibilidad puesto-persona y plan de atención a emergencias inclusivo.",
            "Documentar análisis de accesibilidad y señalización visual/auditiva/táctil aplicable.",
            50,
            2500,
        );
    }

    // =========================================================================
    // II. SALUD Y AGENTES CONTAMINANTES (8 NOMs: 010, 011, 012, 013, 014, 015, 024, 025)
    // =========================================================================

    // 13. NOM-010-STPS-2014 (Agentes químicos contaminantes del ambiente laboral)
    if CalibrationAnswers::resolve_flag(cal.handles_hazardous_chemicals, is_chemical_or_health) {
        push_nom(
            "NOM-010-STPS-2014",
            "Agentes químicos contaminantes del ambiente laboral — Reconocimiento, evaluación y control",
            "Art. 994 Fracc. V LFT / NOM-010-STPS-2014",
            RiskSeverity::High,
            "Falta de estudio actualizado de agentes químicos en aire y vigilancia a la salud del personal ocupacionalmente expuesto (POE).",
            "Ejecutar reconocimiento y evaluación con laboratorio acreditado conforme a Valores Límite de Exposición (VLE).",
            250,
            5000,
        );
    }

    // 14. NOM-011-STPS-2001 (Ruido en centros de trabajo)
    if CalibrationAnswers::resolve_flag(cal.exposed_to_industrial_noise, is_industrial || is_construction) {
        push_nom(
            "NOM-011-STPS-2001",
            "Condiciones de seguridad e higiene en los centros de trabajo donde se genere ruido",
            "Art. 994 Fracc. V LFT / NOM-011-STPS-2001",
            RiskSeverity::Medium,
            "Ausencia de Programa de Conservación de la Audición, sonometrías/dosimetrías (>85 dB) y audiometrías anuales al POE.",
            "Realizar estudio de nivel de exposición a ruido (NER) y dotar equipo de protección auditiva con factor de atenuación NRR.",
            250,
            5000,
        );
    }

    // 15 y 16. NOM-012-STPS-2012 (Radiaciones ionizantes) y NOM-013-STPS-1993 (No ionizantes)
    if CalibrationAnswers::resolve_flag(
        cal.exposed_to_vibrations_or_radiation,
        prefix2 == "62" || prefix2 == "33",
    ) {
        push_nom(
            "NOM-012-013-STPS",
            "Seguridad y salud en fuentes de radiación ionizante (NOM-012) y no ionizante UV/Láser/RF (NOM-013)",
            "Art. 994 Fracc. V LFT / NOM-012-STPS-2012 y NOM-013-STPS-1993",
            RiskSeverity::High,
            "Falta de manual de seguridad radiológica, dosimetría personal o evaluación de campos electromagnéticos/láser.",
            "Integrar licencias CNSNS/COFEPRIS, vigilancia dosimétrica y delimitación de zonas controladas.",
            250,
            5000,
        );
    }

    // 17. NOM-014-STPS-2000 (Presiones ambientales anormales)
    if is_mining {
        push_nom(
            "NOM-014-STPS-2000",
            "Exposición laboral a presiones ambientales anormales",
            "Art. 994 Fracc. V LFT / NOM-014-STPS-2000",
            RiskSeverity::Medium,
            "Omisión de controles médicos y tiempos de descompresión/aclimatación en labores bajo presiones anormales.",
            "Documentar bitácoras médicas y protocolos de seguridad hiperbárica/hipobárica.",
            250,
            5000,
        );
    }

    // 18. NOM-015-STPS-2001 (Condiciones térmicas elevadas o abatidas)
    if CalibrationAnswers::resolve_flag(
        cal.exposed_to_extreme_temperatures,
        is_hospitality_or_food || is_industrial,
    ) {
        push_nom(
            "NOM-015-STPS-2001",
            "Condiciones térmicas elevadas o abatidas — Condiciones de seguridad e higiene",
            "Art. 994 Fracc. V LFT / NOM-015-STPS-2001",
            RiskSeverity::Medium,
            "Falta de estudio de estrés térmico en hornos, cocinas industriales o cámaras de refrigeración/congelación.",
            "Evaluar índice de temperatura de globo bulbo húmedo (Itgbh), ciclos de recuperación e hidratación.",
            250,
            5000,
        );
    }

    // 19. NOM-024-STPS-2001 (Vibraciones mecánicas)
    if CalibrationAnswers::resolve_flag(cal.exposed_to_vibrations_or_radiation, is_construction || is_mining) {
        push_nom(
            "NOM-024-STPS-2001",
            "Vibraciones en los centros de trabajo — Condiciones de seguridad e higiene",
            "Art. 994 Fracc. V LFT / NOM-024-STPS-2001",
            RiskSeverity::Medium,
            "Falta de evaluación de aceleración longitudinal/transversal en cuerpo entero o extremidades superiores.",
            "Implementar programa para la prevención de alteraciones a la salud por vibraciones mecánicas.",
            250,
            5000,
        );
    }

    // 20. NOM-025-STPS-2008 (Iluminación en el centro de trabajo - Obligatoria universal)
    push_nom(
        "NOM-025-STPS-2008",
        "Condiciones de iluminación en los centros de trabajo",
        "Art. 994 Fracc. V LFT / NOM-025-STPS-2008",
        RiskSeverity::Medium,
        "Ausencia de evaluación de niveles de iluminación (luxometrías) y factor de reflexión en planos de trabajo.",
        "Realizar estudio de iluminación cada 2 años y mantener programa de mantenimiento de luminarias.",
        250,
        5000,
    );

    // =========================================================================
    // III. ORGANIZACIÓN DEL TRABAJO (6 NOMs: 017, 018, 019, 026, 028, 030)
    // =========================================================================

    // 21. NOM-017-STPS-2008 (Equipo de Protección Personal - EPP)
    push_nom(
        "NOM-017-STPS-2008",
        "Equipo de protección personal — Selección, uso y manejo en los centros de trabajo",
        "Art. 132 Fracc. XVII LFT / Art. 994 Fracc. V LFT / NOM-017-STPS-2008",
        RiskSeverity::High,
        "Falta de matriz de análisis de riesgos por puesto de trabajo para determinar el EPP y acuses firmados de entrega.",
        "Emitir Matriz de EPP por puesto, bitácoras de entrega firmadas por trabajador y supervisión de uso.",
        250,
        5000,
    );

    // 22. NOM-018-STPS-2015 (Identificación y comunicación de peligros químicos - SGA/GHS)
    push_nom(
        "NOM-018-STPS-2015",
        "Sistema armonizado para la identificación y comunicación de peligros y riesgos por sustancias químicas (SGA)",
        "Art. 994 Fracc. V LFT / NOM-018-STPS-2015",
        RiskSeverity::Medium,
        "Envases de químicos o insumos de limpieza/mantenimiento sin etiquetado pictográfico SGA ni Hojas de Datos de Seguridad (HDS de 16 secciones).",
        "Integrar carpeta de HDS vigentes en español, listado de sustancias y señalización SGA con pictogramas de rombo.",
        250,
        5000,
    );

    // 23. NOM-019-STPS-2011 (Comisiones de Seguridad e Higiene - Obligatoria universal)
    push_nom(
        "NOM-019-STPS-2011",
        "Constitución, integración, organización y funcionamiento de las Comisiones de Seguridad e Higiene",
        "Arts. 509 y 510 LFT / Art. 994 Fracc. V LFT / NOM-019-STPS-2011",
        RiskSeverity::High,
        "Omisión del Acta Constitutiva de la Comisión Mixta de Seguridad e Higiene, programa anual de recorridos y actas trimestrales.",
        "Firmar Acta Constitutiva de la Comisión (incluida en el Kit Mallen), calendario anual y actas de verificación.",
        250,
        5000,
    );

    // 24. NOM-026-STPS-2008 (Colores y señales de seguridad y tuberías - Obligatoria universal)
    push_nom(
        "NOM-026-STPS-2008",
        "Colores y señales de seguridad e higiene, e identificación de riesgos por fluidos conducidos en tuberías",
        "Art. 994 Fracc. V LFT / NOM-026-STPS-2008",
        RiskSeverity::Medium,
        "Deficiencias en señalización de rutas de evacuación, punto de reunión, equipo contra incendio o identificación de tuberías.",
        "Adecuar señalética fotoluminiscente y bandas de dirección de flujo en tuberías conforme a código de colores oficial.",
        250,
        5000,
    );

    // 25. NOM-028-STPS-2012 (Seguridad en procesos y equipos críticos con sustancias químicas)
    if is_chemical_or_health && employee_count >= 50 {
        push_nom(
            "NOM-028-STPS-2012",
            "Sistema para la administración del trabajo — Seguridad en los procesos y equipos críticos que manejen sustancias químicas peligrosas",
            "Art. 994 Fracc. V LFT / NOM-028-STPS-2012",
            RiskSeverity::High,
            "Falta de análisis de riesgos del proceso (HAZOP/What-If) y administración de cambios en equipos críticos.",
            "Implementar sistema de integridad mecánica y auditorías internas de seguridad de procesos.",
            250,
            5000,
        );
    }

    // 26. NOM-030-STPS-2009 (Servicios preventivos de seguridad y salud - Obligatoria universal)
    push_nom(
        "NOM-030-STPS-2009",
        "Servicios preventivos de seguridad y salud en el trabajo — Funciones y actividades",
        "Art. 994 Fracc. V LFT / NOM-030-STPS-2009",
        RiskSeverity::High,
        "Falta de designación formal del Responsable de Seguridad y Salud en el Trabajo, Diagnóstico Integral y Programa Anual.",
        "Emitir nombramiento del Responsable SST y ejecutar el Diagnóstico y Programa de Seguridad y Salud (PASST).",
        250,
        5000,
    );

    // =========================================================================
    // IV. FACTORES DE RIESGO PSICOSOCIAL, ERGONÓMICO Y TELETRABAJO (3 NOMs: 035, 036-1, 037)
    // =========================================================================

    // 27. NOM-035-STPS-2018 (Factores de riesgo psicosocial - Obligatoria universal por umbrales: 1-15, 16-50, >50)
    let nom035_desc = if employee_count > 50 {
        "Centro >50 empleados: Falta de Política Psicosocial, Guías de Referencia I y III (Entorno Organizacional), buzón de quejas y canalización médica."
    } else if employee_count >= 16 {
        "Centro de 16 a 50 empleados: Falta de Política de Prevención de Riesgos Psicosociales, aplicación de Guías de Referencia I y II y buzón confidencial."
    } else {
        "Centro de 1 a 15 empleados: Falta de Política de Prevención de Riesgos Psicosociales firmada, aplicación de Guía de Referencia I (Acontecimientos Traumáticos Severos) y difusión."
    };

    push_nom(
        "NOM-035-STPS-2018",
        "Factores de riesgo psicosocial en el trabajo — Identificación, análisis y prevención",
        "Art. 994 Fracc. V LFT / NOM-035-STPS-2018",
        RiskSeverity::High,
        nom035_desc,
        "Implementar la Política de Prevención de Riesgos Psicosociales (incluida en el Kit Mallen), aplicar cuestionarios oficiales y resguardar evidencias.",
        250,
        5000,
    );

    // 28. NOM-036-1-STPS-2018 (Factores de riesgo ergonómico en manejo manual de cargas > 3 kg)
    if CalibrationAnswers::resolve_flag(cal.has_forklifts_or_manual_loads, !is_office_or_tech) {
        push_nom(
            "NOM-036-1-STPS-2018",
            "Factores de riesgo ergonómico en el Trabajo — Parte 1: Manejo manual de cargas",
            "Art. 994 Fracc. V LFT / NOM-036-1-STPS-2018",
            RiskSeverity::Medium,
            "Falta de análisis ergonómico para levantamiento, empuje, tracción o transporte manual de cargas (>3 kg recurrente) y vigilancia médica.",
            "Documentar evaluación ergonómica (Método MAC/RULA), límites de carga máxima por edad/género y capacitación postural.",
            250,
            5000,
        );
    }

    // 29. NOM-037-STPS-2023 (Teletrabajo - Condiciones de seguridad y salud)
    if CalibrationAnswers::resolve_flag(cal.has_telework_over_40_percent, is_office_or_tech) {
        push_nom(
            "NOM-037-STPS-2023",
            "Teletrabajo — Condiciones de seguridad y salud en el trabajo (Home Office >40%)",
            "Arts. 330-A a 330-K LFT / NOM-037-STPS-2023",
            RiskSeverity::High,
            "Omisión de Política de Teletrabajo (derecho a la desconexión), listado de trabajadores remotos, lista de verificación del lugar de trabajo y pago proporcional de internet/electricidad.",
            "Adoptar la Política de Teletrabajo NOM-037 (incluida en el Kit Mallen), adendas al contrato individual y checklist de ergonomía remota.",
            250,
            5000,
        );
    }

    // =========================================================================
    // V. ESPECÍFICAS DE SECTOR POR SCIAN (7 NOMs: 003, 007, 008, 016, 023, 031, 032)
    // =========================================================================

    if is_agro {
        // 30, 31 y 32: NOM-003 (Insumos fitosanitarios), NOM-007 (Maquinaria agrícola) y NOM-008 (Aprovechamiento forestal)
        push_nom(
            "NOM-003-007-008-STPS",
            "Seguridad y salud en actividades agrícolas, fitosanitarias (NOM-003), maquinaria agrícola (NOM-007) y forestales (NOM-008)",
            "Art. 994 Fracc. V LFT / NOM-003-STPS, NOM-007-STPS y NOM-008-STPS",
            RiskSeverity::High,
            "Falta de controles para manejo de plaguicidas/fertilizantes, exámenes de colinesterasa y seguridad en tractores/aserraderos.",
            "Implementar bitácoras de aplicación fitosanitaria, resguardo de agroquímicos y mantenimiento de maquinaria agrícola.",
            250,
            5000,
        );
    }

    if is_rail_transport && scian.starts_with("482") {
        // 33: NOM-016-STPS-2001 (Operación y mantenimiento de ferrocarriles)
        push_nom(
            "NOM-016-STPS-2001",
            "Operación y mantenimiento de ferrocarriles — Condiciones de seguridad e higiene",
            "Art. 994 Fracc. V LFT / NOM-016-STPS-2001",
            RiskSeverity::High,
            "Falta de procedimientos de seguridad para patio de maniobras, vías y mantenimiento de equipo tractivo.",
            "Integrar plan de atención de emergencias ferroviarias y bitácoras de inspección de vías.",
            250,
            5000,
        );
    }

    if is_mining {
        // 34 y 35: NOM-023-STPS-2012 (Minas subterráneas y cielo abierto) y NOM-032-STPS-2008 (Carbón)
        push_nom(
            "NOM-023-032-STPS",
            "Seguridad y salud en minas subterráneas, a cielo abierto (NOM-023) y minas subterráneas de carbón (NOM-032)",
            "Art. 994 Fracc. V LFT / NOM-023-STPS-2012 y NOM-032-STPS-2008",
            RiskSeverity::CriticalClosure,
            "Riesgo crítico de suspensión de obras mineras por falta de estudios geotécnicos, ventilación, control de polvos y refugios mineros.",
            "Mantener actualizados los planos de ventilación, bitácoras de explosivos y refugios con soporte vital.",
            500,
            5000,
        );
    }

    if is_construction {
        // 36: NOM-031-STPS-2011 (Construcción - Condiciones de seguridad y salud en obras)
        push_nom(
            "NOM-031-STPS-2011",
            "Construcción — Condiciones de seguridad y salud en el trabajo",
            "Art. 994 Fracc. V LFT / NOM-031-STPS-2011",
            RiskSeverity::CriticalClosure,
            "Falta de clasificación del tamaño de la obra (Pequeña, Mediana o Grande), análisis de riesgos jerarquizado y registro de obra ante STPS.",
            "Elaborar el Sistema de Administración de Seguridad y Salud en la Obra, permisos de excavación/demolición y bitácora diaria.",
            250,
            5000,
        );
    }

    findings
}
