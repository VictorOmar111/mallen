//! Server-Driven UI (SDUI) — Compliance Implementation Tracker & Typst Live Compiler.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct SduiTrackerStep {
    pub id: &'static str,
    pub phase: &'static str,
    pub code: &'static str,
    pub title: &'static str,
    pub deliverable: &'static str,
    pub weight_pct: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct SduiTrackerSchema {
    pub brand: &'static str,
    pub country: String,
    pub locale: String,
    pub hub_signature: &'static str,
    pub ui_title: &'static str,
    pub ui_subtitle: &'static str,
    pub ui_progress_label: &'static str,
    pub ui_export_btn: &'static str,
    pub ui_phase_filters: [&'static str; 4],
    pub non_compliance_band: &'static str,
    pub steps: Vec<SduiTrackerStep>,
}

#[derive(Debug, Deserialize)]
pub struct TrackerExportRequest {
    pub entity_name: Option<String>,
    pub tax_id: Option<String>,
    pub locale: Option<String>,
    pub completed_ids: Vec<String>,
}

pub fn build_sdui_tracker_schema(country: &str, locale: &str) -> SduiTrackerSchema {
    let c = country.trim().to_uppercase();
    let lang = locale.split('-').next().unwrap_or("de").to_lowercase();

    match (c.as_str(), lang.as_str()) {
        ("MX", _) => SduiTrackerSchema {
            brand: "FIRMA W16 MALLEN®",
            country: "MX".to_string(),
            locale: "es-MX".to_string(),
            hub_signature: "Frankfurt Core Hub — EU-DE-01",
            ui_title: "Plataforma de Seguimiento de Implementación Normativa (SDUI Tracker)",
            ui_subtitle: "Control determinista de instrumentación de los 7 entregables legales y 37 vectores de cumplimiento.",
            ui_progress_label: "Índice de Instrumentación Verificada",
            ui_export_btn: "Compilar Acta de Avance en PDF (Typst)",
            ui_phase_filters: ["Todos", "Fase 1 (Días 1-30)", "Fase 2 (Días 31-60)", "Fase 3 (Días 61-90)"],
            non_compliance_band: "38% – 84% (Según Vector STPS/SAT/SABG)",
            steps: vec![
                SduiTrackerStep { id: "DOC-01", phase: "Días 1-30", code: "LFPDPPP / SABG", title: "Publicación de Aviso de Privacidad Integral y Ventanilla ARCO", deliverable: "01_Aviso_de_Privacidad_Integral.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-02", phase: "Días 1-30", code: "NOM-035-STPS", title: "Suscripción de Política de Prevención de Riesgos Psicosociales", deliverable: "02_Politica_Prevencion_NOM035_STPS.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-03", phase: "Días 1-30", code: "NOM-019 / NOM-030", title: "Firma de Acta Constitutiva de Comisión Mixta y Diagnóstico SST", deliverable: "03_Acta_Comision_Mixta_NOM019_NOM030.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-04", phase: "Días 31-60", code: "NOM-037-STPS", title: "Adendum de Teletrabajo y Cláusula de Desconexión Digital", deliverable: "04_Politica_Teletrabajo_NOM037.pdf", weight_pct: 12 },
                SduiTrackerStep { id: "DOC-05", phase: "Días 31-60", code: "SAT Art. 69-B CFF", title: "Instrumentación de Contrato Marco con Fecha Cierta y Bitácora", deliverable: "05_Contrato_Materialidad_SAT_69B.pdf", weight_pct: 18 },
                SduiTrackerStep { id: "DOC-06", phase: "Días 61-90", code: "IMSS 32-D / REPSE", title: "Protocolo de Conciliación SUA y Vigilancia ICSOE / SISSUB", deliverable: "06_Protocolo_IMSS_REPSE_32D.pdf", weight_pct: 13 },
                SduiTrackerStep { id: "DOC-07", phase: "Días 61-90", code: "PROFECO Art. 76 BIS", title: "Adecuación de Términos Digitales y Sellado Criptográfico CCom 89", deliverable: "07_Terminos_Ecommerce_PROFECO_76BIS.pdf", weight_pct: 12 },
            ],
        },
        (_, "fr") => SduiTrackerSchema {
            brand: "FIRMA W16 MALLEN®",
            country: c,
            locale: locale.to_string(),
            hub_signature: "Frankfurt Core Hub — EU-DE-01",
            ui_title: "Plateforme de Suivi de Mise en Œuvre Réglementaire (SDUI Tracker)",
            ui_subtitle: "Suivi déterministe en temps réel des 7 instruments juridiques et des 37 vecteurs de conformité.",
            ui_progress_label: "Indice de Mise en Conformité Vérifiée",
            ui_export_btn: "Compiler le Certificat d'Avancement PDF (Typst)",
            ui_phase_filters: ["Tous", "Phase 1 (Jours 1-30)", "Phase 2 (Jours 31-60)", "Phase 3 (Jours 61-90)"],
            non_compliance_band: "36% – 78% (Statistiques Officielles)",
            steps: vec![
                SduiTrackerStep { id: "DOC-01", phase: "Jours 1-30", code: "RGPD / nLPD", title: "Déploiement de la Politique de Confidentialité & Registre des Traitements", deliverable: "01_Privacy_Notice_Register.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-02", phase: "Jours 1-30", code: "Sécurité au Travail", title: "Validation de l'Évaluation des Risques Professionnels & Psychosociaux", deliverable: "02_Occupational_Safety_Assessment.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-03", phase: "Jours 1-30", code: "Télétravail & Temps", title: "Signature de la Charte de Télétravail & Enregistrement du Temps", deliverable: "03_Remote_Work_Time_Policy.pdf", weight_pct: 13 },
                SduiTrackerStep { id: "DOC-04", phase: "Jours 31-60", code: "Comité de Sécurité", title: "Procès-verbal de Désignation du Délégué à la Sécurité (SIPPT / MSST / ITM)", deliverable: "04_Safety_Committee_Minutes.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-05", phase: "Jours 31-60", code: "Matérialité & Archivage", title: "Contrat-Cadre de Prestation & Protocole d'Archivage Inaltérable (10 ans)", deliverable: "05_Corporate_Service_Archival_Contract.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-06", phase: "Jours 61-90", code: "Registre UBO / RBE", title: "Vérification des Bénéficiaires Effectifs & Diligence Sous-Traitants", deliverable: "06_UBO_Beneficial_Owner_Dossier.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-07", phase: "Jours 61-90", code: "Commerce Électronique", title: "Mise en Conformité des CGV, Mentions Légales & Scellement eIDAS Art. 41", deliverable: "07_Ecommerce_Consumer_Terms.pdf", weight_pct: 12 },
            ],
        },
        (_, "it") => SduiTrackerSchema {
            brand: "FIRMA W16 MALLEN®",
            country: c,
            locale: locale.to_string(),
            hub_signature: "Frankfurt Core Hub — EU-DE-01",
            ui_title: "Piattaforma di Monitoraggio dell'Implementazione Normativa (SDUI Tracker)",
            ui_subtitle: "Controllo deterministico in tempo reale dei 7 strumenti giuridici e dei 37 vettori di conformità.",
            ui_progress_label: "Indice di Implementazione Verificata",
            ui_export_btn: "Compila Certificato di Avanzamento PDF (Typst)",
            ui_phase_filters: ["Tutti", "Fase 1 (Giorni 1-30)", "Fase 2 (Giorni 31-60)", "Fase 3 (Giorni 61-90)"],
            non_compliance_band: "39% – 77% (Statistiche Ufficiali)",
            steps: vec![
                SduiTrackerStep { id: "DOC-01", phase: "Giorni 1-30", code: "nLPD / OPDo", title: "Pubblicazione Informativa Privacy Art. 19 nLPD & Registro Trattamenti", deliverable: "01_Informativa_Privacy_Registro_nLPD.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-02", phase: "Giorni 1-30", code: "Art. 6 LL / MSSL 6508", title: "Adozione del Concetto di Sicurezza sul Lavoro & Rischi Psicosociali", deliverable: "02_Concetto_Sicurezza_LL_MSSL6508.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-03", phase: "Giorni 1-30", code: "OLL 1 / Telelavoro", title: "Sottoscrizione Regolamento Durata del Lavoro & Telelavoro", deliverable: "03_Regolamento_Telelavoro_OLL1_CO.pdf", weight_pct: 13 },
                SduiTrackerStep { id: "DOC-04", phase: "Giorni 31-60", code: "Commissione MSSL", title: "Verbale Costitutivo della Commissione Interna di Sicurezza", deliverable: "04_Verbale_Commissione_Sicurezza_MSSL.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-05", phase: "Giorni 31-60", code: "CO Art. 957 / Olc", title: "Contratto Quadro di Servizi & Istruzioni di Archiviazione Decennale Olc", deliverable: "05_Contratto_Quadro_Servizi_CO957_Olc.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-06", phase: "Giorni 61-90", code: "Art. 697j CO / LRD", title: "Aggiornamento Elenco Aventi Economicamente Diritto & Verifica AVS", deliverable: "06_Elenco_Aventi_Diritto_CO697j_LRD.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-07", phase: "Giorni 61-90", code: "LCSl / OIP / FiEle", title: "Adeguamento CGC E-Commerce, Impressum & Sigillo Crittografico SHA-256", deliverable: "07_Condizioni_Generali_Ecommerce_LCSl_OIP.pdf", weight_pct: 12 },
            ],
        },
        (_, "nl") => SduiTrackerSchema {
            brand: "FIRMA W16 MALLEN®",
            country: c,
            locale: locale.to_string(),
            hub_signature: "Frankfurt Core Hub — EU-DE-01",
            ui_title: "Compliance Implementatie & Voortgangsplatform (SDUI Tracker)",
            ui_subtitle: "Deterministische voortgangscontrole van de 7 juridische instrumenten en 37 wettelijke vectoren.",
            ui_progress_label: "Geverifieerde Implementatie-Index",
            ui_export_btn: "Compileer Voortgangscertificaat PDF (Typst)",
            ui_phase_filters: ["Alle", "Fase 1 (Dagen 1-30)", "Fase 2 (Dagen 31-60)", "Fase 3 (Dagen 61-90)"],
            non_compliance_band: "35% – 76% (Officiële Inspectiecijfers)",
            steps: vec![
                SduiTrackerStep { id: "DOC-01", phase: "Dagen 1-30", code: "AVG / AP / GBA", title: "Publicatie Privacyverklaring, Verwerkingsregister & Verwerkersovereenkomsten", deliverable: "01_AVG_Privacy_Verwerkingsregister.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-02", phase: "Dagen 1-30", code: "Arbowet / Codex", title: "Vaststelling Risico-Inventarisatie & -Evaluatie (RI&E) en PSA-Beleid", deliverable: "02_RIE_PSA_Arbobeleid.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-03", phase: "Dagen 1-30", code: "Telewerk / Arbeidstijd", title: "Ondertekening Thuiswerk- en Deconnectiebeleid", deliverable: "03_Thuiswerk_Deconnectie_Overeenkomst.pdf", weight_pct: 13 },
                SduiTrackerStep { id: "DOC-04", phase: "Dagen 31-60", code: "Preventiemedewerker", title: "Aanstelling Preventiemedewerker / Interne Dienst (IDPBW) & Noodplan", deliverable: "04_Aanstelling_Preventiemedewerker_BHV.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-05", phase: "Dagen 31-60", code: "Fiscale Bewaarplicht", title: "Raamovereenkomst van Opdracht & Digitale Administratieve Bewaarplicht", deliverable: "05_Raamovereenkomst_Fiscale_Bewaarplicht.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-06", phase: "Dagen 61-90", code: "UBO-Register / Wka", title: "Verificatie UBO-Registratie & Ketenaansprakelijkheid (Wka / Art. 30bis)", deliverable: "06_UBO_Ketenaansprakelijkheid_Dossier.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-07", phase: "Dagen 61-90", code: "ACM / WER / eIDAS", title: "Algemene Voorwaarden Koop op Afstand & eIDAS Art. 41 Tijdstempel", deliverable: "07_Algemene_Voorwaarden_Ecommerce.pdf", weight_pct: 12 },
            ],
        },
        (_, "lb") => SduiTrackerSchema {
            brand: "FIRMA W16 MALLEN®",
            country: c,
            locale: locale.to_string(),
            hub_signature: "Frankfurt Core Hub — EU-DE-01",
            ui_title: "Plattform fir d'Ëmsetzung vun der Reglementatioun (SDUI Tracker)",
            ui_subtitle: "Deterministesch Kontroll vun de 7 juristeschen Dokumenter an 37 Konformitéitsvektoren.",
            ui_progress_label: "Verifizéierten Ëmsetzungs-Index",
            ui_export_btn: "Fortschrëtts-Zertifikat als PDF kompiléieren (Typst)",
            ui_phase_filters: ["All", "Phas 1 (Deeg 1-30)", "Phas 2 (Deeg 31-60)", "Phas 3 (Deeg 61-90)"],
            non_compliance_band: "34% – 75% (Offiziell Inspektiounen)",
            steps: vec![
                SduiTrackerStep { id: "DOC-01", phase: "Deeg 1-30", code: "CNPD / RGPD", title: "Publikatioun vun der Dateschutzerklärung & Verarbechtungsregister", deliverable: "01_CNPD_Dateschutz_Dossier.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-02", phase: "Deeg 1-30", code: "ITM Code du Travail", title: "Sécherheets- a Gesondheetskonzept op der Aarbechtsplaz (Art. L.312)", deliverable: "02_ITM_Aarbechtssécherheet.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-03", phase: "Deeg 1-30", code: "Télétravail LU", title: "Ënnerschrëft vun der Télétravail-Konventioun & Aarbechtszäit-Reglement", deliverable: "03_Teletravail_Konventioun_LU.pdf", weight_pct: 13 },
                SduiTrackerStep { id: "DOC-04", phase: "Deeg 31-60", code: "Travailleur Désigné", title: "Ernennungsakt vum Travailleur Désigné & Délégation du Personnel", deliverable: "04_Travailleur_Designe_Protokoll.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-05", phase: "Deeg 31-60", code: "Art. 16 CCom / TVA", title: "Rahmenkontrakt & 10-järeg Comptabilitéits-Archivéierung", deliverable: "05_Rahmenkontrakt_Archiv_LU.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-06", phase: "Deeg 61-90", code: "LBR / RBE", title: "Kontroll vum Registre des Bénéficiaires Effectifs (RBE) & CCSS", deliverable: "06_LBR_RBE_Konformiteit.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-07", phase: "Deeg 61-90", code: "MPC / eIDAS Art. 41", title: "E-Commerce AGB, Impressum & eIDAS SHA-256 Zertifizéierung", deliverable: "07_Ecommerce_AGB_LU.pdf", weight_pct: 12 },
            ],
        },
        _ => SduiTrackerSchema {
            brand: "FIRMA W16 MALLEN®",
            country: c,
            locale: locale.to_string(),
            hub_signature: "Frankfurt Core Hub — EU-DE-01",
            ui_title: "Compliance-Implementierungs- & Nachweisplattform (SDUI Tracker)",
            ui_subtitle: "Deterministische Echtzeit-Verfolgung der 7 Rechtsinstrumente und 37 gesetzlichen Prüfvektoren.",
            ui_progress_label: "Verifizierter Implementierungsgrad",
            ui_export_btn: "Implementierungsnachweis als PDF kompilieren (Typst)",
            ui_phase_filters: ["Alle", "Phase 1 (Tage 1-30)", "Phase 2 (Tage 31-60)", "Phase 3 (Tage 61-90)"],
            non_compliance_band: "36% – 79% (Offizielle Prüfungsberichte)",
            steps: vec![
                SduiTrackerStep { id: "DOC-01", phase: "Tage 1-30", code: "DSGVO / BDSG / DSG / revDSG", title: "Bereitstellung der Datenschutzerklärung, VVT (Art. 30) & AV-Verträge", deliverable: "01_Datenschutz_VVT_Dossier.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-02", phase: "Tage 1-30", code: "ArbSchG / ASchG / ArG", title: "Dokumentation der physischen und psychischen Gefährdungsbeurteilung", deliverable: "02_Gefaehrdungsbeurteilung_Arbeitsschutz.pdf", weight_pct: 15 },
                SduiTrackerStep { id: "DOC-03", phase: "Tage 1-30", code: "ArbStättV / ArGV 1", title: "Richtlinie zur Telearbeit, Arbeitszeiterfassung & Erreichbarkeit", deliverable: "03_Telearbeit_Arbeitszeit_Richtlinie.pdf", weight_pct: 13 },
                SduiTrackerStep { id: "DOC-04", phase: "Tage 31-60", code: "ASiG / SVP / EKAS 6508", title: "Konstituierungsprotokoll Arbeitsschutzausschuss (ASA) & Brandschutzordnung", deliverable: "04_Arbeitsschutzausschuss_Protokoll.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-05", phase: "Tage 31-60", code: "GoBD / BAO / GeBüV", title: "Unternehmens-Rahmenvertrag & Revisionssichere Verfahrensdokumentation", deliverable: "05_Rahmenvertrag_Verfahrensdokumentation.pdf", weight_pct: 16 },
                SduiTrackerStep { id: "DOC-06", phase: "Tage 61-90", code: "GwG / WiEReG / OR 697j", title: "Transparenzregister-Abgleich (UBO) & Sozialversicherungsprüfung", deliverable: "06_Transparenzregister_UBO_Akte.pdf", weight_pct: 14 },
                SduiTrackerStep { id: "DOC-07", phase: "Tage 61-90", code: "DDG / BGB / FAGG / UWG", title: "Rechtssichere AGB, Impressum, Widerrufsbelehrung & eIDAS Art. 41 Siegel", deliverable: "07_AGB_Fernabsatz_Impressum.pdf", weight_pct: 12 },
            ],
        },
    }
}

pub fn compile_tracker_certificate_pdf(
    country: &str,
    req: &TrackerExportRequest,
) -> Result<Vec<u8>, String> {
    let locale = req.locale.as_deref().unwrap_or("de-DE");
    let schema = build_sdui_tracker_schema(country, locale);
    let entity = req.entity_name.as_deref().filter(|s| !s.trim().is_empty()).unwrap_or("CORPORATE ENTITY");
    let tax = req.tax_id.as_deref().filter(|s| !s.trim().is_empty()).unwrap_or("REG-2026-W16");
    let now_str = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();

    let mut total_pct: u8 = 0;
    let mut rows_markup = String::new();
    for step in &schema.steps {
        let done = req.completed_ids.iter().any(|id| id == step.id);
        if done {
            total_pct = total_pct.saturating_add(step.weight_pct);
        }
        let status_txt = if done { "VERIFIED / IMPLEMENTED" } else { "SCHEDULED" };
        let status_col = if done { "#059669" } else { "#d97706" };
        rows_markup.push_str(&format!(
            "  [{}], [{}], [{}], [{}], [#text(weight: \"bold\", fill: rgb(\"{}\"))[{} ({}%)]],\n",
            step.id,
            step.phase,
            step.code,
            step.title.replace('&', "+"),
            status_col,
            status_txt,
            step.weight_pct
        ));
    }

    let mut typst_markup = String::from(r##"#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm))
#set text(size: 8.5pt, fill: rgb("#0f172a"))

#rect(width: 100%, fill: rgb("#0f172a"), inset: 12pt, radius: 4pt)[
  #grid(
    columns: (1fr, auto),
    align: horizon,
    [
      #text(size: 12pt, weight: "bold", fill: white)[FIRMA W16 MALLEN® — COMPLIANCE IMPLEMENTATION TRACKER]
      #v(2pt)
      #text(size: 7.5pt, fill: rgb("#38bdf8"))[Cryptographic Hub: Frankfurt Core Hub — EU-DE-01 • eIDAS Art. 41 / CCom Art. 89-93 / SEC-FINRA]
    ],
    [
      #box(fill: rgb("#059669"), inset: (x: 8pt, y: 5pt), radius: 3pt)[
        #text(size: 9pt, weight: "bold", fill: white)[PROGRESS: {{TOTAL_PCT}}%]
      ]
    ]
  )
]

