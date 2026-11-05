/**
 * FIRMA W16 MALLEN® — Selector Inicial de Bloque/País/Idioma, Consentimiento Local y Portales Regionales
 * - Cero emojis en toda la interfaz.
 * - Estructura limpia por bloque: LATAM, DACH y BENELUX en grande, y debajo cada país sin repetir su nombre.
 * - Flujo estricto:
 *     1) Selección de País e Idioma Regional.
 *     2) Aceptación de Políticas de Privacidad y Cookies del país en su propio idioma.
 *     3) Acceso al Portal del País con sus leyes y placeholders en su idioma.
 */
(function () {
    "use strict";

    const REGIONAL_PORTAL_DATA = {
        "es-MX": {
            country: "MX", block: "LATAM", name: "México", langName: "Español (es-MX)", currency: "MXN (\$)",
            privacyTitle: "Aviso de Privacidad Integral, Política de Cookies Técnicas y Alcance Pericial (LFPDPPP / SABG)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat), en cumplimiento con la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP vigente ante la SABG) y el Código de Comercio (Arts. 89 a 93), informa que este sistema utiliza exclusivamente cookies técnicas de sesión y ejecuta compulsas OSINT sobre padrones públicos (INEGI DENUE, SAT Art. 69-B, STPS REPSE, IMSS e INAI/SABG) sin almacenar datos personales sensibles.",
            cookieCheckLabel: "He leído y acepto el Aviso de Privacidad Integral (LFPDPPP), la Política de Cookies Técnicas y los Términos de Alcance Pericial para México.",
            enterBtn: "Ingresar al Portal de México"
        },
        "de-DE": {
            country: "DE", block: "DACH", name: "Deutschland", langName: "Deutsch (de-DE)", currency: "EUR (€)",
            kitPrice: "290,00 €", simulatedFine: "148.500,00 €", templatePath: "templates/dach/de_de",
            heroBadge: "SACHVERSTÄNDIGEN-UNTERNEHMENSAUDIT • 37 COMPLIANCE-VEKTOREN (DEUTSCHLAND)",
            heroTitle: "Haftungsdiagnose, Bußgeldprävention & 360° Corporate Shield",
            heroSubtitle: "Automatisierter Abgleich nach deutschem Bundesrecht: DSGVO/BDSG, ArbSchG (Gefährdungsbeurteilung), GoBD, Transparenzregister (GwG), LkSG, NIS-2 und DDG.",
            taxLabel: "USt-IdNr. / Handelsregister-Nummer (HRB) • WZ 2008",
            taxPlaceholder: "USt-IdNr. oder HRB eingeben (z.B. DE123456789)",
            domainPlaceholder: "Unternehmens-Domain (z.B. unternehmen.de)",
            scanBtn: "Compliance-Audit starten",
            cryptoLaw: "eIDAS-VO (EU) Nr. 910/2014 Art. 41 • § 371a ZPO",
            fineNote: "Berechnung gemäß Art. 83 DSGVO, § 25 ArbSchG, § 30 OWiG und BAFA-Bußgeldkatalog.",
            privacyTitle: "Datenschutzerklärung, Cookie-Richtlinie (§ 25 TDDDG) & Prüfungsbedingungen (DSGVO / BDSG)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) verwendet im Rahmen dieses Compliance-Systems ausschließlich technisch notwendige Sitzungs-Cookies gemäß § 25 Abs. 2 Nr. 2 TDDDG sowie öffentlich zugängliche Unternehmensregisterdaten (Handelsregister, Bundesanzeiger, TLS/SSL-Telemetrie) gemäß Art. 6 Abs. 1 lit. f DSGVO und BDSG.",
            cookieCheckLabel: "Ich habe die Datenschutzerklärung nach DSGVO/BDSG sowie die Cookie-Richtlinie (§ 25 TDDDG) gelesen und akzeptiere diese für Deutschland.",
            enterBtn: "Weiter zum Deutschen Compliance-Portal",
            sectionTitle: "Gesetzliche Prüfungsmodule für Deutschland (6 Hauptsäulen • 37 Vektoren)",
            sectionSub: "Strukturierte Vorab-Platzhalter nach deutschem Bundesrecht (Vollständige Rechtsdokumente im Dossier enthalten)",
            deliverables: [
                "01. Datenschutzerklärung & Verzeichnis (VVT) nach Art. 13, 14, 30 DSGVO / BDSG",
                "02. Dokumentation der Gefährdungsbeurteilung (inkl. psychischer Belastung) nach §§ 5, 6 ArbSchG",
                "03. Richtlinie zur Telearbeit & Mobilen Arbeit nach ArbStättV und ArbZG",
                "04. Konstituierungsprotokoll des Arbeitsschutzausschusses (ASA) nach § 11 ASiG",
                "05. Unternehmens-Rahmenvertrag mit GoBD-Verfahrensdokumentation & LkSG-Klausel",
                "06. GwG-Compliance-Protokoll & Transparenzregister-Dokumentation (§§ 18-20 GwG)",
                "07. Rechtssichere AGB, Impressum (§ 5 DDG) & Widerrufsbelehrung (§ 312d BGB)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "DSGVO & BDSG • BfDI", title: "1. Datenschutz & TDDDG-Telemetrie", ref: "Art. 13, 14, 30, 32 DSGVO • § 26 BDSG • § 25 TDDDG", desc: "Prüfung der Datenschutzerklärung, Verzeichnis von Verarbeitungstätigkeiten (VVT), Auftragsverarbeitung (AVV), TLS 1.3 und Beschäftigtendatenschutz.", status: "Platzhalter: AVV & Datenschutzerklärung (DE)" },
                { color: "border-blue-500 text-blue-400", badge: "BMAS • ArbSchG & DGUV", title: "2. Arbeitsschutz & Gefährdungsbeurteilung", ref: "§§ 5, 6 ArbSchG • ArbStättV • § 11 ASiG", desc: "Dokumentation der physischen und psychischen Gefährdungsbeurteilung, Arbeitsschutzausschuss (ASA) und Bildschirmarbeitsplätze.", status: "Platzhalter: Gefährdungsbeurteilung § 5 ArbSchG" },
                { color: "border-emerald-500 text-emerald-400", badge: "Finanzamt • AO & GoBD", title: "3. Steuerliche Materialität & GoBD", ref: "§§ 146, 147 AO • GoBD • § 14 UStG", desc: "Revisionssichere elektronische Buchführung, Verfahrensdokumentation nach GoBD und Vorsteuerabzugssicherung im Geschäftsverkehr.", status: "Platzhalter: GoBD-Verfahrensdokumentation" },
                { color: "border-purple-500 text-purple-400", badge: "Bundesanzeiger • GwG", title: "4. Geldwäschegesetz & Transparenzregister", ref: "§§ 18–20 GwG • § 28a SGB IV", desc: "Eintragungspflicht der wirtschaftlich Berechtigten im deutschen Transparenzregister, Sozialversicherungsabgleich und KYC-Sorgfaltspflichten.", status: "Platzhalter: GwG & Transparenzregister-Akte" },
                { color: "border-amber-500 text-amber-400", badge: "BSI & BAFA • NIS-2 & LkSG", title: "5. Cybersicherheit (NIS-2) & Lieferkette", ref: "BSIG (NIS-2) • LkSG • ASR A2.2 Brandschutz", desc: "IT-Sicherheitsrichtlinie und Meldepflichten gegenüber dem BSI, Sorgfaltspflichten in der Lieferkette (BAFA) sowie betrieblicher Brandschutz.", status: "Platzhalter: NIS-2 & Brandschutz-Konzept" },
                { color: "border-teal-500 text-teal-400", badge: "BGB & UWG • Fernabsatz", title: "6. Impressum (§ 5 DDG) & Verbraucherrecht", ref: "§ 5 DDG • §§ 312d, 355 BGB • PAngV • UWG", desc: "Abmahnprävention nach UWG: Rechtssicheres Impressum, 14-tägige Widerrufsbelehrung, Preisangabenverordnung und AGB-Kontrolle.", status: "Platzhalter: AGB & Widerrufsbelehrung (DE)" }
            ]
        },
        "de-AT": {
            country: "AT", block: "DACH", name: "Österreich", langName: "Deutsch (de-AT)", currency: "EUR (€)",
            kitPrice: "290,00 €", simulatedFine: "136.000,00 €", templatePath: "templates/dach/de_at",
            heroBadge: "SACHVERSTÄNDIGEN-AUDIT • 37 COMPLIANCE-VEKTOREN (ÖSTERREICH)",
            heroTitle: "Verwaltungsstrafen-Prävention & 360° Compliance-Schutzschild",
            heroSubtitle: "Deterministische Prüfung nach österreichischer Bundesrechtslage: DSB (DSGVO/DSG), ASchG Arbeitsplatzevaluierung, BAO, WiEReG, NISG, FAGG und ECG.",
            taxLabel: "UID-Nummer (ATU) / Firmenbuchnummer (FN) • ÖNACE 2008",
            taxPlaceholder: "UID-Nummer oder FN eingeben (z.B. ATU12345678)",
            domainPlaceholder: "Unternehmens-Domain (z.B. firma.at)",
            scanBtn: "Sachverständigen-Prüfung starten",
            cryptoLaw: "eIDAS-VO Art. 41 • SVG (Signatur- und Vertrauensdienstegesetz)",
            fineNote: "Bemessung nach österreichischem VStG (Verwaltungsstrafgesetz), § 130 ASchG, WiEReG und Art. 83 DSGVO.",
            privacyTitle: "Datenschutzerklärung, Cookie-Richtlinie (§ 165 TKG 2021) & Prüfungsbedingungen (DSG Österreich)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) setzt ausschließlich technisch erforderliche Sitzungs-Cookies gemäß § 165 Abs. 3 TKG 2021 ein und verarbeitet öffentliche Firmenbuch- und UID-Registerdaten im Einklang mit der DSGVO und dem österreichischen Datenschutzgesetz (DSG — Datenschutzbehörde DSB).",
            cookieCheckLabel: "Ich akzeptiere die österreichische Datenschutzerklärung (DSG/DSGVO) sowie die Cookie-Richtlinie nach § 165 TKG 2021.",
            enterBtn: "Weiter zum Österreichischen Compliance-Portal",
            sectionTitle: "Gesetzliche Prüfungsmodule für Österreich (6 Hauptsäulen • 37 Vektoren)",
            sectionSub: "Strukturierte Vorab-Platzhalter nach österreichischem Recht (DSG / ASchG / BAO / WiEReG / FAGG)",
            deliverables: [
                "01. Datenschutzmitteilung & Verarbeitungsverzeichnis nach DSGVO / österreichischem DSG",
                "02. Sicherheits- und Gesundheitsschutzdokument (Arbeitsplatzevaluierung) nach §§ 4, 5 ASchG",
                "03. Homeoffice-Vereinbarung nach österreichischem Homeoffice-Gesetz (AVRAG)",
                "04. Bestellungsurkunde für Sicherheitsvertrauenspersonen (SVP) & Arbeitsschutzausschuss",
                "05. Unternehmens-Werkvertrag mit Aufzeichnungsprotokoll nach §§ 131, 132 BAO & UGB",
                "06. WiEReG-Compliance-Dossier (Wirtschaftliche Eigentümer Registergesetz) & ÖGK-Prüfung",
                "07. Österreichische AGB, Offenlegung (§ 5 ECG / § 25 MedienG) & Rücktrittsbelehrung (FAGG)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "DSB • DSGVO & DSG", title: "1. Österreichisches Datenschutzrecht & TKG", ref: "Art. 13/32 DSGVO • § 1 DSG • § 165 TKG 2021", desc: "Prüfung der Datenschutzmitteilung, Verarbeitungsverzeichnis und technischen Sicherheitsmaßnahmen vor der Österreichischen Datenschutzbehörde (DSB).", status: "Platzhalter: DSB-Datenschutzakte (AT)" },
                { color: "border-blue-500 text-blue-400", badge: "Arbeitsinspektorat • ASchG", title: "2. ArbeitnehmerInnenschutz & Evaluierung", ref: "§§ 4, 5, 130 ASchG • AUVA", desc: "Verpflichtende Arbeitsplatzevaluierung (inkl. psychischer Fehlbelastungen), Sicherheitsvertrauenspersonen (SVP) und Homeoffice-Regelung.", status: "Platzhalter: ASchG Sicherheitsdokument" },
                { color: "border-emerald-500 text-emerald-400", badge: "Finanzamt • BAO & UGB", title: "3. Bundesabgabenordnung & Buchführung", ref: "§§ 131, 132 BAO • § 190 UGB • RKSV", desc: "Ordnungsmäßigkeit der elektronischen Aufzeichnungen, 7-jährige Belegaufbewahrungspflicht und Haftungsvorsorge bei Auftragsvergabe.", status: "Platzhalter: BAO-Aufzeichnungsprotokoll" },
                { color: "border-purple-500 text-purple-400", badge: "BMF • WiEReG & ÖGK", title: "4. Wirtschaftliche Eigentümer (WiEReG) & ASVG", ref: "WiEReG • ASVG / ÖGK • LSD-BG", desc: "Jährliche Überprüfung und Meldung im österreichischen WiEReG-Register sowie Lohn- und Sozialdumping-Prävention (LSD-BG).", status: "Platzhalter: WiEReG & LSD-BG Dossier" },
                { color: "border-amber-500 text-amber-400", badge: "NISG & GewO • Betriebsanlagen", title: "5. Netz- und Informationssicherheit & GewO", ref: "NISG • §§ 74 ff. GewO 1994 • TRVB", desc: "Cybersicherheitsvorgaben nach österreichischem NISG sowie gewerberechtliche Betriebsanlagengenehmigungen und Brandschutz (TRVB).", status: "Platzhalter: NISG & Brandschutz-Akte" },
                { color: "border-teal-500 text-teal-400", badge: "KSchG, FAGG & ECG", title: "6. Konsumentenschutz & E-Commerce (ECG)", ref: "§ 5 ECG • § 25 MedienG • FAGG • KSchG", desc: "Offenlegungspflichten nach ECG/MedienG, Rücktrittsrechte nach FAGG und Klauselprüfung nach dem Konsumentenschutzgesetz (KSchG).", status: "Platzhalter: Österreichische AGB & FAGG" }
            ]
        },
        "de-CH": {
            country: "CH", block: "DACH", name: "Schweiz", langName: "Deutsch (de-CH)", currency: "CHF (Fr.)",
            kitPrice: "CHF 290.00", simulatedFine: "CHF 165'000.00", templatePath: "templates/dach/de_ch",
            heroBadge: "EXPERTEN-AUDIT • 37 SCHWEIZER COMPLIANCE-VEKTOREN (CH)",
            heroTitle: "Haftungs- & Bussenprävention nach Schweizer Bundesrecht (360°)",
            heroSubtitle: "Prüfung gemäss revidiertem Datenschutzgesetz (revDSG / EDÖB), Arbeitsgesetz (ArG / SUVA / EKAS 6508), OR Art. 957 (GeBüV), GAFI und UWG.",
            taxLabel: "UID-Nummer (CHE-XXX.XXX.XXX) • NOGA 2008",
            taxPlaceholder: "UID-Nummer eingeben (z.B. CHE-105.805.649)",
            domainPlaceholder: "Unternehmens-Domain (z.B. unternehmen.ch)",
            scanBtn: "Schweizer Compliance-Audit starten",
            cryptoLaw: "ZertES (Bundesgesetz über die elektronische Signatur) • OR Art. 957",
            fineNote: "Bussenberechnung nach Art. 60 ff. revDSG (persönliche Strafbarkeit bis CHF 250'000), ArG und Schweizer Obligationenrecht (OR).",
            privacyTitle: "Datenschutzerklärung & Cookie-Information gemäss Schweizer revDSG / FMG",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) bearbeitet im Rahmen dieser Plattform ausschliesslich technisch notwendige Sitzungsdaten gemäss Art. 45c FMG sowie öffentlich zugängliche Zefix/UID-Handelsregisterdaten nach dem revidierten Schweizer Datenschutzgesetz (revDSG — EDÖB).",
            cookieCheckLabel: "Ich akzeptiere die Datenschutzerklärung gemäss Schweizer revDSG sowie die Cookie-Richtlinie (Art. 45c FMG).",
            enterBtn: "Weiter zum Schweizer Compliance-Portal (de-CH)",
            sectionTitle: "Gesetzliche Prüfungsmodule für die Schweiz (6 Hauptsäulen • 37 Vektoren)",
            sectionSub: "Strukturierte Vorab-Platzhalter nach Schweizer Bundesrecht (revDSG / ArG / OR / GwG / UWG)",
            deliverables: [
                "01. Datenschutzerklärung & Bearbeitungsverzeichnis gemäss Art. 12, 19 revDSG (EDÖB)",
                "02. Betriebliches Sicherheitskonzept gemäss ArG Art. 6, UVG Art. 82 & EKAS-Richtlinie 6508",
                "03. Homeoffice- und Arbeitszeiterfassungsreglement nach Schweizer OR & ArGV 1",
                "04. Konstituierungsprotokoll der Arbeitssicherheitskommission (ASA / SUVA)",
                "05. Unternehmens-Dienstleistungsvertrag & Archivierungskonzept nach OR Art. 957 und GeBüV",
                "06. Verzeichnis der wirtschaftlich berechtigten Personen (Art. 697j OR / GAFI) & AHV/SUVA-Abgleich",
                "07. Schweizer E-Commerce AGB nach Art. 3 lit. s UWG & Preisbekanntgabeverordnung (PBV)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "EDÖB • revDSG & DSV", title: "1. Revidiertes Datenschutzgesetz (revDSG)", ref: "Art. 8, 12, 19, 60 revDSG • DSV", desc: "Schutz vor persönlichen Bussen bis CHF 250'000 (Art. 60 revDSG): Datenschutzerklärung, Bearbeitungsverzeichnis, Bekanntgabe ins Ausland und Datensicherheit.", status: "Platzhalter: Schweizer revDSG-Dossier" },
                { color: "border-blue-500 text-blue-400", badge: "SECO & SUVA • ArG / UVG", title: "2. Arbeitsgesetz (ArG) & EKAS-Richtlinie 6508", ref: "Art. 6 ArG • Art. 82 UVG • EKAS 6508", desc: "Gesundheitsschutz und Schutz der persönlichen Integrität am Arbeitsplatz, Arbeitszeiterfassung (ArGV 1) und Beizug von Arbeitsärzten (ASA).", status: "Platzhalter: EKAS 6508 & ArG-Reglement" },
                { color: "border-emerald-500 text-emerald-400", badge: "ESTV • OR Art. 957 & GeBüV", title: "3. Obligationenrecht (OR) & Geschäftsbücher", ref: "Art. 957–958f OR • GeBüV • MWSTG", desc: "Unveränderbare 10-jährige Aufbewahrung der Geschäftsbücher gemäss Geschäftsbücherverordnung (GeBüV) und MWST-Abstimmung.", status: "Platzhalter: GeBüV-Archivierungsrichtlinie" },
                { color: "border-purple-500 text-purple-400", badge: "GAFI / GwG • Art. 697j OR", title: "4. Wirtschaftlich Berechtigte & Sozialwerke", ref: "Art. 697j–697m OR • GwG • AHV/SUVA", desc: "Verzeichnis der wirtschaftlich berechtigten Personen nach Schweizer OR, Sorgfaltspflichten nach GwG und AHV/SUVA-Lohnsummenabgleich.", status: "Platzhalter: Art. 697j OR & GwG-Register" },
                { color: "border-amber-500 text-amber-400", badge: "BACS / NCSC • ISG & VKF", title: "5. Informationssicherheitsgesetz (ISG) & VKF", ref: "ISG • Meldepflicht BACS • VKF-Brandschutz", desc: "Cyber-Risikomanagement nach Vorgaben des Bundesamts für Cybersicherheit (BACS) und kantonale VKF-Brandschutzvorschriften.", status: "Platzhalter: ISG & VKF Sicherheitskonzept" },
                { color: "border-teal-500 text-teal-400", badge: "SECO • UWG & PBV", title: "6. Lauterkeitsrecht (UWG) & E-Commerce", ref: "Art. 3 lit. s UWG • Art. 8 UWG • PBV", desc: "Pflichtangaben im elektronischen Geschäftsverkehr (Art. 3 lit. s UWG), Preisbekanntgabeverordnung (PBV) und Kontrolle missbräuchlicher AGB.", status: "Platzhalter: Schweizer E-Commerce AGB" }
            ]
        },
        "fr-CH": {
            country: "CH", block: "DACH", name: "Suisse", langName: "Français (fr-CH)", currency: "CHF (Fr.)",
            kitPrice: "CHF 290.00", simulatedFine: "CHF 165'000.00", templatePath: "templates/dach/fr_ch",
            heroBadge: "AUDIT D'EXPERTISE • 37 VECTEURS DE CONFORMITÉ SUISSES (CH)",
            heroTitle: "Diagnostic des Risques Réglementaires & Blindage Juridique 360° (Suisse)",
            heroSubtitle: "Évaluation selon le droit fédéral suisse : nouvelle Loi sur la protection des données (nLPD / PFPDT), Loi sur le travail (LTr / SUVA / MSST 6508), CO art. 957 (Olico), GAFI et LCD.",
            taxLabel: "Numéro IDE / UID (CHE-XXX.XXX.XXX) • NOGA 2008",
            taxPlaceholder: "Saisir le numéro IDE (ex. CHE-105.805.649)",
            domainPlaceholder: "Domaine Web (ex. entreprise.ch)",
            scanBtn: "Lancer l'Audit de Conformité Suisse",
            cryptoLaw: "SCSE / ZertES (Signature électronique suisse) • CO art. 957",
            fineNote: "Calcul des amendes selon les art. 60 ss nLPD (responsabilité pénale jusqu'à CHF 250'000), LTr et Code des obligations.",
            privacyTitle: "Déclaration de protection des données & Cookies (nLPD Suisse / LTC)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) utilise exclusivement des cookies techniques nécessaires (art. 45c LTC) et traite des données publiques du Registre du commerce (IDE/Zefix) conformément à la Loi fédérale sur la protection des données (nLPD — PFPDT).",
            cookieCheckLabel: "J'accepte la déclaration de protection des données selon la nLPD suisse et la politique de cookies techniques.",
            enterBtn: "Accéder au Portail Suisse (fr-CH)",
            sectionTitle: "Modules d'Audit Réglementaire pour la Suisse (6 Piliers • 37 Vecteurs)",
            sectionSub: "Modèles préliminaires selon le droit fédéral suisse (nLPD / LTr / CO / LBA / LCD)",
            deliverables: [
                "01. Déclaration de confidentialité & Registre des activités selon les art. 12, 19 nLPD (PFPDT)",
                "02. Concept de sécurité au travail selon l'art. 6 LTr, art. 82 LAA & Directive MSST 6508",
                "03. Règlement sur le télétravail & l'enregistrement du temps de travail (OLT 1 / CO)",
                "04. Procès-verbal de constitution de la Commission de sécurité (MSST / SUVA)",
                "05. Contrat-cadre de Prestation & Protocole d'archivage intègre selon l'art. 957 CO et l'Olico",
                "06. Registre des ayants droit économiques (art. 697j CO / GAFI) & Concordance AVS/SUVA",
                "07. Conditions Générales (CGV) E-Commerce selon l'art. 3 let. s LCD & OIP"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "PFPDT • nLPD & OPDo", title: "1. Loi fédérale sur la protection des données", ref: "Art. 8, 12, 19, 60 nLPD • OPDo", desc: "Prévention des amendes pénales jusqu'à CHF 250'000 : avis de confidentialité, registre des traitements, communication à l'étranger et sécurité TLS.", status: "Modèle : Dossier nLPD Suisse" },
                { color: "border-blue-500 text-blue-400", badge: "SECO & SUVA • LTr / LAA", title: "2. Loi sur le travail (LTr) & Directive MSST", ref: "Art. 6 LTr • Art. 82 LAA • MSST 6508", desc: "Protection de la santé physique et psychique, protection de l'intégrité personnelle et enregistrement du temps de travail (OLT 1).", status: "Modèle : Règlement LTr & MSST 6508" },
                { color: "border-emerald-500 text-emerald-400", badge: "AFC • CO Art. 957 & Olico", title: "3. Code des obligations (CO) & Olico", ref: "Art. 957–958f CO • Olico • LTVA", desc: "Conservation inaltérable des livres comptables et pièces justificatives pendant 10 ans selon l'Olico et concordance TVA.", status: "Modèle : Protocole Olico & TVA"},
                { color: "border-purple-500 text-purple-400", badge: "GAFI / LBA • Art. 697j CO", title: "4. Ayants droit économiques & Assurances sociales", ref: "Art. 697j–697m CO • LBA • AVS/SUVA", desc: "Tenue obligatoire de la liste des ayants droit économiques, obligations de diligence LBA et concordance des salaires AVS/SUVA.", status: "Modèle : Registre Art. 697j CO & LBA" },
                { color: "border-amber-500 text-amber-400", badge: "OFCS • LSI & AEAI", title: "5. Loi sur la sécurité de l'information (LSI)", ref: "LSI • Directives OFCS • Prescriptions AEAI", desc: "Gouvernance de cybersécurité selon l'Office fédéral de la cybersécurité (OFCS) et normes de protection incendie AEAI.", status: "Modèle : Plan de Sécurité LSI & AEAI" },
                { color: "border-teal-500 text-teal-400", badge: "SECO • LCD & OIP", title: "6. Loi contre la concurrence déloyale (LCD)", ref: "Art. 3 let. s LCD • Art. 8 LCD • OIP", desc: "Mentions obligatoires dans le commerce électronique (art. 3 let. s LCD), Ordonnance sur l'indication des prix (OIP) et contrôle des CGV.", status: "Modèle : CGV E-Commerce Suisse" }
            ]
        },
        "it-CH": {
            country: "CH", block: "DACH", name: "Svizzera", langName: "Italiano (it-CH)", currency: "CHF (Fr.)",
            kitPrice: "CHF 290.00", simulatedFine: "CHF 165'000.00", templatePath: "templates/dach/it_ch",
            heroBadge: "PERIZIA DI CONFORMITÀ • 37 VETTORI NORMATIVI SVIZZERI (CH)",
            heroTitle: "Diagnosi delle Contingenze Legali & Scudo Societario 360° (Svizzera)",
            heroSubtitle: "Verifica conforme al diritto federale svizzero: nuova Legge sulla protezione dei dati (nLPD / IFPDT), Legge sul lavoro (LL / SUVA / MSSL 6508), CO art. 957 (Olc), GAFI e LCSl.",
            taxLabel: "Numero IDI / UID (CHE-XXX.XXX.XXX) • NOGA 2008",
            taxPlaceholder: "Inserire il numero IDI (es. CHE-105.805.649)",
            domainPlaceholder: "Dominio Web (es. azienda.ch)",
            scanBtn: "Avvia Perizia di Conformità Svizzera",
            cryptoLaw: "FiEle / ZertES (Firma elettronica svizzera) • CO art. 957",
            fineNote: "Stima sanzionatoria secondo gli art. 60 segg. nLPD (fino a CHF 250'000), LL e Codice delle obbligazioni.",
            privacyTitle: "Informativa sulla protezione dei dati & Cookie (nLPD Svizzera / LTC)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) utilizza esclusivamente cookie tecnici necessari (art. 45c LTC) e dati pubblici del Registro di commercio (IDI/Zefix) in conformità con la Legge federale sulla protezione dei dati (nLPD — IFPDT).",
            cookieCheckLabel: "Accetto l'informativa sulla protezione dei dati secondo la nLPD svizzera e la politica sui cookie tecnici.",
            enterBtn: "Accedi al Portale Svizzero (it-CH)",
            sectionTitle: "Moduli di Perizia Normativa per la Svizzera (6 Pilastri • 37 Vettori)",
            sectionSub: "Segnaposto normativi secondo il diritto federale svizzero (nLPD / LL / CO / LRD / LCSl)",
            deliverables: [
                "01. Informativa sulla privacy & Registro delle attività secondo gli art. 12, 19 nLPD (IFPDT)",
                "02. Concetto di sicurezza sul lavoro secondo l'art. 6 LL, art. 82 LAINF & Direttiva MSSL 6508",
                "03. Regolamento sul telelavoro & registrazione della durata del lavoro (OLL 1 / CO)",
                "04. Verbale di costituzione della Commissione di sicurezza sul lavoro (MSSL / SUVA)",
                "05. Contratto quadro di servizi & Protocollo di archiviazione secondo l'art. 957 CO e l'Olc",
                "06. Elenco degli aventi economicamente diritto (art. 697j CO / GAFI) & Verifica AVS/SUVA",
                "07. Condizioni Generali (CGC) E-Commerce secondo l'art. 3 lett. s LCSl & OIP"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "IFPDT • nLPD & OPDo", title: "1. Legge federale sulla protezione dei dati", ref: "Art. 8, 12, 19, 60 nLPD • OPDo", desc: "Prevenzione delle sanzioni penali fino a CHF 250'000: informativa privacy, registro dei trattamenti, comunicazione all'estero e sicurezza TLS.", status: "Segnaposto: Fascicolo nLPD (CH)" },
                { color: "border-blue-500 text-blue-400", badge: "SECO & SUVA • LL / LAINF", title: "2. Legge sul lavoro (LL) & Direttiva MSSL", ref: "Art. 6 LL • Art. 82 LAINF • MSSL 6508", desc: "Tutela della salute fisica e psichica, integrità personale dei lavoratori e registrazione della durata del lavoro (OLL 1).", status: "Segnaposto: Regolamento LL & MSSL" },
                { color: "border-emerald-500 text-emerald-400", badge: "AFC • CO Art. 957 & Olc", title: "3. Codice delle obbligazioni (CO) & Olc", ref: "Art. 957–958f CO • Olc • LIVA", desc: "Tenuta e conservazione inalterabile dei libri contabili per 10 anni secondo l'Ordinanza sui libri di commercio (Olc) e verifica IVA.", status: "Segnaposto: Protocollo Contabile Olc" },
                { color: "border-purple-500 text-purple-400", badge: "GAFI / LRD • Art. 697j CO", title: "4. Aventi economicamente diritto & Previdenza", ref: "Art. 697j–697m CO • LRD • AVS/SUVA", desc: "Elenco obbligatorio degli aventi economicamente diritto, obblighi di diligenza antiriciclaggio (LRD) e verifica contributiva AVS/SUVA.", status: "Segnaposto: Registro Art. 697j CO" },
                { color: "border-amber-500 text-amber-400", badge: "UFCS • LSI & AICAA", title: "5. Legge sulla sicurezza delle informazioni", ref: "LSI • Direttive UFCS • Prescrizioni AICAA", desc: "Misure di cibersicurezza federali (UFCS) e prescrizioni antincendio AICAA per le sedi operative in Svizzera.", status: "Segnaposto: Piano Sicurezza LSI" },
                { color: "border-teal-500 text-teal-400", badge: "SECO • LCSl & OIP", title: "6. Legge contro la concorrenza sleale (LCSl)", ref: "Art. 3 lett. s LCSl • Art. 8 LCSl • OIP", desc: "Obblighi di trasparenza nel commercio elettronico (art. 3 lett. s LCSl), Ordinanza sull'indicazione dei prezzi (OIP) e controllo CGC.", status: "Segnaposto: CGC E-Commerce Svizzera" }
            ]
        },
        "nl-NL": {
            country: "NL", block: "BENELUX", name: "Nederland", langName: "Nederlands (nl-NL)", currency: "EUR (€)",
            kitPrice: "€ 290,00", simulatedFine: "€ 142.000,00", templatePath: "templates/benelux/nl_nl",
            heroBadge: "WETTELIJKE COMPLIANCE AUDIT • 37 TOETSINGSVECTOREN (NEDERLAND)",
            heroTitle: "Boeterisico-Inventarisatie & 360° Juridisch Bedrijfsschild",
            heroSubtitle: "Deterministische controle volgens Nederlands recht: AVG (Autoriteit Persoonsgegevens), Arbowet (RI&E en PSA), Belastingdienst (Wet DBA / UBO) en ACM.",
            taxLabel: "KVK-nummer / RSIN / BTW-id • SBI-code",
            taxPlaceholder: "Voer KVK-nummer of BTW-id in (bijv. 12345678)",
            domainPlaceholder: "Zakelijk webdomein (bijv. bedrijf.nl)",
            scanBtn: "Start Nederlandse Compliance Scan",
            cryptoLaw: "eIDAS-verordening Art. 41 • Art. 156a Wetboek van Burgerlijke Rechtsvordering",
            fineNote: "Berekening conform Boetebeleidsregels Autoriteit Persoonsgegevens (AVG), Arbowet en ACM-richtlijnen.",
            privacyTitle: "Privacyverklaring, Cookiebeleid (Art. 11.7a Telecommunicatiewet) & Auditvoorwaarden (AVG / AP)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) plaatst uitsluitend functioneel noodzakelijke sessiecookies conform art. 11.7a lid 3 Telecommunicatiewet en verwerkt openbare KVK-handelsregistergegevens volgens de Algemene Verordening Gegevensbescherming (AVG — Autoriteit Persoonsgegevens).",
            cookieCheckLabel: "Ik ga akkoord met de Nederlandse Privacyverklaring (AVG/UAVG) en het functionele cookiebeleid.",
            enterBtn: "Open het Nederlandse Compliance Portaal (nl-NL)",
            sectionTitle: "Wettelijke Toetsingsmodules voor Nederland (6 Hoofdpijlers • 37 Vectoren)",
            sectionSub: "Gestructureerde sjablonen volgens Nederlands recht (AVG / Arbowet / Wet DBA / UBO / ACM)",
            deliverables: [
                "01. Privacyverklaring & Verwerkingsregister conform Art. 13, 14, 30 AVG / UAVG (AP)",
                "02. Risico-Inventarisatie en -Evaluatie (RI&E) & Plan van Aanpak (PSA) conform Art. 3, 5 Arbowet",
                "03. Thuiswerk- en Telewerkovereenkomst conform Wet flexibel werken & Arbobesluit",
                "04. Aanstellingsovereenkomst Preventiemedewerker & Ondernemingsraad (OR) Protocol",
                "05. Raamovereenkomst van Opdracht (Wet DBA) & Administratieve Bewaarplicht (Art. 52 AWR)",
                "06. KVK UBO-register Verklaring, Wwft-Compliance & Ketenaansprakelijkheid (Wka) Dossier",
                "07. Nederlandse Algemene Voorwaarden & Koop op Afstand (BW Boek 6 Art. 230m / ACM)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "Autoriteit Persoonsgegevens", title: "1. AVG & Uitvoeringswet (UAVG)", ref: "Art. 13, 28, 30, 32 AVG • UAVG • Art. 11.7a Tw", desc: "Toetsing van privacyverklaring, verwerkingsregister, verwerkersovereenkomsten en strikte AP-regels rond biometrie op de werkvloer.", status: "Sjabloon: AVG & Verwerkersregister (NL)" },
                { color: "border-blue-500 text-blue-400", badge: "Nederlandse Arbeidsinspectie", title: "2. Arbowet, RI&E & Psychosociale Arbeidsbelasting", ref: "Art. 3 en 5 Arbowet • Arbobesluit (PSA)", desc: "Verplichte Risico-Inventarisatie en -Evaluatie (RI&E), Plan van Aanpak, preventiemedewerker en beleid tegen werkdruk (PSA).", status: "Sjabloon: RI&E & PSA-Beleid (NL)" },
                { color: "border-emerald-500 text-emerald-400", badge: "Belastingdienst • Wet DBA", title: "3. Fiscale Bewaarplicht & Wet DBA", ref: "Art. 52 AWR • Wet DBA • Ketenaansprakelijkheid", desc: "Bewaarplicht administratie (7 jaar), voorkoming van schijnzelfstandigheid (Wet DBA) en vrijwaring ketenaansprakelijkheid (Wka).", status: "Sjabloon: Wet DBA & Wka-Overeenkomst" },
                { color: "border-purple-500 text-purple-400", badge: "KVK & FIU • Wwft & UBO", title: "4. UBO-register (KVK) & Wwft-Compliance", ref: "Handelsregisterwet • Wwft", desc: "Verplichte registratie van uiteindelijk belanghebbenden (UBO) bij KVK en cliëntenonderzoek onder de Wwft.", status: "Sjabloon: KVK UBO & Wwft-Dossier" },
                { color: "border-amber-500 text-amber-400", badge: "NCSC • Cyberbeveiligingswet", title: "5. NIS2 / Cyberbeveiligingswet & BHV", ref: "Cyberbeveiligingswet (NIS2) • Art. 15 Arbowet (BHV)", desc: "Zorg- en meldplicht onder de Nederlandse NIS2-implementatie en bedrijfshulpverlening (BHV-noodplan).", status: "Sjabloon: BHV-Plan & NIS2-Protocol" },
                { color: "border-teal-500 text-teal-400", badge: "ACM • Consumentenrecht", title: "6. Koop op Afstand (BW Boek 6) & ACM", ref: "Art. 6:230m e.v. BW • Prijzenwet", desc: "Informatieplichten e-commerce, 14 dagen herroepingsrecht, transparante prijsaanduiding en toetsing van algemene voorwaarden.", status: "Sjabloon: Nederlandse Algemene Voorwaarden" }
            ]
        },
        "nl-BE": {
            country: "BE", block: "BENELUX", name: "België", langName: "Vlaams / Nederlands (nl-BE)", currency: "EUR (€)",
            kitPrice: "290,00 €", simulatedFine: "139.500,00 €", templatePath: "templates/benelux/nl_be",
            heroBadge: "EXPERTISE-AUDIT • 37 BELGISCHE COMPLIANCE-VECTOREN (KBO)",
            heroTitle: "Patrimoniale Risicoanalyse & 360° Juridische Bescherming (België)",
            heroSubtitle: "Doorlichting volgens Belgisch recht: GBA (AVG), Codex Welzijn op het Werk (FOD WASO), RSZ Art. 30bis, UBO-register en Wetboek van Economisch Recht (WER).",
            taxLabel: "KBO-ondernemingsnummer • NACE-BEL",
            taxPlaceholder: "Voer KBO-nummer in (bijv. 0123.456.789)",
            domainPlaceholder: "Webdomein van de onderneming (bijv. onderneming.be)",
            scanBtn: "Start Belgische Compliance Audit",
            cryptoLaw: "eIDAS Art. 41 • Wetboek van economisch recht Boek XII",
            fineNote: "Berekend volgens het Belgisch Sociaal Strafwetboek (Niveau 1-4 met opdeciemen), GBA en WER.",
            privacyTitle: "Privacyverklaring, Cookiebeleid & Voorwaarden (Belgische GBA / AVG)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) gebruikt uitsluitend strikt noodzakelijke technische cookies (Wet Elektronische Communicatie) en openbare KBO-ondernemingsgegevens conform de AVG en de Belgische Kaderwet van 30 juli 2018 (Gegevensbeschermingsautoriteit — GBA).",
            cookieCheckLabel: "Ik aanvaard de Belgische Privacyverklaring (GBA) en het technisch cookiebeleid.",
            enterBtn: "Open het Belgische Compliance Portaal (nl-BE)",
            sectionTitle: "Wettelijke Auditmodules voor België (6 Hoofdpijlers • 37 Vectoren)",
            sectionSub: "Gestructureerde sjablonen naar Belgisch recht (GBA / Codex Welzijn / RSZ / UBO / WER)",
            deliverables: [
                "01. Privacybeleid, Verwerkingsregister & CAO nr. 81/68 Dossier (Belgische GBA)",
                "02. Globaal Preventieplan (5 jaar) & Jaarlijks Actieplan conform Codex Welzijn op het Werk",
                "03. Telewerk- en Deconnectiebeleid conform CAO nr. 85 & Belgische Arbeidsdeal",
                "04. Oprichtingsakte Interne Dienst voor Preventie en Bescherming op het Werk (IDPBW)",
                "05. Dienstverleningsovereenkomst & Boekhoudkundige Bewaarplicht (WIB 92 / WVV)",
                "06. Artikel 30bis RSZ Inhoudingsplicht-Protocol & Jaarlijkse Belgische UBO-bevestiging",
                "07. Belgische Algemene Voorwaarden & E-Commerce conform WER Boek VI en Boek XII"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "GBA / APD • Wet 30 juli 2018", title: "1. Gegevensbescherming (GBA) & CAO nr. 81", ref: "AVG • Kaderwet 30/07/2018 • CAO nr. 68/81", desc: "Controle van privacybeleid, dataregister, verwerkerscontracten en Belgische regels inzake camerabewaking en elektronische controle.", status: "Sjabloon: GBA Privacy-Dossier (BE)" },
                { color: "border-blue-500 text-blue-400", badge: "FOD WASO • Codex Welzijn", title: "2. Welzijnswet & Psychosociale Risico's", ref: "Wet 4 aug. 1996 • Codex Welzijn op het Werk", desc: "Globaal Preventieplan (5 jaar), Jaarlijks Actieplan, Interne Dienst (IDPBW), vertrouwenspersoon en recht op deconnectie.", status: "Sjabloon: Globaal Preventieplan (BE)" },
                { color: "border-emerald-500 text-emerald-400", badge: "FOD Financiën & RSZ", title: "3. Artikel 30bis RSZ & Fiscale Bewaarplicht", ref: "Art. 30bis RSZ-wet • WIB 92 (10 jaar)", desc: "Inhoudingsplicht bij onderaanneming (Art. 30bis RSZ) en digitale boekhoudkundige bewaarplicht (10 jaar).", status: "Sjabloon: Art. 30bis RSZ Protocol" },
                { color: "border-purple-500 text-purple-400", badge: "Thesaurie • UBO & AML", title: "4. Belgisch UBO-register & Witwaspreventie", ref: "Wet van 18 september 2017 • FOD Financiën", desc: "Jaarlijkse bevestiging van de uiteindelijk begunstigden in het Belgische UBO-register (MyMinfin) en anti-witwasverplichtingen.", status: "Sjabloon: Belgisch UBO-Dossier" },
                { color: "border-amber-500 text-amber-400", badge: "CCB • NIS2-wet België", title: "5. Belgische NIS2-wet (CCB) & Brandpreventie", ref: "Wet van 26 april 2024 (NIS2) • Codex Boek III", desc: "Verplichte cyberbeveiligingsmaatregelen onder toezicht van het Centrum voor Cybersecurity België (CCB) en intern noodplan.", status: "Sjabloon: CCB NIS2 & Intern Noodplan" },
                { color: "border-teal-500 text-teal-400", badge: "FOD Economie • WER Boek VI", title: "6. Wetboek van Economisch Recht (WER)", ref: "WER Boek VI en Boek XII • Onrechtmatige bedingen", desc: "E-commerce informatieplichten, verzakingsrecht en naleving van de Belgische wetgeving inzake onrechtmatige contractuele bedingen.", status: "Sjabloon: Belgische Voorwaarden (WER)" }
            ]
        },
        "fr-BE": {
            country: "BE", block: "BENELUX", name: "Belgique", langName: "Français (fr-BE)", currency: "EUR (€)",
            kitPrice: "290,00 €", simulatedFine: "139 500,00 €", templatePath: "templates/benelux/fr_be",
            heroBadge: "AUDIT D'EXPERTISE • 37 VECTEURS DE CONFORMITÉ BELGES (BCE)",
            heroTitle: "Diagnostic de Contingence Patrimoniale & Blindage 360° (Belgique)",
            heroSubtitle: "Contrôle selon le droit belge : APD (RGPD), Code du bien-être au travail (SPF ETCS), ONSS art. 30bis, Registre UBO et Code de droit économique (CDE).",
            taxLabel: "Numéro d'entreprise BCE • NACE-BEL",
            taxPlaceholder: "Saisir le numéro BCE (ex. 0123.456.789)",
            domainPlaceholder: "Domaine Web (ex. societe.be)",
            scanBtn: "Lancer l'Audit Réglementaire Belge",
            cryptoLaw: "eIDAS Art. 41 • Code de droit économique Livre XII",
            fineNote: "Calcul selon le Code pénal social belge (Niveaux 1 à 4 avec décimes additionnels), APD et CDE.",
            privacyTitle: "Politique de Confidentialité, Cookies & Conditions (APD Belgique / RGPD)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) utilise exclusivement des cookies techniques strictement nécessaires et traite les données publiques de la Banque-Carrefour des Entreprises (BCE) conformément au RGPD et à la loi belge du 30 juillet 2018 (Autorité de protection des données — APD).",
            cookieCheckLabel: "J'accepte la Politique de Confidentialité belge (APD/RGPD) et la politique relative aux cookies techniques.",
            enterBtn: "Accéder au Portail Belge (fr-BE)",
            sectionTitle: "Modules d'Audit Réglementaire pour la Belgique (6 Piliers • 37 Vecteurs)",
            sectionSub: "Modèles préliminaires selon le droit belge (APD / Code du bien-être / ONSS / UBO / CDE)",
            deliverables: [
                "01. Politique de confidentialité, Registre RGPD & Dossier CCT n° 81/68 (APD Belgique)",
                "02. Plan Global de Prévention (5 ans) & Plan d'Action Annuel (Code du bien-être au travail)",
                "03. Charte de Télétravail & Droit à la Déconnexion (CCT n° 85 & Deal pour l'emploi)",
                "04. Acte constitutif du Service Interne pour la Prévention et la Protection au Travail (SIPPT)",
                "05. Contrat de Prestation de Services & Conservation Comptable (CIR 92 / CSA)",
                "06. Protocole de retenue Article 30bis ONSS & Confirmation annuelle Registre UBO Belge",
                "07. Conditions Générales Belges & Commerce Électronique (CDE Livres VI et XII)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "APD / GBA • Loi 30/07/2018", title: "1. Protection des Données (APD) & CCT n° 81", ref: "RGPD • Loi du 30 juillet 2018 • CCT n° 68/81", desc: "Vérification de la politique de confidentialité, du registre RGPD et des règles belges sur la surveillance électronique des travailleurs.", status: "Modèle : Dossier RGPD / APD (Belgique)" },
                { color: "border-blue-500 text-blue-400", badge: "SPF ETCS • Bien-être au travail", title: "2. Code du bien-être & Risques psychosociaux", ref: "Loi du 4 août 1996 • Code du bien-être", desc: "Plan global de prévention (5 ans), plan d'action annuel, SIPPT, personne de confiance et droit à la déconnexion.", status: "Modèle : Plan Global de Prévention" },
                { color: "border-emerald-500 text-emerald-400", badge: "SPF Finances & ONSS", title: "3. Article 30bis ONSS & Conservation Fiscale", ref: "Art. 30bis Loi ONSS • CIR 92 (10 ans)", desc: "Obligation de retenue sur factures de sous-traitants (Art. 30bis ONSS) et conservation légale des pièces comptables.", status: "Modèle : Protocole ONSS Art. 30bis" },
                { color: "border-purple-500 text-purple-400", badge: "Trésorerie • Registre UBO", title: "4. Registre UBO Belge & Loi Anti-Blanchiment", ref: "Loi du 18 septembre 2017 • SPF Finances", desc: "Validation annuelle obligatoire des bénéficiaires effectifs sur MyMinfin (UBO) et vigilance anti-blanchiment.", status: "Modèle : Dossier UBO Belgique" },
                { color: "border-amber-500 text-amber-400", badge: "CCB • Loi NIS2 Belgique", title: "5. Loi NIS2 Belge (CCB) & Plan d'Urgence Interne", ref: "Loi du 26 avril 2024 (NIS2) • Code Bien-être", desc: "Conformité cybersécurité auprès du Centre pour la Cybersécurité Belgique (CCB) et prévention incendie.", status: "Modèle : Charte NIS2 & Plan d'Urgence" },
                { color: "border-teal-500 text-teal-400", badge: "SPF Économie • CDE Livre VI", title: "6. Code de droit économique (CDE)", ref: "CDE Livres VI et XII • Clauses abusives", desc: "Conformité des ventes à distance, droit de rétractation et purge des clauses abusives selon le CDE belge.", status: "Modèle : CGV Belges conformes au CDE" }
            ]
        },
        "lb-LU": {
            country: "LU", block: "BENELUX", name: "Lëtzebuerg", langName: "Lëtzebuergesch (lb-LU)", currency: "EUR (€)",
            kitPrice: "290,00 €", simulatedFine: "152.000,00 €", templatePath: "templates/benelux/lb_lu",
            heroBadge: "COMPLIANCE-AUDIT • 37 LËTZEBUERGER RECHTSVEKTOREN (RCS)",
            heroTitle: "Juristesche Risiko-Diagnos & 360° Entreprise-Schutzschëld (Lëtzebuerg)",
            heroSubtitle: "Deterministesch Kontroll no Lëtzebuerger Recht: CNPD (RGPD), ITM (Code du Travail), LBR (RBE Bénéficiaires Effectifs), ACD/AED an eIDAS.",
            taxLabel: "RCS Lëtzebuerg / Matricule National / TVA • NACELUX",
            taxPlaceholder: "RCS-Nummer oder TVA aginn (z.B. B123456)",
            domainPlaceholder: "Entreprise Web-Domain (z.B. firma.lu)",
            scanBtn: "Lëtzebuerger Audit starten",
            cryptoLaw: "eIDAS Art. 41 • Code civil luxembourgeois Art. 1322-1",
            fineNote: "Berechnung no CNPD-Sanktiounen, Code du Travail (ITM) a Loi RBE vum 13. Januar 2019.",
            privacyTitle: "Dateschutz- & Cookie-Richtlinn fir Lëtzebuerg (CNPD / RGPD)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) benotzt nëmmen technesch noutwendeg Sessiouns-Cookien a veraarbecht ëffentlech RCS- an Entreprise-Donnéeën am Aklang mat der RGPD an dem Lëtzebuerger Dateschutzgesetz vum 1. August 2018 (CNPD).",
            cookieCheckLabel: "Ech akzeptéieren d'Lëtzebuerger Dateschutzrichtlinn (CNPD) an d'Cookie-Politik.",
            enterBtn: "Op de Lëtzebuerger Compliance-Portal (lb-LU)",
            sectionTitle: "Gesetzlech Prüfungsmoduler fir Lëtzebuerg (6 Haaptsäilen • 37 Vektoren)",
            sectionSub: "Strukturéiert Modeller no Lëtzebuerger Recht (CNPD / ITM Code du Travail / LBR RBE / MPC)",
            deliverables: [
                "01. Dateschutzerklärung & Verarbechtungsregister no RGPD / CNPD & Art. L.261-1 Code du Travail",
                "02. Sécherheets- a Gesondheetskonzept op der Aarbecht (ITM / Art. L.311-1 Code du Travail)",
                "03. Lëtzebuerger Télétravail-Konventioun & Recht op Déconnexioun (Code du Travail)",
                "04. Ernennungsakt vum Travailleur Désigné & Délégation du Personnel Protokoll",
                "05. Dienstleistungsvertrag & 10-järeg Comptabilitéits-Archivéierung (Art. 16 Code de Commerce)",
                "06. RBE-Konformitéitsdossier (Registre des Bénéficiaires Effectifs — LBR) & CCSS-Kontroll",
                "07. Lëtzebuerger E-Commerce AGB & Konsumenteschutz (Code de la Consommation LU)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "CNPD • Gesetz 01/08/2018", title: "1. Dateschutz (CNPD) & Art. L.261-1 Code du Travail", ref: "RGPD • Gesetz vum 1. August 2018 • L.261-1", desc: "Kontroll vun der Dateschutzerklärung, dem Verarbechtungsregister an de strenge Lëtzebuerger Reegele fir d'Iwwerwaachung op der Aarbechtsplaz.", status: "Modell: CNPD Dateschutz-Dossier (LU)" },
                { color: "border-blue-500 text-blue-400", badge: "ITM • Code du Travail", title: "2. Aarbechtsrecht, Sécherheet & Télétravail", ref: "Art. L.311-1 bis L.314-4 Code du Travail", desc: "Sécherheets- a Gesondheetschutz op der Aarbecht (ITM), Travailleur désigné, Délégation du Personnel a Lëtzebuerger Télétravail-Konventioun.", status: "Modell: ITM Sécherheets- & Télétravail-Akt" },
                { color: "border-emerald-500 text-emerald-400", badge: "ACD & AED • Steierrecht", title: "3. Comptabilitéit, TVA & Steierlech Materialitéit", ref: "Code de Commerce Art. 16 • Loi TVA LU", desc: "10-järeg Archivéierungsflicht vun de Bicher a Rechnungen zu Lëtzebuerg a steierlech Substanznokweiser.", status: "Modell: Lëtzebuerger Rahmenkontrakt" },
                { color: "border-purple-500 text-purple-400", badge: "LBR • RBE & CCSS", title: "4. Registre des Bénéficiaires Effectifs (RBE)", ref: "Gesetz vum 13. Januar 2019 (RBE) • Loi AML", desc: "Verflichtend Deklaratioun am Lëtzebuerger RBE (LBR) fir Strofe vu 1.250 € bis 1.250.000 € ze vermeiden.", status: "Modell: LBR / RBE Konformitéits-Dossier" },
                { color: "border-amber-500 text-amber-400", badge: "ILR / HCPN • NIS2 Lëtzebuerg", title: "5. Cybersécherheet (NIS2) & Evakuatiounsplang", ref: "NIS2-Gesetz LU • ITM Noutruffplang", desc: "IT-Sécherheetsrichtlinnen ënner Opsiicht vum ILR/HCPN a Brandschutz- an Evakuatiounsprotokoller.", status: "Modell: NIS2 & Noutruff-Protokoll (LU)" },
                { color: "border-teal-500 text-teal-400", badge: "MPC • Code de la Consommation", title: "6. Konsumenteschutz & E-Commerce (LU)", ref: "Code de la Consommation • Loi E-Commerce", desc: "Rechtssécher Impressums- an AGB-Klauselen, 14 Deeg Widderruffsrecht a Präistransparenz zu Lëtzebuerg.", status: "Modell: Lëtzebuerger E-Commerce AGB" }
            ]
        },
        "fr-LU": {
            country: "LU", block: "BENELUX", name: "Luxembourg", langName: "Français (fr-LU)", currency: "EUR (€)",
            kitPrice: "290,00 €", simulatedFine: "152 000,00 €", templatePath: "templates/benelux/fr_lu",
            heroBadge: "AUDIT D'EXPERTISE • 37 VECTEURS DE CONFORMITÉ LUXEMBOURGEOIS (RCS)",
            heroTitle: "Diagnostic de Conformité Réglementaire & Blindage 360° (Luxembourg)",
            heroSubtitle: "Audit déterministe selon le droit luxembourgeois : CNPD (RGPD), ITM (Code du Travail L.312), Registre des Bénéficiaires Effectifs (RBE / LBR) et Code de la Consommation.",
            taxLabel: "Numéro RCS Luxembourg / Matricule / TVA • NACELUX",
            taxPlaceholder: "Saisir le numéro RCS ou TVA (ex. B123456)",
            domainPlaceholder: "Domaine Web (ex. societe.lu)",
            scanBtn: "Lancer l'Audit Luxembourgeois",
            cryptoLaw: "eIDAS Art. 41 • Code civil luxembourgeois Art. 1322-1",
            fineNote: "Barème conforme au RGPD (CNPD), Code du Travail (ITM) et Loi du 13 janvier 2019 (RBE : 1 250 € à 1 250 000 €).",
            privacyTitle: "Politique de Confidentialité, Cookies & Conditions (CNPD Luxembourg / RGPD)",
            privacyBody: "FIRMA W16 MALLEN® (mallen.lat) utilise exclusivement des cookies techniques nécessaires et traite les données publiques du Registre de Commerce et des Sociétés (RCS Luxembourg) conformément au RGPD et à la loi luxembourgeoise du 1er août 2018 (CNPD).",
            cookieCheckLabel: "J'accepte la Politique de Confidentialité luxembourgeoise (CNPD/RGPD) et la politique de cookies techniques.",
            enterBtn: "Accéder au Portail Luxembourgeois (fr-LU)",
            sectionTitle: "Modules d'Audit Réglementaire pour le Luxembourg (6 Piliers • 37 Vecteurs)",
            sectionSub: "Modèles préliminaires selon le droit du Grand-Duché de Luxembourg (CNPD / ITM / RBE / MPC)",
            deliverables: [
                "01. Politique de confidentialité, Registre RGPD & Dossier Art. L.261-1 Code du Travail (CNPD)",
                "02. Plan de santé et sécurité au travail (ITM / Art. L.311-1 à L.314-4 Code du Travail)",
                "03. Convention de Télétravail & Droit à la Déconnexion selon le Code du Travail luxembourgeois",
                "04. Acte de désignation du Travailleur Désigné & Procès-verbal de la Délégation du Personnel",
                "05. Contrat-cadre de Prestation & Conservation Comptable 10 ans (Art. 16 Code de Commerce)",
                "06. Dossier de conformité RBE (Registre des Bénéficiaires Effectifs — LBR) & CCSS",
                "07. Conditions Générales E-Commerce & Protection du Consommateur (Code de la Consommation LU)"
            ],
            laws: [
                { color: "border-cyan-500 text-cyan-400", badge: "CNPD • Loi du 01/08/2018", title: "1. Protection des Données (CNPD) & Art. L.261-1", ref: "RGPD • Loi du 1er août 2018 • Art. L.261-1 CT", desc: "Conformité RGPD devant la CNPD et respect strict de l'article L.261-1 du Code du travail sur la surveillance sur le lieu de travail.", status: "Modèle : Dossier CNPD Luxembourg" },
                { color: "border-blue-500 text-blue-400", badge: "ITM • Code du Travail L.312", title: "2. Santé au Travail (ITM) & Convention Télétravail", ref: "Art. L.311-1 à L.314-4 Code du Travail", desc: "Travailleur désigné, évaluation des postes à risques, prévention du harcèlement et convention collective sur le télétravail.", status: "Modèle : Charte ITM & Télétravail (LU)" },
                { color: "border-emerald-500 text-emerald-400", badge: "ACD & AED • Fiscalité LU", title: "3. Conservation Comptable (10 ans) & TVA", ref: "Art. 16 Code de Commerce • Loi TVA LU", desc: "Conservation probante des documents comptables pendant 10 ans et justification économique des prestations.", status: "Modèle : Contrat Cadre Luxembourgeois" },
                { color: "border-purple-500 text-purple-400", badge: "LBR • Loi RBE (1 250 000 €)", title: "4. Registre des Bénéficiaires Effectifs (RBE / LBR)", ref: "Loi du 13 janvier 2019 (RBE) • Loi AML 2004", desc: "Mise à jour exacte au Registre des Bénéficiaires Effectifs (LBR) prévenant les amendes pénales de 1 250 € à 1 250 000 €.", status: "Modèle : Acte de Conformité RBE (LBR)" },
                { color: "border-amber-500 text-amber-400", badge: "ILR / HCPN • NIS2 & Sécurité", title: "5. Cybersécurité NIS2 & Sécurité Incendie ITM", ref: "Directive NIS2 LU • Prescriptions ITM", desc: "Mesures de résilience cyber (ILR/HCPN) et registre de sécurité incendie et d'évacuation des locaux.", status: "Modèle : Plan NIS2 & Sécurité ITM" },
                { color: "border-teal-500 text-teal-400", badge: "MPC • Code de la Consommation", title: "6. Droit de la Consommation & Commerce Électronique", ref: "Code de la Consommation LU • Loi du 14 août 2000", desc: "Mentions légales obligatoires, garantie légale de conformité, droit de rétractation et purge des clauses abusives.", status: "Modèle : CGV Luxembourgeoises" }
            ]
        }
    };

    REGIONAL_PORTAL_DATA["de-BE"] = Object.assign({}, REGIONAL_PORTAL_DATA["de-DE"], {
        country: "BE", block: "BENELUX", name: "Belgien", langName: "Deutsch (de-BE)",
        templatePath: "templates/benelux/nl_be",
        enterBtn: "Weiter zum Belgischen Compliance-Portal (de-BE)"
    });
    REGIONAL_PORTAL_DATA["de-LU"] = Object.assign({}, REGIONAL_PORTAL_DATA["de-DE"], {
        country: "LU", block: "BENELUX", name: "Luxemburg", langName: "Deutsch (de-LU)",
        templatePath: "templates/benelux/fr_lu",
        enterBtn: "Weiter zum Luxemburger Compliance-Portal (de-LU)"
    });

    
    
    const OFFICIAL_COUNTRY_SOURCES = {
        "DE": "Offizielle Prüfungs- & Statistikquellen (Deutschland): Bundesbeauftragter für den Datenschutz (BfDI Tätigkeitsbericht), Bundesanstalt für Arbeitsschutz und Arbeitsmedizin (BAuA / DGUV SuGA-Bericht), Bundesministerium der Finanzen (BMF-Betriebsprüfungsstatistik), Bundesanzeiger (Transparenzregister / FIU) und Bundesamt für Sicherheit in der Informationstechnik (BSI-Lagebericht).",
        "AT": "Offizielle Prüfungs- & Statistikquellen (Österreich): Österreichische Datenschutzbehörde (DSB-Datenschutzbericht), Zentral-Arbeitsinspektorat (BMASGPC Tätigkeitsbericht der Arbeitsinspektion), Bundesministerium für Finanzen (BMF / Prüfdienst für Lohnabgaben und Beiträge PLAB) und WiEReG-Registerbehörde.",
        "CH": {
            "de": "Offizielle Prüfungs- & Statistikquellen (Schweiz): Eidgenössischer Datenschutz- und Öffentlichkeitsbeauftragter (EDÖB Tätigkeitsbericht), Staatssekretariat für Wirtschaft (SECO / EKAS-Jahresbericht 6508), Schweizerische Unfallversicherungsanstalt (SUVA-Unfallstatistik) und Eidgenössische Steuerverwaltung (ESTV).",
            "fr": "Sources statistiques et d'inspection officielles (Suisse) : Préposé fédéral à la protection des données et à la transparence (Rapport d'activité PFPDT), Secrétariat d'État à l'économie (SECO / Rapport annuel CFSL 6508), Caisse nationale suisse d'assurance en cas d'accidents (Statistique SUVA) et Administration fédérale des contributions (AFC).",
            "it": "Fonti statistiche e ispettive ufficiali (Svizzera): Incaricato federale della protezione dei dati e della trasparenza (Rapporto d'attività IFPDT), Segreteria di Stato dell'economia (SECO / Rapporto annuale CFSL/MSSL 6508), Istituto nazionale svizzero di assicurazione contro gli infortuni (Statistica SUVA) e Amministrazione federale delle contribuzioni (AFC)."
        },
        "NL": "Officiële inspectie- en statistiekbronnen (Nederland): Autoriteit Persoonsgegevens (AP Jaarverslag & Boetebeleidsregels), Nederlandse Arbeidsinspectie (Jaarverslag RI&E en Arbowet), Belastingdienst, Kamer van Koophandel (KVK Handelsregister / UBO) en Autoriteit Consument & Markt (ACM Jaarverslag).",
        "BE": {
            "nl": "Officiële inspectie- en statistiekbronnen (België): Gegevensbeschermingsautoriteit (GBA Jaarverslag), FOD Werkgelegenheid, Arbeid en Sociaal Overleg (FOD WASO — Toezicht op het Welzijn op het Werk), Rijksdienst voor Sociale Zekerheid (RSZ / SIOD Sociale Inlichtingen- en Opsporingsdienst) en FOD Financiën / FOD Economie.",
            "fr": "Sources statistiques et d'inspection officielles (Belgique) : Autorité de protection des données (Rapport annuel APD), SPF Emploi, Travail et Concertation sociale (SPF ETCS — Contrôle du bien-être au travail), Office national de sécurité sociale (ONSS / SIRS Service d'information et de recherche sociale) et SPF Finances / SPF Économie.",
            "de": "Offizielle Prüfungs- & Statistikquellen (Belgien): Datenschutzbehörde (GBA/APD Jahresbericht), FÖD Beschäftigung, Arbeit und Soziale Konzertierung, Landesamt für Soziale Sicherheit (LSS / SIOD) und FÖD Finanzen."
        },
        "LU": {
            "lb": "Offiziell Inspektiouns- a Statistikquellen (Lëtzebuerg): Commission Nationale pour la Protection des Données (CNPD Joresrapport), Inspection du Travail et des Mines (ITM Joresrapport), Luxembourg Business Registers (LBR / Registre des Bénéficiaires Effectifs RBE) an Administration des contributions directes (ACD / AED).",
            "fr": "Sources statistiques et d'inspection officielles (Luxembourg) : Commission Nationale pour la Protection des Données (Rapport annuel CNPD), Inspection du Travail et des Mines (Rapport annuel ITM), Luxembourg Business Registers (LBR / RBE) et Administration des contributions directes (ACD / AED).",
            "de": "Offizielle Prüfungs- & Statistikquellen (Luxemburg): Nationale Kommission für den Datenschutz (CNPD Jahresbericht), Gewerbe- und Grubenaufsichtsamt (ITM Jahresbericht), Luxembourg Business Registers (LBR / RBE) und Steuerverwaltung (ACD / AED)."
        }
    };

    
    const THEME_BTN_LABELS = {
        "de": { light: "Hellmodus (Weiß)", dark: "Dunkelmodus" },
        "fr": { light: "Mode Clair", dark: "Mode Sombre" },
        "it": { light: "Modalità Chiara", dark: "Modalità Scura" },
        "nl": { light: "Lichte Modus", dark: "Donkere Modus" },
        "lb": { light: "Helle Modus", dark: "Donkele Modus" },
        "es": { light: "Modo Claro", dark: "Modo Oscuro" }
    };

    window.updateThemeButtonLabel = function () {
        const btn = document.getElementById("w16-theme-toggle-btn");
        if (!btn) return;
        const lang = (document.documentElement.lang || localStorage.getItem("mallen_locale") || "de-DE").split("-")[0].toLowerCase();
        const dict = THEME_BTN_LABELS[lang] || THEME_BTN_LABELS["de"];
        const isLight = document.documentElement.classList.contains("w16-light-mode");
        btn.textContent = isLight ? dict.dark : dict.light;
    };

    window.toggleW16PureLightMode = function () {
        const htmlEl = document.documentElement;
        const isLight = htmlEl.classList.toggle("w16-light-mode");
        localStorage.setItem("w16_theme_mode", isLight ? "light" : "dark");
        const btn = document.getElementById("w16-theme-toggle-btn");
        if (btn) {
            window.updateThemeButtonLabel();
        }
    };

    (function applySavedThemeOnLoad() {
        setTimeout(function(){ if(window.updateThemeButtonLabel) window.updateThemeButtonLabel(); }, 10);
        if (localStorage.getItem("w16_theme_mode") === "light") {
            document.documentElement.classList.add("w16-light-mode");
        }
    })();

    const NATIVE_PORTAL_UI = {
        "de": {
            dialectLabel: "Regionalsprache:",
            changeRegionBtn: "Land / Sprache wechseln",
            headerSub: "INFRASTRUKTUR FÜR DETERMINISTISCHE AUDITS & DIGITALE BEGUTACHTUNG W16",
            navCoverage: "Gesetzliche Abdeckung",
            navTracker: "Compliance-Tracker",
            navDeliverables: "7 Rechtsinstrumente",
            navProof: "eIDAS Beweiskraft",
            domainLabel: "Unternehmens-Domain (TLS 1.3 / DSGVO-Telemetrie)",
            vectorsPill: "37 Prüfvektoren • 10 Zertifikate • 7 Rechtsinstrumente",
            kitTag: "UNTERNEHMENS-COMPLIANCE-DOSSIER",
            kitTitle: "7 Parametrisierte Rechtsinstrumente im Regionalordner enthalten",
            folderPrefix: "Pfad: ",
            proofHeaderTag: "SHA-256 • L2 MERKLE ROOT • VOLLE BEWEISKRAFT",
            simReadyPrefix: "VORAB-TELEMETRIE ERSTELLT • ",
            simPackPrefix: "Vollständiges Dossier: ",
            simExpLabel: "Potenzielles Bußgeldrisiko (Vermeidbar)",
            simBandLabel: "Typische Prüfungsquote (Branchen-Richtwert)",
            simBandVal: "42% – 54% (Präventiv)",
            simKitLabel: "Dokumentations- & ROI-Schutz",
            simKitVal: "37 Vektoren + 7 Verträge",
            simBodyTpl: function(name, lang, path) {
                return "Deterministische Wert- und Compliance-Analyse über die 37 gesetzlichen Vektoren für " + name + " (" + lang + "). Sämtliche 7 unterschriftsreifen Rechtsinstrumente sind strukturiert unter " + path + ".";
            },
            footer1: "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Alle Rechte vorbehalten.",
            footer2: "Infrastruktur für deterministische Audits und digitale Sachverständigenbegutachtung",
            footer3: "Kryptografische Signatur: Region Zentraleuropa (Frankfurt Core Hub — EU-DE-01).",
            footer4: "Regulatorische Konformität und grenzüberschreitende Beweiskraft: eIDAS Art. 41 (EU) • Handelsgesetzbuch Art. 89-93 (MX) • SEC / FINRA Compliance."
        },
        "fr": {
            dialectLabel: "Langue régionale :",
            changeRegionBtn: "Changer de Pays / Langue",
            headerSub: "INFRASTRUCTURE D'AUDIT DÉTERMINISTE ET D'EXPERTISE NUMÉRIQUE W16",
            navCoverage: "Couverture Légale",
            navTracker: "Suivi de Conformité",
            navDeliverables: "7 Instruments Juridiques",
            navProof: "Valeur Probante eIDAS",
            domainLabel: "Domaine Web de l'Entreprise (TLS 1.3 / Télémétrie RGPD)",
            vectorsPill: "37 Vecteurs d'Audit • 10 Certificats • 7 Instruments Juridiques",
            kitTag: "DOSSIER EXÉCUTIF DE CONFORMITÉ D'ENTREPRISE",
            kitTitle: "7 Instruments Juridiques Paramétrés Inclus dans le Dossier Régional",
            folderPrefix: "Répertoire : ",
            proofHeaderTag: "SHA-256 • RACINE DE MERKLE L2 • PLEINE VALEUR PROBANTE",
            simReadyPrefix: "DIAGNOSTIC PRÉLIMINAIRE GÉNÉRÉ • ",
            simPackPrefix: "Dossier Complet : ",
            simExpLabel: "Exposition Réglementaire Évitable",
            simBandLabel: "Fourchette d'Observation Réaliste",
            simBandVal: "41% – 53% (Préventif)",
            simKitLabel: "Protection Documentaire & ROI",
            simKitVal: "37 Vecteurs + 7 Contrats",
            simBodyTpl: function(name, lang, path) {
                return "Évaluation déterministe axée sur la création de valeur et la conformité sur les 37 vecteurs réglementaires pour " + name + " (" + lang + "). Les 7 actes prêts à signer sont structurés dans " + path + ".";
            },
            footer1: "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Tous droits réservés.",
            footer2: "Infrastructure d'Audit Déterministe et d'Expertise Numérique",
            footer3: "Signature Cryptographique : Région Europe Centrale (Frankfurt Core Hub — EU-DE-01).",
            footer4: "Conformité réglementaire et valeur probante transfrontalière : eIDAS Art. 41 (UE) • Code de Commerce Art. 89-93 (MX) • Conformité SEC / FINRA."
        },
        "it": {
            dialectLabel: "Lingua regionale:",
            changeRegionBtn: "Cambia Paese / Lingua",
            headerSub: "INFRASTRUTTURA DI AUDIT DETERMINISTICO E PERIZIA DIGITALE W16",
            navCoverage: "Copertura Normativa",
            navTracker: "Monitoraggio Conformità",
            navDeliverables: "7 Strumenti Giuridici",
            navProof: "Valore Probatorio FiEle",
            domainLabel: "Dominio Web Aziendale (TLS 1.3 / Telemetria nLPD)",
            vectorsPill: "37 Vettori di Perizia • 10 Certificati • 7 Strumenti Giuridici",
            kitTag: "FASCICOLO ESECUTIVO DI CONFORMITÀ SOCIETARIA",
            kitTitle: "7 Strumenti Giuridici Parametrizzati Inclusi nella Cartella Regionale",
            folderPrefix: "Percorso: ",
            proofHeaderTag: "SHA-256 • L2 MERKLE ROOT • PIENO VALORE PROBATORIO",
            simReadyPrefix: "DIAGNOSI PRELIMINARE GENERATA • ",
            simPackPrefix: "Fascicolo Completo: ",
            simExpLabel: "Esposizione Sanzionatoria Prevenibile",
            simBandLabel: "Fascia Realistica di Osservazione",
            simBandVal: "40% – 52% (Preventivo)",
            simKitLabel: "Copertura Documentale & ROI",
            simKitVal: "37 Vettori + 7 Contratti",
            simBodyTpl: function(name, lang, path) {
                return "Valutazione deterministica orientata al valore operativo sui 37 vettori normativi per " + name + " (" + lang + "). I 7 contratti e regolamenti pronti per la firma sono strutturati in " + path + ".";
            },
            footer1: "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Tutti i diritti riservati.",
            footer2: "Infrastruttura di Audit Deterministico e Perizia Digitale",
            footer3: "Firma Crittografica: Regione Europa Centrale (Frankfurt Core Hub — EU-DE-01).",
            footer4: "Conformità normativa e validità probatoria transfrontaliera: eIDAS Art. 41 (UE) • Codice di Commercio Art. 89-93 (MX) • SEC / FINRA Compliance."
        },
        "nl": {
            dialectLabel: "Regionale taal:",
            changeRegionBtn: "Land / Taal wijzigen",
            headerSub: "INFRASTRUCTUUR VOOR DETERMINISTISCHE AUDIT EN DIGITALE EXPERTISE W16",
            navCoverage: "Wettelijke Dekking",
            navTracker: "Compliance Tracker",
            navDeliverables: "7 Juridische Instrumenten",
            navProof: "eIDAS Bewijskracht",
            domainLabel: "Zakelijk Webdomein (TLS 1.3 / AVG-Telemetrie)",
            vectorsPill: "37 Toetsingsvectoren • 10 Certificaten • 7 Juridische Instrumenten",
            kitTag: "EXECUTIEF BEDRIJFSCOMPLIANCE DOSSIER",
            kitTitle: "7 Geparametriseerde Juridische Instrumenten in de Regionale Map",
            folderPrefix: "Map: ",
            proofHeaderTag: "SHA-256 • L2 MERKLE ROOT • VOLLEDIGE BEWIJSKRACHT",
            simReadyPrefix: "VOORLOPIGE DIAGNOSE GEGENEREERD • ",
            simPackPrefix: "Volledig Dossier: ",
            simExpLabel: "Voorkombare Boete-Blootstelling",
            simBandLabel: "Realistische Observatiebandbreedte",
            simBandVal: "41% – 54% (Preventief)",
            simKitLabel: "Documentaire Bescherming & ROI",
            simKitVal: "37 Vectoren + 7 Contracten",
            simBodyTpl: function(name, lang, path) {
                return "Deterministische waarde- en compliance-evaluatie over de 37 wettelijke vectoren voor " + name + " (" + lang + "). Alle 7 ondertekeningsklare overeenkomsten zijn gestructureerd in " + path + ".";
            },
            footer1: "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Alle rechten voorbehouden.",
            footer2: "Infrastructuur voor Deterministische Audit en Digitale Expertise",
            footer3: "Cryptografische Handtekening: Regio Centraal-Europa (Frankfurt Core Hub — EU-DE-01).",
            footer4: "Wettelijke naleving en grensoverschrijdende bewijskracht: eIDAS Art. 41 (EU) • Wetboek van Koophandel Art. 89-93 (MX) • SEC / FINRA Compliance."
        },
        "lb": {
            dialectLabel: "Regionalsprooch:",
            changeRegionBtn: "Land / Sprooch wiesselen",
            headerSub: "INFRASTRUKTUR FIR DETERMINISTESCHEN AUDIT AN DIGITAL EXPERTISE W16",
            navCoverage: "Gesetzlech Ofdeckung",
            navTracker: "Compliance Tracker",
            navDeliverables: "7 Juristesch Instrumenter",
            navProof: "eIDAS Beweiskraaft",
            domainLabel: "Entreprise Web-Domain (TLS 1.3 / RGPD-Telemetrie)",
            vectorsPill: "37 Prüfungsvektoren • 10 Zertifikater • 7 Juristesch Instrumenter",
            kitTag: "EXEKUTIV ENTREPRISE-COMPLIANCE DOSSIER",
            kitTitle: "7 Parametriséiert Juristesch Instrumenter am Regionalen Dossier",
            folderPrefix: "Dossier: ",
            proofHeaderTag: "SHA-256 • L2 MERKLE ROOT • VOLL BEWEISKRAAFT",
            simReadyPrefix: "VIRLEEFEG DIAGNOS GENERÉIERT • ",
            simPackPrefix: "Kompletten Dossier: ",
            simExpLabel: "Vermeidbar Reglementaresch Belaaschtung",
            simBandLabel: "Realistesch Observatiouns-Bandbreet",
            simBandVal: "40% – 52% (Präventiv)",
            simKitLabel: "Dokumentaresche Schutz & ROI",
            simKitVal: "37 Vektoren + 7 Kontrakter",
            simBodyTpl: function(name, lang, path) {
                return "Deterministesch Wäert- a Konformitéitsevaluatioun iwwer déi 37 gesetzlech Vektore fir " + name + " (" + lang + "). Déi 7 ënnerschrëftsreife Kontrakter sinn an " + path + " strukturéiert.";
            },
            footer1: "© 2026 FIRMA W16 MALLEN® (mallen.lat) • All Rechter virbehalen.",
            footer2: "Infrastruktur fir Deterministeschen Audit an Digital Expertise",
            footer3: "Kryptographesch Ënnerschrëft: Regioun Zentraleuropa (Frankfurt Core Hub — EU-DE-01).",
            footer4: "Reglementaresch Konformitéit a grenziwwerschreidend Beweiskraaft: eIDAS Art. 41 (EU) • Code de Commerce Art. 89-93 (MX) • SEC / FINRA Compliance."
        }
    };

    window.applyNativePortalTranslationsAndSdui = function (countryCode, activeLocale, data) {
        const langPrefix = (activeLocale || "de-DE").split("-")[0].toLowerCase();
        const t = NATIVE_PORTAL_UI[langPrefix] || NATIVE_PORTAL_UI["de"];

        const setTxt = function (id, val) {
            const el = document.getElementById(id);
            if (el && val) el.textContent = val;
        };

        setTxt("eu-dialect-label", t.dialectLabel);
        setTxt("eu-change-region-btn", t.changeRegionBtn);
        setTxt("eu-header-sub", t.headerSub);
        setTxt("eu-nav-coverage", t.navCoverage);
        setTxt("eu-nav-tracker", t.navTracker);
        setTxt("eu-nav-deliverables", t.navDeliverables);
        setTxt("eu-nav-proof", t.navProof);
        setTxt("eu-domain-label", t.domainLabel);
        setTxt("eu-vectors-pill", t.vectorsPill);
        setTxt("eu-kit-tag", t.kitTag);
        setTxt("eu-kit-title", t.kitTitle);
        setTxt("eu-kit-folder-badge", t.folderPrefix + data.templatePath);
        setTxt("eu-proof-header-tag", t.proofHeaderTag);
        setTxt("eu-sim-exp-label", t.simExpLabel);
        setTxt("eu-sim-band-label", t.simBandLabel);
        setTxt("eu-sim-band-val", t.simBandVal);
        setTxt("eu-sim-kit-label", t.simKitLabel);
        setTxt("eu-sim-kit", t.simKitVal);
        setTxt("eu-footer-line-1", t.footer1);
        setTxt("eu-footer-line-2", t.footer2);
        setTxt("eu-footer-line-3", t.footer3);
        setTxt("eu-footer-line-4", t.footer4);
        if (window.updateThemeButtonLabel) window.updateThemeButtonLabel();
        const fRegBtn = document.getElementById("eu-footer-region-btn");
        if (fRegBtn) fRegBtn.textContent = t.changeRegionBtn;
        const srcBox = document.getElementById("eu-official-sources-box");
        if (srcBox) {
            const rawSrc = OFFICIAL_COUNTRY_SOURCES[countryCode];
            if (typeof rawSrc === "string") {
                srcBox.textContent = rawSrc;
            } else if (rawSrc && typeof rawSrc === "object") {
                srcBox.textContent = rawSrc[langPrefix] || rawSrc["de"] || "";
            }
        }

        window.runEuRegionalSimulation = function () {
            const taxVal = document.getElementById("eu-tax-input").value.trim() || ("ID-" + data.country + "-2026");
            const domVal = document.getElementById("eu-domain-input").value.trim() || ("corporate." + data.country.toLowerCase());
            const prev = document.getElementById("eu-scan-preview");
            prev.classList.remove("hidden");
            setTxt("eu-preview-title", t.simReadyPrefix + taxVal + " (" + domVal + ")");
            setTxt("eu-preview-price", t.simPackPrefix + data.kitPrice);
            setTxt("eu-sim-fine", data.simulatedFine);
            setTxt("eu-preview-body", t.simBodyTpl(data.name, data.langName, data.templatePath));
            setTxt("eu-fine-note", data.fineNote);
        };

        // Cargar Server-Driven UI (Compliance Implementation Tracker) desde Rust
        fetch("/api/sdui/tracker/" + encodeURIComponent(countryCode) + "?lang=" + encodeURIComponent(activeLocale))
            .then(function (r) { return r.json(); })
            .then(function (schema) {
                window.ACTIVE_SDUI_SCHEMA = schema;
                setTxt("sdui-title", schema.ui_title);
                setTxt("sdui-subtitle", schema.ui_subtitle);
                setTxt("sdui-progress-lbl", schema.ui_progress_label);
                setTxt("sdui-export-pdf-btn", schema.ui_export_btn);

                const storageKey = "w16_sdui_done_" + schema.country;
                let completedIds = [];
                try {
                    completedIds = JSON.parse(localStorage.getItem(storageKey) || "[]");
                } catch (_) { completedIds = []; }

                let currentFilterIdx = 0;

                function updateSduiView() {
                    const stepsBox = document.getElementById("sdui-steps-container");
                    const tabsBox = document.getElementById("sdui-phase-tabs");
                    if (!stepsBox || !tabsBox) return;

                    let totalWeight = 0;
                    schema.steps.forEach(function (st) {
                        if (completedIds.indexOf(st.id) !== -1) totalWeight += st.weight_pct;
                    });
                    setTxt("sdui-progress-pct", totalWeight + "%");
                    const pBar = document.getElementById("sdui-progress-bar");
                    if (pBar) pBar.style.width = totalWeight + "%";

                    tabsBox.innerHTML = "";
                    (schema.ui_phase_filters || []).forEach(function (tabLbl, idx) {
                        const btn = document.createElement("button");
                        btn.type = "button";
                        const active = idx === currentFilterIdx;
                        btn.style.cssText = active
                            ? "background:linear-gradient(90deg,#1d4ed8,#059669);color:#ffffff;border:1px solid #34d399;"
                            : "background:#0f172a;color:#94a3b8;border:1px solid #1e293b;";
                        btn.className = "px-3.5 py-1.5 rounded-lg font-bold transition";
                        btn.textContent = tabLbl;
                        btn.onclick = function () {
                            currentFilterIdx = idx;
                            updateSduiView();
                        };
                        tabsBox.appendChild(btn);
                    });

                    stepsBox.innerHTML = "";
                    schema.steps.forEach(function (st) {
                        if (currentFilterIdx === 1 && st.phase.indexOf("1-30") === -1) return;
                        if (currentFilterIdx === 2 && st.phase.indexOf("31-60") === -1) return;
                        if (currentFilterIdx === 3 && st.phase.indexOf("61-90") === -1) return;

                        const isDone = completedIds.indexOf(st.id) !== -1;
                        const row = document.createElement("label");
                        row.className = "p-4 rounded-xl border transition cursor-pointer flex items-start justify-between gap-3 " +
                            (isDone ? "bg-emerald-950/25 border-emerald-500/50" : "bg-slate-950/90 border-slate-800 hover:border-slate-700");
                        row.innerHTML = `
                            <div class="flex items-start gap-3">
                                <input type="checkbox" ${isDone ? "checked" : ""} class="mt-1 w-4 h-4 accent-emerald-500 rounded cursor-pointer">
                                <div>
                                    <div class="flex items-center gap-2 mb-1">
                                        <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-700 text-sky-400 font-bold text-[10px]">${st.code}</span>
                                        <span class="text-[10px] text-slate-400">${st.phase}</span>
                                    </div>
                                    <div class="text-slate-100 font-bold text-xs mb-1">${st.title}</div>
                                    <div class="text-[10px] text-emerald-400">${st.deliverable}</div>
                                </div>
                            </div>
                            <span class="text-xs font-extrabold ${isDone ? "text-emerald-400" : "text-slate-500"}">+${st.weight_pct}%</span>
                        `;
                        row.querySelector("input").addEventListener("change", function (ev) {
                            if (ev.target.checked) {
                                if (completedIds.indexOf(st.id) === -1) completedIds.push(st.id);
                            } else {
                                completedIds = completedIds.filter(function (x) { return x !== st.id; });
                            }
                            localStorage.setItem(storageKey, JSON.stringify(completedIds));
                            updateSduiView();
                        });
                        stepsBox.appendChild(row);
                    });
                }

                updateSduiView();

                window.exportSduiTrackerPdf = function () {
                    const taxInput = document.getElementById("eu-tax-input");
                    const domInput = document.getElementById("eu-domain-input");
                    const payload = {
                        entity_name: domInput && domInput.value.trim() ? domInput.value.trim() : (data.name + " Corporate Entity"),
                        tax_id: taxInput && taxInput.value.trim() ? taxInput.value.trim() : ("W16-" + schema.country + "-2026"),
                        locale: activeLocale,
                        completed_ids: completedIds
                    };
                    fetch("/api/sdui/tracker/" + encodeURIComponent(countryCode) + "/export_pdf", {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify(payload)
                    })
                    .then(function (r) { return r.blob(); })
                    .then(function (blob) {
                        const url = URL.createObjectURL(blob);
                        const a = document.createElement("a");
                        a.href = url;
                        a.download = "W16_Mallen_Compliance_Tracker_" + schema.country + ".pdf";
                        document.body.appendChild(a);
                        a.click();
                        a.remove();
                    });
                };
            });
    };

    const COUNTRY_DIALECTS = {
        "DE": ["de-DE"],
        "AT": ["de-AT"],
        "CH": ["de-CH", "fr-CH", "it-CH"],
        "NL": ["nl-NL"],
        "BE": ["nl-BE", "fr-BE", "de-BE"],
        "LU": ["lb-LU", "fr-LU", "de-LU"]
    };

    function buildFullScreenGateway(forceShow) {
        const consentedLocale = localStorage.getItem("mallen_consented_locale_v3");
        if (consentedLocale && !forceShow) return;

        let overlay = document.getElementById("mallen-fullscreen-gateway");
        if (overlay) overlay.remove();

        overlay = document.createElement("div");
        overlay.id = "mallen-fullscreen-gateway";
        overlay.style.cssText = "position:fixed;inset:0;background:#020617;color:#f8fafc;z-index:999999;overflow-y:auto;font-family:'Inter',system-ui,sans-serif;display:flex;flex-direction:column;justify-content:space-between;padding:24px;transform:translate3d(0,0,0);";

        overlay.innerHTML = `
            <div style="max-width:1120px;width:100%;margin:0 auto;padding-top:10px;">
                <div style="display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid #1e293b;padding-bottom:16px;margin-bottom:28px;flex-wrap:wrap;gap:12px;">
                    <div style="display:flex;align-items:center;gap:12px;">
                        <div style="width:42px;height:42px;border-radius:12px;background:linear-gradient(135deg,#1d4ed8,#059669);display:flex;align-items:center;justify-content:center;font-weight:900;font-size:20px;color:#fff;">M</div>
                        <div>
                            <div style="font-weight:800;font-size:15px;letter-spacing:0.04em;color:#ffffff;">FIRMA W16 MALLEN®</div>
                            <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:#34d399;">GLOBAL REGULATORY COMPLIANCE ENGINE W16</div>
                        </div>
                    </div>
                    <span style="font-family:'JetBrains Mono',monospace;font-size:11px;color:#cbd5e1;background:#0f172a;border:1px solid #1e293b;padding:6px 14px;border-radius:8px;font-weight:700;">
                        LATAM • DACH • BENELUX
                    </span>
                </div>

                <!-- PASO 1: SELECCIÓN DE BLOQUE ECONÓMICO, PAÍS E IDIOMA REGIONAL (SIN EMOJIS NI DUPLICADOS) -->
                <div id="mallen-step-language-select">


                    <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(320px,1fr));gap:20px;">
                        <!-- BLOQUE LATAM -->
                        <div style="background:#0f172a;border:1px solid #1e293b;border-top:4px solid #10b981;border-radius:16px;padding:24px;display:flex;flex-direction:column;justify-content:space-between;">
                            <div>
                                <div style="font-size:24px;font-weight:900;color:#10b981;letter-spacing:0.04em;margin-bottom:4px;">LATAM</div>
                                <div style="font-size:11px;font-family:'JetBrains Mono',monospace;color:#94a3b8;margin-bottom:18px;">Jurisdicción Federal Mexicana</div>
                                <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:16px;margin-bottom:12px;">
                                    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px;">
                                        <span style="font-size:16px;font-weight:800;color:#ffffff;">México</span>
                                        <span style="font-size:11px;font-family:'JetBrains Mono',monospace;color:#34d399;font-weight:700;">MXN ($)</span>
                                    </div>
                                    <p style="font-size:11px;color:#94a3b8;margin:0 0 12px 0;line-height:1.45;">
                                        STPS (39 NOMs) • SAT Art. 69-B • IMSS • LFPDPPP (SABG) • PROFECO
                                    </p>
                                    <button type="button" data-loc="es-MX" class="mallen-lang-card-btn" style="width:100%;background:#10b981;color:#022c22;font-weight:800;font-size:12px;padding:10px 14px;border:none;border-radius:8px;cursor:pointer;text-align:center;">
                                        Español (es-MX)
                                    </button>
                                </div>
                            </div>

                        </div>

                        <!-- BLOQUE DACH -->
                        <div style="background:#0f172a;border:1px solid #1e293b;border-top:4px solid #38bdf8;border-radius:16px;padding:24px;">
                            <div style="font-size:24px;font-weight:900;color:#38bdf8;letter-spacing:0.04em;margin-bottom:4px;">DACH</div>
                            <div style="font-size:11px;font-family:'JetBrains Mono',monospace;color:#94a3b8;margin-bottom:18px;">Mitteleuropa • EUR (€) & CHF (Fr.)</div>

                            <!-- Deutschland -->
                            <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:12px 14px;margin-bottom:10px;">
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:6px;">
                                    <span style="font-size:14px;font-weight:800;color:#ffffff;">Deutschland</span>
                                    <span style="font-size:10px;font-family:'JetBrains Mono',monospace;color:#38bdf8;">DSGVO • ArbSchG • GoBD</span>
                                </div>
                                <button type="button" data-loc="de-DE" class="mallen-lang-card-btn" style="width:100%;background:#0f172a;border:1px solid #38bdf8;color:#38bdf8;font-weight:700;font-size:11px;padding:7px 10px;border-radius:7px;cursor:pointer;">
                                    Deutsch (de-DE)
                                </button>
                            </div>

                            <!-- Österreich -->
                            <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:12px 14px;margin-bottom:10px;">
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:6px;">
                                    <span style="font-size:14px;font-weight:800;color:#ffffff;">Österreich</span>
                                    <span style="font-size:10px;font-family:'JetBrains Mono',monospace;color:#38bdf8;">DSB • ASchG • BAO • WiEReG</span>
                                </div>
                                <button type="button" data-loc="de-AT" class="mallen-lang-card-btn" style="width:100%;background:#0f172a;border:1px solid #38bdf8;color:#38bdf8;font-weight:700;font-size:11px;padding:7px 10px;border-radius:7px;cursor:pointer;">
                                    Deutsch (de-AT)
                                </button>
                            </div>

                            <!-- Schweiz / Suisse / Svizzera -->
                            <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:12px 14px;">
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px;">
                                    <span style="font-size:14px;font-weight:800;color:#ffffff;">Schweiz / Suisse / Svizzera</span>
                                    <span style="font-size:10px;font-family:'JetBrains Mono',monospace;color:#38bdf8;">revDSG • ArG • OR</span>
                                </div>
                                <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:6px;">
                                    <button type="button" data-loc="de-CH" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #38bdf8;color:#38bdf8;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Deutsch (de-CH)</button>
                                    <button type="button" data-loc="fr-CH" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #38bdf8;color:#38bdf8;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Français (fr-CH)</button>
                                    <button type="button" data-loc="it-CH" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #38bdf8;color:#38bdf8;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Italiano (it-CH)</button>
                                </div>
                            </div>
                        </div>

                        <!-- BLOQUE BENELUX -->
                        <div style="background:#0f172a;border:1px solid #1e293b;border-top:4px solid #a855f7;border-radius:16px;padding:24px;">
                            <div style="font-size:24px;font-weight:900;color:#c084fc;letter-spacing:0.04em;margin-bottom:4px;">BENELUX</div>
                            <div style="font-size:11px;font-family:'JetBrains Mono',monospace;color:#94a3b8;margin-bottom:18px;">Westeuropa • EUR (€)</div>

                            <!-- Nederland -->
                            <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:12px 14px;margin-bottom:10px;">
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:6px;">
                                    <span style="font-size:14px;font-weight:800;color:#ffffff;">Nederland</span>
                                    <span style="font-size:10px;font-family:'JetBrains Mono',monospace;color:#c084fc;">KVK • AVG • Arbowet • ACM</span>
                                </div>
                                <button type="button" data-loc="nl-NL" class="mallen-lang-card-btn" style="width:100%;background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 10px;border-radius:7px;cursor:pointer;">
                                    Nederlands (nl-NL)
                                </button>
                            </div>

                            <!-- België / Belgique -->
                            <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:12px 14px;margin-bottom:10px;">
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px;">
                                    <span style="font-size:14px;font-weight:800;color:#ffffff;">België / Belgique</span>
                                    <span style="font-size:10px;font-family:'JetBrains Mono',monospace;color:#c084fc;">KBO/BCE • GBA • Codex</span>
                                </div>
                                <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:6px;">
                                    <button type="button" data-loc="nl-BE" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Vlaams (nl-BE)</button>
                                    <button type="button" data-loc="fr-BE" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Français (fr-BE)</button>
                                    <button type="button" data-loc="de-BE" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Deutsch (de-BE)</button>
                                </div>
                            </div>

                            <!-- Lëtzebuerg / Luxembourg -->
                            <div style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:12px 14px;">
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px;">
                                    <span style="font-size:14px;font-weight:800;color:#ffffff;">Lëtzebuerg / Luxembourg</span>
                                    <span style="font-size:10px;font-family:'JetBrains Mono',monospace;color:#c084fc;">RCS • CNPD • ITM • RBE</span>
                                </div>
                                <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:6px;">
                                    <button type="button" data-loc="lb-LU" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Lëtzebuergesch</button>
                                    <button type="button" data-loc="fr-LU" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Français (fr-LU)</button>
                                    <button type="button" data-loc="de-LU" class="mallen-lang-card-btn" style="background:#0f172a;border:1px solid #c084fc;color:#c084fc;font-weight:700;font-size:11px;padding:7px 6px;border-radius:7px;cursor:pointer;">Deutsch (de-LU)</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- PASO 2: CONSENTIMIENTO DE PRIVACIDAD Y COOKIES EN EL IDIOMA DE LA JURISDICCIÓN ELEGIDA -->
                <div id="mallen-step-privacy-consent" style="display:none;max-width:740px;margin:20px auto 0 auto;background:#0f172a;border:1px solid #334155;border-radius:20px;padding:28px;">
                    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:14px;flex-wrap:wrap;gap:8px;">
                        <span id="mallen-consent-badge" style="font-family:'JetBrains Mono',monospace;font-size:11px;font-weight:800;color:#38bdf8;background:#020617;padding:5px 12px;border-radius:6px;border:1px solid #1e293b;"></span>
                        <button type="button" id="mallen-back-to-langs" style="background:transparent;border:1px solid #334155;color:#cbd5e1;font-size:11px;padding:5px 10px;border-radius:6px;cursor:pointer;">
                            ←
                        </button>
                    </div>
                    <h2 id="mallen-consent-title" style="font-size:20px;font-weight:800;color:#ffffff;margin:0 0 12px 0;"></h2>
                    <div id="mallen-consent-body" style="background:#020617;border:1px solid #1e293b;border-radius:12px;padding:16px;font-size:13px;color:#cbd5e1;line-height:1.6;margin-bottom:18px;"></div>
                    <label style="display:flex;align-items:flex-start;gap:10px;font-size:13px;color:#f8fafc;cursor:pointer;margin-bottom:20px;font-weight:600;">
                        <input type="checkbox" id="mallen-local-cookie-check" checked style="margin-top:3px;width:18px;height:18px;accent-color:#10b981;">
                        <span id="mallen-consent-check-label"></span>
                    </label>
                    <button type="button" id="mallen-confirm-enter-btn" style="width:100%;background:linear-gradient(90deg,#2563eb,#059669);color:#ffffff;font-weight:800;font-size:14px;padding:14px;border:none;border-radius:12px;cursor:pointer;"></button>
                </div>
            </div>

            <div style="text-align:center;font-family:'JetBrains Mono',monospace;font-size:11px;color:#64748b;padding-top:20px;">
                FIRMA W16 MALLEN® • SHA-256 & L2 MERKLE ANCHORED GRC SYSTEM • ALL RIGHTS RESERVED
            </div>
        `;

        document.body.appendChild(overlay);

        let pickedLocale = "es-MX";

        overlay.querySelectorAll(".mallen-lang-card-btn").forEach(function (btn) {
            btn.addEventListener("click", function () {
                pickedLocale = btn.getAttribute("data-loc") || "es-MX";
                const info = REGIONAL_PORTAL_DATA[pickedLocale] || REGIONAL_PORTAL_DATA["es-MX"];
                document.getElementById("mallen-step-language-select").style.display = "none";
                document.getElementById("mallen-step-privacy-consent").style.display = "block";
                document.getElementById("mallen-consent-badge").textContent = info.block + " • " + info.name + " • " + info.langName;
                document.getElementById("mallen-consent-title").textContent = info.privacyTitle;
                document.getElementById("mallen-consent-body").textContent = info.privacyBody;
                document.getElementById("mallen-consent-check-label").textContent = info.cookieCheckLabel;
                document.getElementById("mallen-confirm-enter-btn").textContent = info.enterBtn;
            });
        });

        document.getElementById("mallen-back-to-langs").addEventListener("click", function () {
            document.getElementById("mallen-step-privacy-consent").style.display = "none";
            document.getElementById("mallen-step-language-select").style.display = "block";
        });

        document.getElementById("mallen-confirm-enter-btn").addEventListener("click", function () {
            const chk = document.getElementById("mallen-local-cookie-check");
            if (chk && !chk.checked) return;
            const info = REGIONAL_PORTAL_DATA[pickedLocale] || REGIONAL_PORTAL_DATA["es-MX"];
            localStorage.setItem("mallen_consented_locale_v3", pickedLocale);
            localStorage.setItem("mallen_country", info.country);
            localStorage.setItem("mallen_locale", pickedLocale);

            if (info.country === "MX") {
                if (window.location.pathname.indexOf("/region/") === 0) {
                    window.location.href = "/?country=MX";
                } else {
                    overlay.remove();
                    injectSwitchRegionButtonInMexico();
                }
            } else {
                window.location.href = "/region/" + info.country.toLowerCase() + "/?lang=" + encodeURIComponent(pickedLocale);
            }
        });
    }

    window.openMallenGateway = function () {
        buildFullScreenGateway(true);
    };

    function injectSwitchRegionButtonInMexico() {
        if (document.getElementById("mallen-mx-switch-region-pill")) return;
        const pill = document.createElement("button");
        pill.id = "mallen-mx-switch-region-pill";
        pill.type = "button";
        pill.onclick = function () { buildFullScreenGateway(true); };
        pill.style.cssText = "position:fixed;bottom:18px;left:18px;z-index:9990;background:#0f172a;color:#38bdf8;border:1px solid #334155;border-radius:999px;padding:8px 16px;font-family:'JetBrains Mono',monospace;font-size:11px;font-weight:700;cursor:pointer;box-shadow:0 10px 25px rgba(0,0,0,0.5);";
        pill.textContent = "México (es-MX) • Cambiar País / DACH & Benelux";
        document.body.appendChild(pill);
    }

    function renderEuCountryPortalPage() {
        const pathParts = window.location.pathname.split("/").filter(Boolean);
        const countryCode = (pathParts[1] || "de").toUpperCase();
        const urlParams = new URLSearchParams(window.location.search);
        const supportedLocales = COUNTRY_DIALECTS[countryCode] || ["de-DE"];
        let activeLocale = urlParams.get("lang") || localStorage.getItem("mallen_locale") || supportedLocales[0];
        if (supportedLocales.indexOf(activeLocale) === -1) {
            activeLocale = supportedLocales[0];
        }

        const data = REGIONAL_PORTAL_DATA[activeLocale] || REGIONAL_PORTAL_DATA["de-DE"];
        localStorage.setItem("mallen_country", countryCode);
        localStorage.setItem("mallen_locale", activeLocale);

        document.documentElement.lang = activeLocale;
        document.title = "FIRMA W16 MALLEN® — " + data.name + " (" + activeLocale + ")";

        document.getElementById("eu-block-pill").textContent = data.block;
        document.getElementById("eu-country-name").textContent = data.name;
        document.getElementById("eu-currency-badge").textContent = data.currency;

        const sel = document.getElementById("eu-dialect-select");
        sel.innerHTML = "";
        supportedLocales.forEach(function (locCode) {
            const locInfo = REGIONAL_PORTAL_DATA[locCode];
            if (!locInfo) return;
            const opt = document.createElement("option");
            opt.value = locCode;
            opt.textContent = locInfo.langName;
            if (locCode === activeLocale) opt.selected = true;
            sel.appendChild(opt);
        });
        sel.onchange = function (e) {
            localStorage.setItem("mallen_locale", e.target.value);
            window.location.search = "?lang=" + encodeURIComponent(e.target.value);
        };

        document.getElementById("eu-hero-badge").textContent = data.heroBadge;
        document.getElementById("eu-crypto-law-badge").textContent = data.cryptoLaw;
        document.getElementById("eu-hero-title").textContent = data.heroTitle;
        document.getElementById("eu-hero-subtitle").textContent = data.heroSubtitle;
        document.getElementById("eu-tax-label").textContent = data.taxLabel;
        document.getElementById("eu-tax-input").placeholder = data.taxPlaceholder;
        document.getElementById("eu-domain-input").placeholder = data.domainPlaceholder;
        document.getElementById("eu-scan-btn").textContent = data.scanBtn;

        document.getElementById("eu-section-title").textContent = data.sectionTitle;
        document.getElementById("eu-section-sub").textContent = data.sectionSub;
        document.getElementById("eu-kit-folder-badge").textContent = "Ruta: " + data.templatePath;
        document.getElementById("eu-crypto-law-footer").textContent = data.cryptoLaw;
        document.getElementById("eu-disclaimer-footer").textContent = data.privacyBody;
        window.applyNativePortalTranslationsAndSdui(countryCode, activeLocale, data);

        const cardsContainer = document.getElementById("eu-laws-cards-container");
        cardsContainer.innerHTML = "";
        (data.laws || []).forEach(function (law) {
            const card = document.createElement("div");
            card.className = "quantum-card rounded-2xl p-6 border-l-4 " + law.color + " flex flex-col justify-between";
            card.innerHTML = `
                <div>
                    <div class="flex items-center justify-between gap-2 mb-3">
                        <span class="text-[10px] font-mono-code font-bold uppercase px-2.5 py-1 rounded bg-slate-900 border border-slate-700 text-slate-200">${law.badge}</span>
                        <span class="text-[10px] font-mono-code text-emerald-400 font-bold">${data.country}</span>
                    </div>
                    <h3 class="text-base font-extrabold text-white mb-1.5">${law.title}</h3>
                    <p class="text-[11px] font-mono-code text-sky-400 mb-3">${law.ref}</p>
                    <p class="text-xs text-slate-300 leading-relaxed mb-4">${law.desc}</p>
                </div>
                <div class="pt-3 border-t border-slate-800/80 flex items-center justify-between text-[11px] font-mono-code">
                    <span class="text-amber-400 font-semibold">${law.status}</span>
                    <span class="text-slate-400">SHA-256</span>
                </div>
            `;
            cardsContainer.appendChild(card);
        });

        const delivContainer = document.getElementById("eu-deliverables-list");
        if (delivContainer) {
            delivContainer.innerHTML = "";
            (data.deliverables || []).forEach(function (docTitle) {
                const item = document.createElement("div");
                item.className = "bg-slate-950/90 border border-slate-800 rounded-xl p-3.5 text-slate-200 flex items-start justify-between gap-2";
                item.innerHTML = `<span>${docTitle}</span><span class="text-emerald-400 font-bold shrink-0">PDF</span>`;
                delivContainer.appendChild(item);
            });
        }

        window.runEuRegionalSimulation = function () {
            const taxVal = document.getElementById("eu-tax-input").value.trim() || ("ID-" + data.country + "-2026");
            const domVal = document.getElementById("eu-domain-input").value.trim() || ("unternehmen." + data.country.toLowerCase());
            const prev = document.getElementById("eu-scan-preview");
            prev.classList.remove("hidden");
            document.getElementById("eu-preview-title").textContent = "DIAGNÓSTICO PRELIMINAR GENERADO • " + taxVal + " (" + domVal + ")";
            document.getElementById("eu-preview-price").textContent = "Expediente Completo: " + data.kitPrice;
            document.getElementById("eu-sim-fine").textContent = data.simulatedFine;
            document.getElementById("eu-preview-body").textContent = "Evaluación determinística sobre los 37 vectores de cumplimiento de " + data.name + " (" + data.langName + "). Los 7 contratos y protocolos listos para firma se encuentran pre-estructurados en " + data.templatePath + ".";
            document.getElementById("eu-fine-note").textContent = data.fineNote;
        };
    }

    function boot() {
        const oldBar = document.getElementById("mallen-global-jurisdiction-bar");
        if (oldBar) oldBar.remove();

        const urlParams = new URLSearchParams(window.location.search);
        if (urlParams.get("country") === "MX") {
            localStorage.setItem("mallen_consented_locale_v3", "es-MX");
            localStorage.setItem("mallen_country", "MX");
        }

        if (window.location.pathname.indexOf("/region/") === 0) {
            renderEuCountryPortalPage();
            buildFullScreenGateway(false);
        } else {
            const consented = localStorage.getItem("mallen_consented_locale_v3");
            const savedCountry = localStorage.getItem("mallen_country");
            if (!consented) {
                buildFullScreenGateway(true);
            } else if (savedCountry && savedCountry !== "MX" && !urlParams.get("country")) {
                window.location.href = "/region/" + savedCountry.toLowerCase() + "/?lang=" + encodeURIComponent(consented);
            } else {
                injectSwitchRegionButtonInMexico();
            }
        }
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", boot);
    } else {
        boot();
    }
})();
