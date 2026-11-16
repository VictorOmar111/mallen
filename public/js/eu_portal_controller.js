/**
 * FIRMA MALLEN® — European DACH & Benelux Controller (de, fr, it, nl)
 * 100% traducido, sin fugas de rutas de servidor, sin términos internos crudos.
 */
(function () {
    "use strict";

    const EU_CONFIG = {
        de: {
            block: "DACH",
            countryName: "Deutschland",
            currency: "290,00 €",
            dialects: [{ code: "de-DE", label: "Deutsch (Deutschland)" }],
            regCount: "+3.8M Handelsregister & Bundesanzeiger",
            cryptoLaw: "eIDAS Art. 41 • GoBD • SHA-256",
            laws: [
                { code: "DSGVO / BDSG", rate: "68%", title: "Datenschutz-Grundverordnung & TDDDG", desc: "Prüfung von Datenschutzerklärung, Auftragsverarbeitung (Art. 28 DSGVO) und TLS-Verschlüsselung." },
                { code: "ArbSchG / DGUV", rate: "74%", title: "Arbeitsschutzgesetz & Gefährdungsbeurteilung", desc: "Dokumentation psychischer Belastungen (§ 5 ArbSchG) und Arbeitsschutzausschuss (§ 11 ASiG)." },
                { code: "GoBD / AO", rate: "61%", title: "Steuerliche Verfahrensdokumentation & KassenSichV", desc: "Revisionssichere elektronische Belegablage und Nachweis der wirtschaftlichen Substanz." },
                { code: "LkSG / GwG", rate: "53%", title: "Lieferkettensorgfaltspflichten & Geldwäscheprävention", desc: "Transparenzregister-Abgleich (wirtschaftlich Berechtigter) und Lieferanten-Compliance." },
                { code: "NIS-2 / BSI", rate: "64%", title: "IT-Sicherheitsgesetz & NIS-2 Richtlinie", desc: "Technische und organisatorische Maßnahmen (TOM) sowie Meldepflichten für Sicherheitsvorfälle." },
                { code: "BGB / UWG", rate: "49%", title: "Digitale Verbraucherverträge & Impressumspflicht (§ 5 DDG)", desc: "Widerrufsbelehrung, Preisangabenverordnung (PAngV) und barrierefreie AGB." }
            ],
            registries: [
                "1. Gemeinsames Registerportal der Länder (Handelsregister A/B)",
                "2. Bundesanzeiger & Unternehmensregister",
                "3. BZSt — USt-IdNr. Validierung",
                "4. Transparenzregister (Wirtschaftlich Berechtigte)",
                "5. BAuA / DGUV Branchen-Risikomatrix",
                "6. BSI / TLS-Zertifikats- & Domain-Register"
            ]
        },
        at: {
            block: "DACH",
            countryName: "Österreich",
            currency: "290,00 €",
            dialects: [{ code: "de-AT", label: "Deutsch (Österreich)" }],
            regCount: "+610K Firmenbuch & WKO",
            cryptoLaw: "eIDAS Art. 41 • BAO • SHA-256",
            laws: [
                { code: "DSGVO / DSG", rate: "66%", title: "Österreichisches Datenschutzgesetz & TKG 2021", desc: "Prüfung der Informationspflichten, Verarbeitungsverzeichnisse und Datensicherheit." },
                { code: "ASchG", rate: "72%", title: "ArbeitnehmerInnenschutzgesetz & Arbeitsplatzevaluierung", desc: "Evaluierung psychischer Belastungen und Sicherheitsvertrauenspersonen (SVP)." },
                { code: "BAO / UGB", rate: "59%", title: "Bundesabgabenordnung & Aufbewahrungspflichten (§ 132 BAO)", desc: "Ordnungsmäßige Buchführung, Belegnachweis und RKSV-Konformität." },
                { code: "WiEReG / FM-GwG", rate: "54%", title: "Wirtschaftliche Eigentümer Registergesetz", desc: "Jährliche Sorgfalts- und Meldepflichten zum wirtschaftlichen Eigentümer." },
                { code: "NISG", rate: "62%", title: "Netz- und Informationssystemsicherheitsgesetz", desc: "Risikomanagementmaßnahmen und Kryptografie-Vorgaben." },
                { code: "KSchG / ECG", rate: "48%", title: "Konsumentenschutzgesetz & E-Commerce-Gesetz (§ 5 ECG)", desc: "Offenlegungspflichten, Fernabsatz-Widerruf und AGB-Transparenz." }
            ],
            registries: [
                "1. Firmenbuch (Justiz Republik Österreich)",
                "2. WKO Firmen A-Z Verzeichnis",
                "3. FinanzOnline UID-Prüfung",
                "4. Register der wirtschaftlichen Eigentümer (WiEReG)",
                "5. Arbeitsinspektorat Branchenprofil",
                "6. RTR / TLS- & Domain-Integritätsprüfung"
            ]
        },
        ch: {
            block: "DACH",
            countryName: "Schweiz / Suisse / Svizzera",
            currency: "CHF 290.00",
            dialects: [
                { code: "de-CH", label: "Deutsch (Schweiz)" },
                { code: "fr-CH", label: "Français (Suisse)" },
                { code: "it-CH", label: "Italiano (Svizzera)" }
            ],
            regCount: "+680K Zefix & UID-Register BFS",
            cryptoLaw: "ZertES • revDSG • SHA-256",
            laws: [
                { code: "revDSG / nLPD", rate: "69%", title: "Revidiertes Datenschutzgesetz (EDÖB / PFPDT)", desc: "Datenschutzerklärung, Verzeichnis der Bearbeitungstätigkeiten und grenzüberschreitende Bekanntgabe." },
                { code: "ArG / UVG / EKAS", rate: "71%", title: "Arbeitsgesetz, Unfallversicherung & EKAS-Richtlinie 6508", desc: "Gesundheitsschutz am Arbeitsplatz, Arbeitszeiterfassung und Gefahrenermittlung." },
                { code: "OR / GeBüV", rate: "58%", title: "Obligationenrecht (Art. 957a OR) & Geschäftsbücherverordnung", desc: "Unveränderbare elektronische Aufbewahrung von Geschäftsbüchern und Buchungsbelegen." },
                { code: "GwG / LBA", rate: "52%", title: "Geldwäschereigesetz & Feststellung des wirtschaftlich Berechtigten", desc: "Sorgfaltspflichten nach Art. 697j OR und Dokumentation der Kontrollinhaber." },
                { code: "ISG / BACS", rate: "63%", title: "Informationssicherheitsgesetz & Meldepflicht für Cyberangriffe", desc: "Technische Schutzmassnahmen für kritische Infrastrukturen und Unternehmensdaten." },
                { code: "UWG / LCD", rate: "47%", title: "Bundesgesetz gegen den unlauteren Wettbewerb (Art. 3 Abs. 1 lit. s UWG)", desc: "Pflichtangaben im elektronischen Geschäftsverkehr und Preisbekanntgabeverordnung (PBV)." }
            ],
            registries: [
                "1. Zefix — Zentraler Firmenindex des Bundes",
                "2. BFS UID-Register (CHE-Unternehmens-Identifikation)",
                "3. ESTV Mehrwertsteuerregister",
                "4. SECO / SUVA Gefahrenklassen-Register",
                "5. EDÖB Datenschutz-Kontrollmatrix",
                "6. SWITCH / TLS- & Domain-Sicherheitsprüfung"
            ]
        },
        nl: {
            block: "BENELUX",
            countryName: "Nederland",
            currency: "290,00 €",
            dialects: [{ code: "nl-NL", label: "Nederlands (Nederland)" }],
            regCount: "+2.4M KVK Handelsregister",
            cryptoLaw: "eIDAS Art. 41 • AVG • SHA-256",
            laws: [
                { code: "AVG / UAVG", rate: "67%", title: "Algemene Verordening Gegevensbescherming (AP)", desc: "Verwerkingsregister (Art. 30 AVG), verwerkersovereenkomsten en beveiliging van persoonsgegevens." },
                { code: "Arbowet", rate: "75%", title: "Arbeidsomstandighedenwet & RI&E Verplichting", desc: "Risico-inventarisatie en -evaluatie (RI&E), psychosociale arbeidsbelasting (PSA) en thuiswerkbeleid." },
                { code: "AWR / Wet OB", rate: "60%", title: "Algemene wet inzake rijksbelastingen (Art. 52 AWR)", desc: "Fiscale bewaarplicht (7 jaar) en aantoonbaarheid van zakelijke transacties." },
                { code: "Wwft / UBO", rate: "56%", title: "Wet ter voorkoming van witwassen en financieren van terrorisme", desc: "UBO-registratie bij KVK en cliëntenonderzoek (KYC)." },
                { code: "Cyberbeveiligingswet", rate: "63%", title: "NIS2 / Cyberbeveiligingswet & NCSC Richtlijnen", desc: "Zorgplicht voor netwerk- en informatiesystemen en meldplicht incidenten." },
                { code: "BW Boek 6 / ACM", rate: "49%", title: "Consumentenrecht & Elektronisch Rechtsverkeer", desc: "Informatieplichten bij verkoop op afstand, herroepingsrecht en transparante prijzen." }
            ],
            registries: [
                "1. KVK Handelsregister (Kamer van Koophandel)",
                "2. Belastingdienst BTW-identificatienummer",
                "3. KVK UBO-register (Uiteindelijk Belanghebbenden)",
                "4. Nederlandse Arbeidsinspectie SBI-Risicoprofiel",
                "5. Autoriteit Persoonsgegevens (AP) Toetsingskader",
                "6. SIDN / TLS- & Domeinbeveiliging"
            ]
        },
        be: {
            block: "BENELUX",
            countryName: "België / Belgique",
            currency: "290,00 €",
            dialects: [
                { code: "nl-BE", label: "Nederlands (Vlaanderen)" },
                { code: "fr-BE", label: "Français (Belgique)" }
            ],
            regCount: "+1.3M KBO / BCE Kruispuntbank",
            cryptoLaw: "eIDAS Art. 41 • WER/CDE • SHA-256",
            laws: [
                { code: "AVG / RGPD (GBA/APD)", rate: "68%", title: "Gegevensbescherming & Elektronische Communicatie", desc: "Privacyverklaring, verwerkersovereenkomsten en beveiligingsmaatregelen (Art. 32)." },
                { code: "Codex Welzijn / Bien-être", rate: "73%", title: "Welzijnswet & Psychosociale Risico's op het Werk", desc: "Globaal preventieplan, jaarlijks actieplan en telewerkovereenkomsten (CAO nr. 85)." },
                { code: "WIB / CIR 92", rate: "61%", title: "Fiscale Bewaarplicht & Elektronische Facturatie (B2B)", desc: "Bewaring van boeken en bescheiden en materiële bewijskracht van diensten." },
                { code: "Anti-witwaswet / UBO", rate: "55%", title: "Wet tot voorkoming van het witwassen & UBO-register (FOD Financiën)", desc: "Jaarlijkse bevestiging van uiteindelijke begunstigden en cliëntidentificatie." },
                { code: "NIS2 België (CCB)", rate: "65%", title: "NIS2-wetgeving & Cyberfundamentals (CyFun)", desc: "Beveiliging van ketens, risicobeheer en incidentmelding bij het CCB." },
                { code: "WER / CDE Boek VI", rate: "48%", title: "Wetboek van Economisch Recht & E-Commerce", desc: "Precontractuele informatie, verzakingsrecht en transparante voorwaarden." }
            ],
            registries: [
                "1. KBO / BCE — Kruispuntbank van Ondernemingen",
                "2. Belgisch Staatsblad / Moniteur Belge",
                "3. FOD Financiën — UBO-register",
                "4. RSZ / ONSS Werkgevers- & NACE-BEL Klasse",
                "5. GBA / APD Gegevensbeschermingsregister",
                "6. DNS Belgium / TLS- & Domeincontrole"
            ]
        },
        lu: {
            block: "BENELUX",
            countryName: "Luxembourg",
            currency: "290,00 €",
            dialects: [
                { code: "fr-LU", label: "Français (Luxembourg)" },
                { code: "de-LU", label: "Deutsch (Luxemburg)" }
            ],
            regCount: "+145K LBR / RCS Luxembourg",
            cryptoLaw: "eIDAS Art. 41 • CNPD • SHA-256",
            laws: [
                { code: "RGPD / CNPD", rate: "66%", title: "Loi du 1er août 2018 & Protection des Données (CNPD)", desc: "Registre des activités de traitement, sous-traitance (Art. 28) et sécurité TLS." },
                { code: "Code du Travail (ITM)", rate: "70%", title: "Sécurité et Santé au Travail & Convention Télétravail", desc: "Évaluation des risques professionnels, délégation du personnel et droit à la déconnexion." },
                { code: "Code de Commerce / AO", rate: "58%", title: "Conservation décennale des documents comptables & TVA", desc: "Intégrité des pièces justificatives et justification économique des prestations." },
                { code: "Loi AML / RBE", rate: "57%", title: "Lutte contre le blanchiment & Registre des Bénéficiaires Effectifs", desc: "Déclaration exacte au LBR (RBE) et obligations de vigilance professionnelle." },
                { code: "NIS2 / ILR", rate: "64%", title: "Sécurité des réseaux et des systèmes d'information", desc: "Mesures techniques de gouvernance cyber et notification des incidents." },
                { code: "Code de la Consommation", rate: "46%", title: "Commerce électronique & Protection des consommateurs", desc: "Mentions légales obligatoires, droit de rétractation et transparence tarifaire." }
            ],
            registries: [
                "1. LBR — Registre de Commerce et des Sociétés (RCS)",
                "2. RBE — Registre des Bénéficiaires Effectifs",
                "3. AED — Administration de l'Enregistrement (TVA)",
                "4. ITM — Inspection du Travail et des Mines (NACE)",
                "5. CNPD — Référentiel de Conformité RGPD",
                "6. RESTENA / TLS & Sécurité de Domaine"
            ]
        }
    };

    const LANG_TEXT = {
        de: {
            headerSub: "PRÄVENTIVE RECHTS- UND COMPLIANCE-TECHNOLOGIE",
            navCal: "Diagnose",
            navTrust: "Trust Center & Lieferumfang",
            navProof: "Beweiskraft eIDAS",
            navCov: "Rechtsrahmen",
            navPartner: "Kanzlei-Portal",
            heroBadge: "DETERMINISTISCHE COMPLIANCE-PRÜFUNG (ISO 19011)",
            heroTitle: "Präventive Rechts- und Risikoanalyse für Ihr Unternehmen in Echtzeit",
            heroSub: "Prüfen Sie Ihre Handelsregister-, Datenschutz-, Arbeitsschutz- und Steuer-Compliance in zwei Schritten. Alle 16 Vertragsvariablen für Ihre 7 Rechtsinstrumente werden erst nach dem Checkout in einem geführten Assistenten abgefragt.",
            taxLabel: "1. USt-IdNr., Handelsregister-Nr. (HRB/FN/UID) oder Firmenname",
            taxMicro: "Wird ausschließlich für den Abgleich mit öffentlichen Registern und die Rechnungsstellung verwendet.",
            domLabel: "2. Unternehmens-Domain (Optional für TLS/DSGVO-Prüfung)",
            domMicro: "Prüft ausschliesslich öffentlich erreichbare SSL/TLS-Header und Datenschutzerklärungen.",
            empLabel: "Mitarbeiteranzahl (Richtwert)",
            empMicro: "Dient zur exakten Skalierung der Arbeitsschutz-Schwellenwerte.",
            scanBtn: "Präventive Diagnose starten",
            demoBtn: "Vollständiges Muster-Gutachten ansehen (PDF)",
            deferNote: "Vertragsparametrisierung (Vertretungsberechtigte, Ausschüsse) erfolgt erst bequem nach der Aktivierung.",
            trustKicker: "TRUST CENTER & LIEFERUMFANG (TRANSPARENZ-MATRIX)",
            trustTitle: "Was unser System automatisiert prüft und welche Dokumente Sie sofort erhalten",
            col1Title: "1. Automatisierte Systemprüfung",
            col1List: [
                "• Abgleich mit nationalen Unternehmensregistern und NACE-Branchenprofilen.",
                "• Deterministische Berechnung über 37 regulatorische Prüfvektoren.",
                "• Primärbeweissicherung Ihrer Website (TLS 1.3, DSGVO-Hinweise) mit SHA-256 Hash."
            ],
            col2Title: "2. Enthaltene 7 Verträge & 10 Zertifikate (.ZIP)",
            col2List: [
                "• Dreiseitiges Sachverständigengutachten (PDF) + 10 SHA-256 Einzelzertifikate.",
                "• 01. Datenschutzerklärung & Auftragsverarbeitung (DSGVO/revDSG).",
                "• 02. Richtlinie zur psychischen Gefährdungsbeurteilung.",
                "• 03. Konstituierende Akte des Arbeitsschutzausschusses (4 Quartalsprüfungen).",
                "• 04. Telearbeits- und Homeoffice-Vereinbarung.",
                "• 05. Rahmenvertrag zur steuerlichen Substanz & Leistungsdokumentation.",
                "• 06. Betrieblicher Brandschutz- & Notfallplan.",
                "• 07. E-Commerce AGB & Dokumentation der wirtschaftlich Berechtigten.",
                "• Geliefert als versiegeltes PDF + bearbeitbare Word-HTML-Datei."
            ],
            col3Title: "3. Transparente Konditionen",
            planSingle: "Komplettpaket 360° (Einzelunternehmen)",
            descSingle: "Einmalzahlung pro Rechtsträger • Sofort-Download nach Freigabe • Inkl. 90-Tage-Überwachung.",
            planPartner: "Kanzlei- & Steuerberater-Tarif",
            descPartner: "Mandanten-Stapelprüfung (CSV) und White-Label-Berichte im Kanzlei-Portal.",
            scopeBoundary: "Abgrenzung: Das System erstellt die branchenspezifische Präventivdokumentation; die rechtsverbindliche Unterzeichnung vor Ort obliegt der Geschäftsführung.",
            proofKicker: "BEWEISKRAFT & KRYPTOGRAFISCHE INTEGRITÄT",
            proofTitle: "Revisionssichere Dokumentation nach ISO 19011 und eIDAS Art. 41",
            proofSub: "Jedes Dokument wird mit einem deterministischen SHA-256 Hash und kanonischer RFC-8785-Signatur versiegelt.",
            proofCards: [
                { t: "eIDAS Art. 41 / ZertES", d: "Qualifizierter Zeitstempel-Nachweis und kryptografische Unveränderbarkeit." },
                { t: "ISO 19011 Methodik", d: "Reproduzierbare Bewertungsmatrix über alle 37 Compliance-Vektoren." },
                { t: "SHA-256 Merkle-Baum", d: "Manipulation von nur einem Bit verändert sofort den Prüfsiegel-Hash." },
                { t: "Datensparsamkeit (Art. 5 DSGVO)", d: "Verschlüsselte Speicherung ohne Weitergabe an Dritte." }
            ],
            dirKicker: "BRANCHENSPEZIFISCHE RECHTSABDECKUNG",
            dirTitle: "6 Geprüfte Rechtsgebiete Ihrer Jurisdiktion"
        },
        fr: {
            headerSub: "INGÉNIERIE JURIDIQUE PRÉVENTIVE",
            navCal: "Diagnostic",
            navTrust: "Centre de Confiance & Livrables",
            navProof: "Force Probante eIDAS",
            navCov: "Cadre Normatif",
            navPartner: "Portail Fiduciaires",
            heroBadge: "AUDIT DE CONFORMITÉ DÉTERMINISTE (ISO 19011)",
            heroTitle: "Diagnostic juridique préventif et analyse de risques en temps réel",
            heroSub: "Vérifiez votre conformité RGPD/nLPD, droit du travail et fiscalité en deux champs. La personnalisation des 16 clauses nominatives s'effectue après le paiement via un assistant guidé.",
            taxLabel: "1. N° TVA / UID / BCE-KBO / RCS ou Raison Sociale",
            taxMicro: "Utilisé exclusivement pour la vérification des registres officiels et la facturation.",
            domLabel: "2. Domaine Web de l'entreprise (Optionnel — TLS/RGPD)",
            domMicro: "Analyse uniquement les en-têtes publics SSL/TLS et la politique de confidentialité.",
            empLabel: "Effectif (Collaborateurs)",
            empMicro: "Permet de calibrer les seuils légaux de sécurité au travail.",
            scanBtn: "Lancer le diagnostic préventif",
            demoBtn: "Voir un rapport complet d'exemple (PDF)",
            deferNote: "La saisie des représentants légaux et comités internes est reportée après validation pour zéro friction.",
            trustKicker: "CENTRE DE CONFIANCE & LIVRABLES (TRANSPARENCE)",
            trustTitle: "Ce que notre moteur vérifie automatiquement et ce que vous téléchargez immédiatement",
            col1Title: "1. Vérification Automatisée",
            col1List: [
                "• Croisement avec les registres nationaux et la nomenclature NACE.",
                "• Calcul déterministe sur 37 vecteurs réglementaires.",
                "• Preuve technique du site web (TLS 1.3, mentions RGPD/nLPD) scellée par SHA-256."
            ],
            col2Title: "2. Les 7 Contrats & 10 Certificats inclus (.ZIP)",
            col2List: [
                "• Rapport d'expertise de 3 pages (PDF) + 10 Certificats SHA-256.",
                "• 01. Politique de Confidentialité & Sous-traitance (RGPD/nLPD).",
                "• 02. Politique de Prévention des Risques Psychosociaux.",
                "• 03. Acte du Comité de Sécurité et Santé (4 revues trimestrielles).",
                "• 04. Convention et Avenant de Télétravail.",
                "• 05. Contrat-Cadre de Matérialité Fiscale et Prestations.",
                "• 06. Plan Interne de Sécurité Incendie et Évacuation.",
                "• 07. CGV E-Commerce & Registre des Bénéficiaires Effectifs (UBO/RBE).",
                "• Livrés en double format : PDF scellé + HTML éditable dans Word."
            ],
            col3Title: "3. Tarification Transparente",
            planSingle: "Dossier Intégral 360° (Par Entité)",
            descSingle: "Paiement unique • Téléchargement immédiat • Surveillance de 90 jours incluse.",
            planPartner: "Offre Fiduciaires & Cabinets",
            descPartner: "Audit par lots (CSV) et rapports en marque blanche.",
            scopeBoundary: "Transparence : Le système compile l'ingénierie documentaire préventive adaptée à votre code NACE ; la signature finale relève de la direction.",
            proofKicker: "GARANTIES PROBATOIRES",
            proofTitle: "Intégrité cryptographique selon ISO 19011 et eIDAS Art. 41",
            proofSub: "Chaque livrable comporte une empreinte SHA-256 et une signature canonique RFC 8785.",
            proofCards: [
                { t: "eIDAS Art. 41 / ZertES", d: "Horodatage cryptographique et preuve d'intégrité documentaire." },
                { t: "Norme ISO 19011", d: "Méthodologie d'audit reproductible sur 37 vecteurs." },
                { t: "Arbre de Merkle SHA-256", d: "Détection immédiate de toute altération d'un seul bit." },
                { t: "Minimisation RGPD", d: "Protection stricte des données sans cession à des tiers." }
            ],
            dirKicker: "COUVERTURE RÉGLEMENTAIRE NATIONALE",
            dirTitle: "6 Piliers Légaux Audités dans Votre Juridiction"
        },
        it: {
            headerSub: "INGEGNERIA LEGALE PREVENTIVA",
            navCal: "Diagnosi",
            navTrust: "Centro di Fiducia & Livrabili",
            navProof: "Valore Probatorio ZertES",
            navCov: "Quadro Normativo",
            navPartner: "Portale Studi",
            heroBadge: "VERIFICA DI CONFORMITÀ DETERMINISTICA (ISO 19011)",
            heroTitle: "Diagnosi legale preventiva e analisi del rischio in tempo reale",
            heroSub: "Verificate la conformità nLPD/revDSG, sicurezza sul lavoro e fiscalità in due campi. La personalizzazione dei 16 campi contrattuali avviene dopo il pagamento.",
            taxLabel: "1. Numero IDI / CHE o Ragione Sociale",
            taxMicro: "Utilizzato esclusivamente per il riscontro nei registri pubblici Zefix/UID e fatturazione.",
            domLabel: "2. Dominio Web Aziendale (Opzionale — TLS/nLPD)",
            domMicro: "Verifica solo gli header pubblici SSL/TLS e l'informativa sulla privacy.",
            empLabel: "Numero di collaboratori",
            empMicro: "Necessario per calibrare le soglie LAINF/LL.",
            scanBtn: "Avvia diagnosi preventiva",
            demoBtn: "Visualizza perizia completa di esempio (PDF)",
            deferNote: "I dati dei rappresentanti legali vengono richiesti solo dopo l'attivazione.",
            trustKicker: "CENTRO DI FIDUCIA & TRASPARENZA",
            trustTitle: "Cosa verifica il motore automatico e quali documenti ricevete subito",
            col1Title: "1. Verifica Automatizzata",
            col1List: [
                "• Riscontro immediato con Zefix, registro UID e profilo di rischio.",
                "• Calcolo deterministico su 37 vettori normativi.",
                "• Evidenza tecnica del sito web (TLS 1.3, informativa nLPD) con hash SHA-256."
            ],
            col2Title: "2. I 7 Contratti & 10 Certificati inclusi (.ZIP)",
            col2List: [
                "• Perizia esecutiva di 3 pagine (PDF) + 10 Certificati SHA-256.",
                "• 01. Informativa sulla Privacy e Responsabili (nLPD).",
                "• 02. Politica di Prevenzione dei Rischi Psicosociali.",
                "• 03. Verbale Commissione Sicurezza (4 ispezioni trimestrali).",
                "• 04. Accordo di Telelavoro.",
                "• 05. Contratto Quadro di Materialità Fiscale (OR/GeBüV).",
                "• 06. Piano Interno di Sicurezza e Protezione Antincendio.",
                "• 07. Condizioni E-Commerce (LCD) e Registro Aventi Diritto Economico.",
                "• Consegnati in doppio formato: PDF sigillato + HTML modificabile in Word."
            ],
            col3Title: "3. Investimento Trasparente",
            planSingle: "Fascicolo Integrale 360° (Svizzera)",
            descSingle: "Pagamento unico • Download immediato • Include sorveglianza per 90 giorni.",
            planPartner: "Piano Fiduciarie e Studi Legali",
            descPartner: "Audit multi-azienda (CSV) e report White-Label.",
            scopeBoundary: "Trasparenza: Il sistema compila la documentazione preventiva per il vostro settore; la firma autografa spetta alla direzione aziendale.",
            proofKicker: "GARANZIE PROBATORIE",
            proofTitle: "Integrità crittografica secondo ISO 19011 e ZertES",
            proofSub: "Ogni documento è sigillato con impronta SHA-256 e firma canonica RFC 8785.",
            proofCards: [
                { t: "ZertES / eIDAS", d: "Validazione temporale crittografica e integrità documentale." },
                { t: "Standard ISO 19011", d: "Matrice di audit riproducibile su 37 vettori." },
                { t: "Albero di Merkle SHA-256", d: "Rilevamento immediato di qualsiasi modifica." },
                { t: "Protezione Dati nLPD", d: "Crittografia rigorosa senza cessione a terzi." }
            ],
            dirKicker: "COPERTURA NORMATIVA SVIZZERA",
            dirTitle: "6 Ambiti Legali Verificati nella Vostra Giurisdizione"
        },
        nl: {
            headerSub: "PREVENTIEVE JURIDISCHE ENGINEERING",
            navCal: "Diagnose",
            navTrust: "Trust Center & Documenten",
            navProof: "Bewijskracht eIDAS",
            navCov: "Wettelijk Kader",
            navPartner: "Kantorenportaal",
            heroBadge: "DETERMINISTISCHE COMPLIANCE-AUDIT (ISO 19011)",
            heroTitle: "Preventieve juridische en risico-analyse voor uw onderneming",
            heroSub: "Controleer uw KVK/KBO-, AVG-, Arbo- en fiscale compliance in twee stappen. De 16 contractvelden voor uw 7 juridische instrumenten worden pas na betaling ingevuld.",
            taxLabel: "1. KVK-nummer, BTW-id, KBO-nummer of Bedrijfsnaam",
            taxMicro: "Uitsluitend gebruikt voor verificatie in openbare handelsregisters en facturatie.",
            domLabel: "2. Zakelijke Website (Optioneel — TLS/AVG-controle)",
            domMicro: "Controleert enkel publieke SSL/TLS-headers en de aanwezigheid van een privacyverklaring.",
            empLabel: "Aantal medewerkers",
            empMicro: "Voor exacte kalibratie van Arbo- en RI&E-drempelwaarden.",
            scanBtn: "Start preventieve diagnose",
            demoBtn: "Bekijk volledig voorbeeld-rapport (PDF)",
            deferNote: "Gegevens van bestuurders en interne commissies worden pas na activering gevraagd.",
            trustKicker: "TRUST CENTER & LEVERINGSOMVANG",
            trustTitle: "Wat onze engine automatisch verifieert en welke documenten u direct ontvangt",
            col1Title: "1. Geautomatiseerde Systeemcontrole",
            col1List: [
                "• Koppeling met KVK/KBO-handelsregisters en SBI/NACE-sectoren.",
                "• Deterministische berekening over 37 wettelijke controlevectoren.",
                "• Technische bewijsvastlegging van uw website (TLS 1.3, AVG) met SHA-256 hash."
            ],
            col2Title: "2. De 7 Contracten & 10 Certificaten inbegrepen (.ZIP)",
            col2List: [
                "• Drie pagina's tellend deskundigenrapport (PDF) + 10 SHA-256 Certificaten.",
                "• 01. Privacyverklaring & Verwerkersovereenkomst (AVG).",
                "• 02. Beleid Psychosociale Arbeidsbelasting (PSA / RI&E).",
                "• 03. Instellingsakte Arbo-/Preventiecommissie (4 kwartaalrondes).",
                "• 04. Thuiswerk- en Telewerkovereenkomst.",
                "• 05. Raamovereenkomst Fiscale Substantie & Dienstverlening.",
                "• 06. Intern Nood- en Bedrijfshulpverleningsplan (BHV).",
                "• 07. E-Commerce Voorwaarden & UBO-documentatie.",
                "• Geleverd als verzegelde PDF + bewerkbaar Word-HTML-bestand."
            ],
            col3Title: "3. Transparante Tarieven",
            planSingle: "Volledig 360° Dossier (Per Onderneming)",
            descSingle: "Eenmalige betaling • Directe download na betaling • Inclusief 90 dagen monitoring.",
            planPartner: "Accountants- & Advocatenkantoren",
            descPartner: "Batch-analyse (CSV) en White-Label rapportages.",
            scopeBoundary: "Afbakening: Het systeem genereert de sectorspecifieke preventieve documentatie; de rechtsgeldige ondertekening berust bij de directie.",
            proofKicker: "BEWIJSKRACHT & INTEGRITEIT",
            proofTitle: "Cryptografische borging volgens ISO 19011 en eIDAS Art. 41",
            proofSub: "Elk document wordt verzegeld met een SHA-256 hash en canonieke RFC 8785 handtekening.",
            proofCards: [
                { t: "eIDAS Art. 41", d: "Cryptografische tijdstempel en onweerlegbare documentintegriteit." },
                { t: "ISO 19011 Richtlijn", d: "Reproduceerbare auditmatrix over 37 vectoren." },
                { t: "SHA-256 Merkle Tree", d: "Directe detectie van elke wijziging van één enkele bit." },
                { t: "AVG Dataminimalisatie", d: "Versleutelde opslag zonder doorgifte aan derden." }
            ],
            dirKicker: "WETTELIJKE DEKKING PER SECTOR",
            dirTitle: "6 Geauditeerde Rechtsgebieden in Uw Jurisdictie"
        }
    };

    function detectCountryCode() {
        const m = window.location.pathname.match(/\/region\/([a-z]{2})/i);
        return m ? m[1].toLowerCase() : "de";
    }

    function setText(id, val) {
        const el = document.getElementById(id);
        if (el && val !== undefined) el.textContent = val;
    }

    function renderPortal(countryKey, dialectCode) {
        const cfg = EU_CONFIG[countryKey] || EU_CONFIG.de;
        const langPrefix = (dialectCode || cfg.dialects[0].code).split("-")[0].toLowerCase();
        const t = LANG_TEXT[langPrefix] || LANG_TEXT.de;

        document.documentElement.lang = langPrefix;
        try {
            localStorage.setItem("mallen_consented_locale_v3", dialectCode || cfg.dialects[0].code);
            localStorage.setItem("mallen_country", countryKey.toUpperCase());
        } catch (e) {}

        setText("eu-block-pill", cfg.block);
        setText("eu-country-name", cfg.countryName);
        setText("eu-currency-badge", cfg.currency);
        setText("eu-header-sub", t.headerSub);
        setText("eu-nav-calibration", t.navCal);
        setText("eu-nav-deliverables", t.navTrust);
        setText("eu-nav-proof", t.navProof);
        setText("eu-nav-coverage", t.navCov);
        setText("eu-nav-partner", t.navPartner);

        setText("eu-hero-badge", t.heroBadge);
        setText("eu-crypto-law-badge", cfg.cryptoLaw);
        setText("eu-hero-title", t.heroTitle);
        setText("eu-hero-subtitle", t.heroSub);

        setText("eu-tax-label", t.taxLabel);
        setText("eu-tax-microcopy", t.taxMicro);
        setText("eu-domain-label", t.domLabel);
        setText("eu-domain-microcopy", t.domMicro);
        setText("eu-emp-label", t.empLabel);
        setText("eu-emp-microcopy", t.empMicro);
        setText("eu-scan-btn", t.scanBtn);
        setText("eu-demo-pdf-btn", t.demoBtn);
        setText("eu-postpay-defer-note", t.deferNote);

        setText("eu-trust-kicker", t.trustKicker);
        setText("eu-trust-title", t.trustTitle);
        setText("eu-trust-col1-title", t.col1Title);
        setText("eu-trust-col2-title", t.col2Title);
        setText("eu-trust-col3-title", t.col3Title);
        setText("eu-price-plan-single", t.planSingle);
        setText("eu-price-amount-single", cfg.currency);
        setText("eu-price-desc-single", t.descSingle);
        setText("eu-price-plan-partner", t.planPartner);
        setText("eu-price-desc-partner", t.descPartner);
        setText("eu-trust-scope-Boundary", t.scopeBoundary);

        const c1 = document.getElementById("eu-trust-col1-list");
        if (c1) c1.innerHTML = t.col1List.map(function (i) { return "<li>" + i + "</li>"; }).join("");
        const c2 = document.getElementById("eu-trust-col2-list");
        if (c2) c2.innerHTML = t.col2List.map(function (i) { return "<li>" + i + "</li>"; }).join("");

        setText("eu-proof-kicker", t.proofKicker);
        setText("eu-proof-title", t.proofTitle);
        setText("eu-proof-sub", t.proofSub);

        const pGrid = document.getElementById("eu-proof-cards-grid");
        if (pGrid) {
            pGrid.innerHTML = t.proofCards.map(function (c) {
                return '<div class="mallen-surface-card p-4">' +
                    '<div class="text-sm font-bold text-white mb-1">' + c.t + '</div>' +
                    '<p class="text-xs text-slate-300">' + c.d + '</p>' +
                    '</div>';
            }).join("");
        }

        setText("eu-dir-kicker", t.dirKicker);
        setText("eu-section-title", t.dirTitle);
        setText("eu-reg-count-badge", cfg.regCount);

        const lGrid = document.getElementById("eu-laws-cards-container");
        if (lGrid) {
            lGrid.innerHTML = cfg.laws.map(function (lw) {
                return '<div class="mallen-surface-card p-5 flex flex-col justify-between">' +
                    '<div>' +
                    '<div class="flex items-center justify-between gap-2 mb-2">' +
                    '<span class="font-mono-data text-xs font-bold text-emerald-400">' + lw.code + '</span>' +
                    '<span class="font-mono-data text-xs px-2 py-0.5 rounded bg-slate-900 border border-slate-700 text-amber-400">Inspektionsquote: ' + lw.rate + '</span>' +
                    '</div>' +
                    '<h3 class="text-base font-bold text-white mb-1.5">' + lw.title + '</h3>' +
                    '<p class="text-xs text-slate-300">' + lw.desc + '</p>' +
                    '</div>' +
                    '</div>';
            }).join("");
        }

        const rGrid = document.getElementById("eu-registries-grid");
        if (rGrid) {
            rGrid.innerHTML = cfg.registries.map(function (rg) {
                return '<div class="p-3.5 rounded-lg border border-slate-800 text-xs font-medium text-slate-200">' + rg + '</div>';
            }).join("");
        }

        if (typeof window.getMallenThemeLabel === "function") {
            const tBtn = document.getElementById("eu-theme-toggle-btn");
            if (tBtn) tBtn.textContent = window.getMallenThemeLabel();
        }
    }

    window.runEuRegionalSimulation = function () {
        const countryKey = detectCountryCode();
        const cfg = EU_CONFIG[countryKey] || EU_CONFIG.de;
        const taxVal = (document.getElementById("eu-tax-input")?.value || "").trim() || (countryKey.toUpperCase() + "-8849201");
        const empVal = Math.max(1, parseInt(document.getElementById("eu-emp-input")?.value || "45", 10));
        const box = document.getElementById("eu-simulation-result-box");
        if (!box) return;

        const exposure = (18500 + empVal * 1420).toLocaleString("de-DE", { minimumFractionDigits: 2, maximumFractionDigits: 2 });
        box.classList.remove("hidden");
        box.innerHTML =
            '<div class="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-slate-800">' +
                '<div>' +
                    '<span class="text-xs font-mono-data text-emerald-400 block">SHA-256 SSOT SEAL • ' + cfg.countryName.toUpperCase() + '</span>' +
                    '<h3 class="text-lg font-extrabold text-white mt-0.5">' + taxVal + ' (' + empVal + ' Mitarbeitende / Collaborateurs)</h3>' +
                '</div>' +
                '<div class="text-right">' +
                    '<span class="text-xs text-slate-400 block">Geschätztes Bußgeld-Exponat (37 Vektoren):</span>' +
                    '<span class="text-xl font-extrabold font-mono-data text-amber-400">' + exposure + ' (' + cfg.currency.split(" ")[1] + ')</span>' +
                '</div>' +
            '</div>' +
            '<div class="flex flex-wrap items-center justify-between gap-4 pt-2">' +
                '<div class="text-xs text-slate-300">7 Rechtsinstrumente (.PDF &amp; Word-.HTML) + 10 SHA-256 Zertifikate bereit zur Kompilierung (' + cfg.currency + ').</div>' +
                '<div class="flex flex-wrap gap-3">' +
                    '<a href="/api/micro_scan/' + encodeURIComponent(taxVal) + '" target="_blank" class="mallen-btn-secondary px-4 py-2 text-xs">Vorab-Gutachten (PDF)</a>' +
                    '<a href="/checkout?ref=' + encodeURIComponent(taxVal) + '" class="mallen-btn-primary px-5 py-2 text-xs">360°-Paket aktivieren (' + cfg.currency + ')</a>' +
                '</div>' +
            '</div>';
    };

    function initEuPortal() {
        const countryKey = detectCountryCode();
        const cfg = EU_CONFIG[countryKey] || EU_CONFIG.de;
        const sel = document.getElementById("eu-dialect-select");
        if (sel) {
            sel.innerHTML = cfg.dialects.map(function (d) {
                return '<option value="' + d.code + '">' + d.label + '</option>';
            }).join("");
            sel.addEventListener("change", function () {
                renderPortal(countryKey, sel.value);
            });
        }
        renderPortal(countryKey, cfg.dialects[0].code);
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", initEuPortal);
    } else {
        initEuPortal();
    }
})();