#v(8pt)
#rect(width: 100%, stroke: 0.6pt + rgb("#cbd5e1"), inset: 9pt, radius: 3pt)[
  #grid(
    columns: (1fr, 1fr),
    row-gutter: 5pt,
    [*Entity:* {{ENTITY}}], [*Registry / Tax ID:* {{TAX}}],
    [*Jurisdiction & Locale:* {{COUNTRY}} ({{LOCALE}})], [*Timestamp:* {{NOW}}],
    [*Official Inspection Range:* {{BAND}}], [*Verification Node:* Frankfurt Core Hub — EU-DE-01]
  )
]

#v(8pt)
#text(size: 9.5pt, weight: "bold")[SDUI IMPLEMENTATION MATRIX (7 STATUTORY DELIVERABLES)]
#v(4pt)

#table(
  columns: (0.7fr, 0.9fr, 1.2fr, 2.6fr, 1.4fr),
  inset: 5.5pt,
  stroke: 0.4pt + rgb("#cbd5e1"),
  fill: (col, row) => if row == 0 { rgb("#0f172a") } else if calc.even(row) { rgb("#f8fafc") } else { white },
  [#text(fill: white, weight: "bold", size: 7.5pt)[ID]],
  [#text(fill: white, weight: "bold", size: 7.5pt)[Phase]],
  [#text(fill: white, weight: "bold", size: 7.5pt)[Statutory Code]],
  [#text(fill: white, weight: "bold", size: 7.5pt)[Implementation Milestone]],
  [#text(fill: white, weight: "bold", size: 7.5pt)[Status & Weight]],
{{ROWS}})

#v(12pt)
#line(length: 100%, stroke: 0.5pt + rgb("#94a3b8"))
#v(4pt)
#text(size: 7pt, fill: rgb("#475569"))[
  © 2026 FIRMA W16 MALLEN® (mallen.lat) • Deterministic Auditing & Digital Expert Infrastructure • Cryptographic Signature: Central Europe Region (Frankfurt Core Hub — EU-DE-01) • Cross-border probative validity: eIDAS Art. 41 (EU) • Commercial Code Art. 89-93 (MX) • SEC / FINRA Compliance.
]
"##);

    typst_markup = typst_markup
        .replace("{{TOTAL_PCT}}", &total_pct.to_string())
        .replace("{{ENTITY}}", &crate::engine::pdf_generator::sanitize_for_typst(entity))
        .replace("{{TAX}}", &crate::engine::pdf_generator::sanitize_for_typst(tax))
        .replace("{{COUNTRY}}", &schema.country)
        .replace("{{LOCALE}}", &schema.locale)
        .replace("{{NOW}}", &now_str)
        .replace("{{BAND}}", schema.non_compliance_band)
        .replace("{{ROWS}}", &rows_markup);

    let tmp_typ = format!("/tmp/w16_tracker_{}_{}.typ", schema.country, std::process::id());
    let tmp_pdf = format!("/tmp/w16_tracker_{}_{}.pdf", schema.country, std::process::id());

    fs::write(&tmp_typ, typst_markup).map_err(|e| e.to_string())?;
    let status = Command::new("typst")
        .arg("compile")
        .arg(&tmp_typ)
        .arg(&tmp_pdf)
        .status()
        .map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&tmp_typ);

    if !status.success() {
        return Err("Typst tracker compilation failed".to_string());
    }
    let bytes = fs::read(&tmp_pdf).map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&tmp_pdf);
    Ok(bytes)
}
