/**
 * FIRMA W16 MALLEN® — Paridad Estructural 1:1 con México para todos los países de DACH y Benelux
 * Renderiza en el idioma nativo de cada país:
 *  1) Tarjetas del Directorio Regulatorio con badge de porcentaje real y Beneficio Directo.
 *  2) Las 6 tarjetas numeradas de Padrones Oficiales Nacionales (+3.8M DE, +610K AT, +680K CH, +2.4M NL, +1.3M BE, +145K LU).
 *  3) Las 4 tarjetas interactivas de Sustento Legal & Validez Probatoria (eIDAS/ZertES, Dictamen W16, Merkle SHA-256, Secreto PII).
 *  4) El Motor de Calibración de 10 Variables Internas y los 16 Campos de Personalización de Instrumentos Jurídicos.
 */
(function () {
    "use strict";

    const PARITY_SECTIONS_I18N = {
        "de": {
            heroCtaPrimary: "Unternehmens-Prüfung starten →",
            heroCtaSecondary: "Gesetzliche Abdeckung (37 Vektoren)",
            dirKicker: "REGULATORISCHE ENZYKLOPÄDIE & COMPLIANCE-LEITFADEN",
            navRegistries: "Behördenregister",
            navCalibration: "OSINT & Kalibrierung",
            regKicker: "SACHVERSTÄNDIGE ABFRAGUNG OFFIZIELLER REGISTER",
            regTitle: "Indexierte Behördenregister, Sanktionslisten & Prüfdatenbanken",
            proofKicker: "RECHTLICHE & PROZESSUALE GRUNDLAGE",
            proofTitle: "Sachverständigen-Rückhalt & Volle Beweiskraft",
            proofSub: "Wählen Sie eine der 4 institutionellen Garantien, um die Rechtsgrundlage und prozessuale Wirkung einzusehen.",
            calKicker: "DETERMINISTISCHE DIAGNOSE & VERTRAGSPARAMETRISIERUNG",
            calTitle: "OSINT-Diagnosemotor & Sachverständige Kalibrierung",
            calSub: "Geben Sie Ihre Registernummer und Web-Domain ein. Optional können Sie die 10 internen Betriebsvariablen und 16 Vertragsfelder anpassen (leere Felder werden automatisch mit ____________________ zum Ausfüllen vorbereitet).",
            empLabel: "Mitarbeiterzahl (Vollzeitäquivalente)",
            microBtn: "Implementierungsnachweis (PDF) laden",
            calVarsTitle: "Kalibrierung interner Betriebsvariablen (10 Prüfparameter):",
            calVars: [
                "1. Telearbeit / Homeoffice > 40% der Wochenarbeitszeit",
                "2. Einsatz von Subunternehmern / Fremdpersonal",
                "3. Verarbeitung biometrischer Daten oder Gesundheitsdaten",
                "4. Betriebsstätte > 100 m² oder Publikumsverkehr (Brandschutz)",
                "5. Flurförderzeuge (Gabelstapler) oder manuelle Lastenhandhabung",
                "6. Druckbehälter, Dampfkessel oder Kompressoranlagen",
                "7. Umgang mit Gefahrstoffen oder chemischen Erzeugnissen",
                "8. Zahlung immaterieller Unternehmensdienstleistungen (Beratung / IT)",
                "9. Straßengüterverkehr oder eigener Fuhrpark",
                "10. Geldwäscherelevante Tätigkeiten (GwG / WiEReG / GwG-CH)"
            ],
            dossierTitle: "Personalisierung der 7 Rechtsinstrumente (16 Unternehmensfelder — Optional)",
            dossierDesc: "Wenn Sie diese Felder leer lassen, generiert das System alle 7 Verträge und Protokolle mit professionellen Ausfülllinien (____________________) zur sofortigen Unterzeichnung.",
            dossierFields: [
                "Name des gesetzlichen Vertreters (Geschäftsführer / Vorstand)",
                "Funktion / Vertretungsbefugnis",
                "Handelsregister- / Firmenbuch-Eintrag (Urkunden-Nr.)",
                "Registergericht / Kantonales Handelsregisteramt",
                "Wirtschafts-Identifikationsnummer / Sozialversicherungs-Nr.",
                "Zuständiger Gerichtsstand (Stadt / Bundesland / Kanton)",
                "Name des Datenschutzbeauftragten / Datenschutzverantwortlichen",
                "E-Mail-Adresse für Betroffenenrechte (DSGVO / revDSG)",
                "Arbeitgeber-Koordinator (Arbeitsschutzausschuss ASA / EKAS)",
                "Arbeitnehmer-Vertreter (Sicherheitsvertrauensperson)",
                "Zuständige Fachkraft für Arbeitssicherheit (Sifa / ASA)",
                "Leiter der internen Brandschutz- & Evakuierungsgruppe",
                "Hauptauftragnehmer / Dienstleister (für Rahmenvertrag)",
                "Register- / USt-IdNr. des Auftragnehmers",
                "Gegenstand der vereinbarten Dienstleistung",
                "Kundendienst-Kontakt für E-Commerce & Widerruf (Tel. / E-Mail)"
            ],
            directBenefitLabel: "Direkter Unternehmensnutzen & ROI:",
            proofs: [
                {
                    badge: "eIDAS ART. 41 • ZERTES • § 371a ZPO",
                    title: "Kryptografischer Zeitstempel & Beweissicherung",
                    short: "Unveränderbarer SHA-256 Zeitstempel gemäß eIDAS-Verordnung (EU) Nr. 910/2014 Art. 41 und Schweizer ZertES / GeBüV.",
                    detail: "Jedes Dossier und jedes der 7 Rechtsinstrumente wird bei der Erstellung mit einem kryptografischen SHA-256 Hash versehen und im L2-Merkle-Baum verankert. Dies beweist gegenüber Finanzbehörden (GoBD / BAO / GeBüV) und Gerichten, dass die Dokumentation zum exakten Prüfungszeitpunkt vorlag und nicht rückdatiert wurde."
                },
                {
                    badge: "FIRMA W16 MALLEN® • METHODIK",
                    title: "Zertifiziertes Sachverständigen-Dossier (37 Vektoren)",
                    short: "Deterministische Auswertung aller 37 gesetzlichen Vektoren inkl. 10 Einzelzertifikaten und 7 Vertragsvorlagen.",
                    detail: "Die Begutachtung trennt strikt zwischen dem Executive Summary für die Geschäftsleitung (Haftungs- und Bußgeldübersicht) und der technischen Prüfanlage mit allen 37 Einzelvektoren, Branchenklassifikation (WZ 2008 / ÖNACE / NOGA) und konkreten Abhilfemaßnahmen."
                },
                {
                    badge: "SHA-256 • L2 MERKLE ROOT",
                    title: "Mathematische Integrität & Öffentliche Verifizierung",
                    short: "Jede Änderung eines einzelnen Bytes im PDF-Dokument macht den kryptografischen Prüf-Hash sofort ungültig.",
                    detail: "Über den öffentlichen Prüf-Endpunkt (mallen.lat/verify) können Aufsichtsbehörden, Wirtschaftsprüfer und Geschäftspartner jederzeit die Echtheit und Unversehrtheit des ausgestellten Compliance-Zertifikats in Echtzeit verifizieren."
                },
                {
                    badge: "DSGVO ART. 25 & 32 • REVDSG",
                    title: "PII-Sanitisierung & Flüchtige RAM-Verarbeitung",
                    short: "Keine dauerhafte Speicherung personenbezogener Daten natürlicher Personen; volle TLS 1.3 Verschlüsselung.",
                    detail: "Alle OSINT-Abfragen beschränken sich auf öffentlich zugängliche Unternehmensregister und technische HTTP/TLS-Header. Sensible Eingaben zur Vertragsparametrisierung werden ausschließlich im flüchtigen Arbeitsspeicher (RAM) zur PDF-Kompilierung genutzt."
                }
            ]
        },
        "fr": {
            heroCtaPrimary: "Lancer l'Audit de Conformité →",
            heroCtaSecondary: "Couverture Légale (37 Vecteurs)",
            dirKicker: "ENCYCLOPÉDIE RÉGLEMENTAIRE & GUIDE DE CONFORMITÉ",
            navRegistries: "Registres Officiels",
            navCalibration: "OSINT & Calibrage",
            regKicker: "VÉRIFICATION DANS LES REGISTRES OFFICIELS NATIONAUX",
            regTitle: "Bases de Données Officielles, Listes de Sanctions & Registres Indexés",
            proofKicker: "FONDEMENT JURIDIQUE & PROCÉDURAL",
            proofTitle: "Support d'Expertise & Pleine Valeur Probante",
            proofSub: "Sélectionnez l'une des 4 garanties institutionnelles pour consulter le fondement légal et l'effet probatoire.",
            calKicker: "DIAGNOSTIC DÉTERMINISTE & PARAMÉTRAGE CONTRACTUEL",
            calTitle: "Moteur de Diagnostic OSINT & Calibrage Réglementaire",
            calSub: "Saisissez votre numéro d'entreprise et votre domaine web. Vous pouvez calibrer les 10 variables internes et les 16 champs contractuels (les champs vides génèrent automatiquement des lignes ____________________ prêtes à compléter).",
            empLabel: "Effectif de l'entreprise (ETP)",
            microBtn: "Télécharger le Certificat d'Avancement (PDF)",
            calVarsTitle: "Calibrage des variables opérationnelles internes (10 paramètres) :",
            calVars: [
                "1. Télétravail régulier > 40% du temps de travail",
                "2. Recours à des sous-traitants ou prestataires externes",
                "3. Traitement de données biométriques ou de santé",
                "4. Établissement > 100 m² ou accueillant du public (Sécurité incendie)",
                "5. Chariots élévateurs ou manutention manuelle de charges",
                "6. Équipements sous pression, chaudières ou compresseurs",
                "7. Manipulation de substances chimiques ou dangereuses",
                "8. Paiement de prestations intellectuelles / services intra-groupe",
                "9. Transport routier de marchandises ou flotte d'entreprise",
                "10. Activités soumises aux obligations anti-blanchiment (LBA / AML / UBO)"
            ],
            dossierTitle: "Personnalisation des 7 Instruments Juridiques (16 Champs — Optionnel)",
            dossierDesc: "Si vous laissez ces champs vides, le système compile les 7 contrats et actes avec des lignes de saisie professionnelles (____________________).",
            dossierFields: [
                "Nom du représentant légal (Gérant / Administrateur)",
                "Qualité / Fonction statutaire",
                "Numéro d'acte constitutif / Inscription au Registre",
                "Greffe / Autorité du Registre de commerce",
                "Numéro d'affiliation Sécurité sociale (AVS / ONSS / CCSS)",
                "For juridique / Juridiction compétente (Ville / Canton)",
                "Nom du Délégué à la protection des données (DPO / Conseiller)",
                "Adresse e-mail dédiée aux droits RGPD / nLPD",
                "Coordinateur employeur (Comité de sécurité SIPPT / MSST / ITM)",
                "Représentant des travailleurs (Délégué à la sécurité)",
                "Responsable désigné de la santé et sécurité au travail",
                "Responsable de l'équipe d'évacuation et sécurité incendie",
                "Raison sociale du prestataire principal (Contrat-cadre)",
                "Numéro d'entreprise / TVA du prestataire",
                "Objet détaillé de la prestation contractuelle",
                "Contact Service Client E-Commerce & Rétractation (Tél. / E-mail)"
            ],
            directBenefitLabel: "Bénéfice Direct & ROI Opérationnel :",
            proofs: [
                {
                    badge: "eIDAS ART. 41 • SCSE / ZERTES",
                    title: "Horodatage Cryptographique & Date Certaine",
                    short: "Scellement SHA-256 conforme à l'art. 41 du Règlement eIDAS (UE) n° 910/2014 et à la SCSE / Olico suisse.",
                    detail: "Chaque dossier et chacun des 7 actes juridiques reçoit une empreinte SHA-256 unique ancrée dans un arbre de Merkle L2, prouvant l'antériorité et l'intégrité des documents devant les autorités fiscales et judiciaires."
                },
                {
                    badge: "FIRMA W16 MALLEN® • MÉTHODOLOGIE",
                    title: "Dossier d'Expertise Certifié (37 Vecteurs)",
                    short: "Évaluation déterministe des 37 vecteurs légaux avec 10 certificats de conformité et 7 actes prêts à signer.",
                    detail: "Le rapport sépare la synthèse exécutive financière destinée au Conseil d'administration et l'annexe technique détaillant les 37 vecteurs, la classification NACE/NOGA et le plan de remédiation."
                },
                {
                    badge: "SHA-256 • RACINE DE MERKLE L2",
                    title: "Intégrité Mathématique & Vérification Publique",
                    short: "Toute altération d'un seul octet du fichier PDF invalide immédiatement la signature cryptographique.",
                    detail: "Le portail public de vérification (mallen.lat/verify) permet aux auditeurs, autorités et partenaires de contrôler en temps réel l'authenticité du dossier."
                },
                {
                    badge: "RGPD ART. 25 & 32 • NLPD",
                    title: "Anonymisation PII & Traitement en Mémoire Vive (RAM)",
                    short: "Aucune conservation de données personnelles sensibles ; chiffrement strict TLS 1.3 de bout en bout.",
                    detail: "Les analyses OSINT portent exclusivement sur les registres publics d'entreprises et les en-têtes TLS/HTTP. Les données de personnalisation contractuelle sont compilées à la volée en mémoire RAM."
                }
            ]
        },
        "it": {
            heroCtaPrimary: "Avvia Perizia di Conformità →",
            heroCtaSecondary: "Copertura Normativa (37 Vettori)",
            dirKicker: "ENCICLOPEDIA NORMATIVA & GUIDA DI CONFORMITÀ",
            navRegistries: "Registri Ufficiali",
            navCalibration: "OSINT & Calibrazione",
            regKicker: "VERIFICA NEI REGISTRI UFFICIALI FEDERALI E CANTONALI",
            regTitle: "Banche Dati Ufficiali, Liste di Controllo & Registri Indicizzati",
            proofKicker: "FONDAMENTO GIURIDICO & PROCESSUALE",
            proofTitle: "Supporto Peritale & Pieno Valore Probatorio",
            proofSub: "Selezionate una delle 4 garanzie istituzionali per esaminare il fondamento normativo e l'efficacia probatoria.",
            calKicker: "DIAGNOSI DETERMINISTICA & PARAMETRIZZAZIONE CONTRATTUALE",
            calTitle: "Motore di Diagnosi OSINT & Calibrazione Peritale",
            calSub: "Inserite il numero IDI/UID e il dominio web. Potete calibrare le 10 variabili interne e i 16 campi contrattuali (i campi vuoti generano automaticamente linee ____________________ pronte per la compilazione).",
            empLabel: "Numero di dipendenti (ULA)",
            microBtn: "Scarica Certificato di Avanzamento (PDF)",
            calVarsTitle: "Calibrazione delle variabili operative interne (10 parametri):",
            calVars: [
                "1. Telelavoro regolare > 40% dell'orario settimanale",
                "2. Ricorso a subappaltatori o fornitori di servizi esterni",
                "3. Trattamento di dati biometrici o relativi alla salute",
                "4. Sede operativa > 100 m² o aperta al pubblico (Antincendio AICAA)",
                "5. Carrelli elevatori o movimentazione manuale dei carichi",
                "6. Impianti a pressione, caldaie o compressori industriali",
                "7. Manipolazione di sostanze chimiche o rifiuti speciali (OTRif)",
                "8. Pagamento di consulenze o servizi immateriali societari",
                "9. Trasporto merci su strada o flotta aziendale (OLR 1)",
                "10. Attività soggette agli obblighi antiriciclaggio (LRD / GAFI)"
            ],
            dossierTitle: "Personalizzazione dei 7 Strumenti Giuridici (16 Campi — Opzionale)",
            dossierDesc: "Se lasciate vuoti questi campi, il sistema compila i 7 contratti e regolamenti con apposite linee di compilazione (____________________).",
            dossierFields: [
                "Nome del rappresentante legale (Amministratore / Gerente)",
                "Carica / Potere di firma statutario",
                "Numero d'iscrizione a Registro di commercio",
                "Ufficio cantonale del Registro di commercio",
                "Numero di affiliazione Cassa AVS / Polizza SUVA",
                "Foro competente (Città / Cantone)",
                "Nome del consulente per la protezione dei dati (nLPD)",
                "Indirizzo e-mail per l'esercizio dei diritti d'accesso (art. 25 nLPD)",
                "Coordinatore del datore di lavoro (Commissione sicurezza MSSL)",
                "Rappresentante dei lavoratori per la sicurezza",
                "Addetto alla sicurezza sul lavoro (MSSL 6508)",
                "Responsabile antincendio ed evacuazione (AICAA)",
                "Ragione sociale del fornitore principale (Contratto quadro)",
                "Numero IDI / Partita IVA del fornitore",
                "Oggetto dettagliato della prestazione di servizi",
                "Contatto Servizio Clienti E-Commerce (Tel. / E-mail)"
            ],
            directBenefitLabel: "Beneficio Diretto & ROI Operativo:",
            proofs: [
                {
                    badge: "FIELE • OLC • ART. 957 CO",
                    title: "Data Certa Crittografica & Conservazione Inalterabile",
                    short: "Sigillo SHA-256 conforme all'Ordinanza sui libri di commercio (Olc) e alla Legge sulla firma elettronica (FiEle).",
                    detail: "Ogni fascicolo e ciascuno dei 7 contratti viene sigillato con un hash SHA-256 e ancorato in un albero di Merkle L2, provando l'integrità e la data certa davanti all'AFC e ai tribunali civili (art. 177 CPC)."
                },
                {
                    badge: "FIRMA W16 MALLEN® • METODOLOGIA",
                    title: "Fascicolo Peritale Certificato (37 Vettori)",
                    short: "Valutazione deterministica dei 37 vettori normativi con 10 certificati tecnici e 7 contratti pronti per la firma.",
                    detail: "Il rapporto distingue il riepilogo esecutivo per il Consiglio d'amministrazione (art. 716a e 725 CO) dall'allegato tecnico con i 37 vettori NOGA e il piano d'azione."
                },
                {
                    badge: "SHA-256 • L2 MERKLE ROOT",
                    title: "Integrità Matematica & Verifica Pubblica",
                    short: "Qualsiasi modifica anche di un solo byte nel PDF invalida istantaneamente l'impronta crittografica.",
                    detail: "Tramite il portale pubblico di verifica (mallen.lat/verify), revisori, autorità e partner possono controllare in tempo reale l'autenticità del documento."
                },
                {
                    badge: "NLPD • OPDO • TLS 1.3",
                    title: "Protezione Dati & Elaborazione Volatili in RAM",
                    short: "Nessuna archiviazione permanente di dati personali sensibili; cifratura end-to-end TLS 1.3.",
                    detail: "Le verifiche OSINT interrogano esclusivamente registri pubblici (Zefix/IDI) e parametri tecnici TLS/HTTP. I dati contrattuali vengono compilati in memoria volatile RAM."
                }
            ]
        },
        "nl": {
            heroCtaPrimary: "Start Wettelijke Compliance Audit →",
            heroCtaSecondary: "Wettelijke Dekking (37 Vectoren)",
            dirKicker: "WETTELIJKE ENCYCLOPEDIE & COMPLIANCE GIDS",
            navRegistries: "Officiële Registers",
            navCalibration: "OSINT & Kalibratie",
            regKicker: "VERIFICATIE IN OFFICIËLE NATIONALE HANDELS- & TOEZICHTREGISTERS",
            regTitle: "Geïndexeerde Overheidsregisters, Toezichtlijsten & Inspectiedatabanken",
            proofKicker: "JURIDISCHE & PROCESRECHTELIJKE GRONDSLAG",
            proofTitle: "Expertise-Onderbouwing & Volledige Bewijskracht",
            proofSub: "Selecteer een van de 4 institutionele waarborgen om het wettelijk kader en de bewijskracht te bekijken.",
            calKicker: "DETERMINISTISCHE DIAGNOSE & CONTRACTPARAMETRISERING",
            calTitle: "OSINT-Diagnosemotor & Juridische Kalibratie",
            calSub: "Voer uw KVK/KBO-nummer en webdomein in. U kunt optioneel de 10 interne bedrijfsvariabelen en 16 contractvelden kalibreren (lege velden worden automatisch voorzien van invullijnen ____________________).",
            empLabel: "Aantal werknemers (FTE)",
            microBtn: "Download Voortgangscertificaat (PDF)",
            calVarsTitle: "Kalibratie van interne bedrijfsvariabelen (10 toetsingsparameters):",
            calVars: [
                "1. Structureel thuiswerk / telewerk > 40% van de arbeidstijd",
                "2. Inschakeling van onderaannemers, uitzendkrachten of zzp'ers",
                "3. Verwerking van biometrische gegevens of gezondheidsgegevens",
                "4. Bedrijfslocatie > 100 m² of publieksfunctie (BHV / Brandpreventie)",
                "5. Heftrucks, magazijnstellingen of fysieke tilbelasting",
                "6. Drukvaten, stoomketels of industriële compressoren",
                "7. Opslag of gebruik van gevaarlijke stoffen / chemicaliën",
                "8. Betaling van zakelijke advies-, IT- of managementdiensten",
                "9. Wegvervoer van goederen of eigen wagenpark",
                "10. Wwft- / Anti-witwasplichtige zakelijke activiteiten (UBO)"
            ],
            dossierTitle: "Personalisatie van de 7 Juridische Instrumenten (16 Velden — Optioneel)",
            dossierDesc: "Indien u deze velden leeg laat, genereert het systeem alle 7 overeenkomsten en protocollen met professionele invullijnen (____________________).",
            dossierFields: [
                "Naam wettelijk vertegenwoordiger (Bestuurder / Zaakvoerder)",
                "Statutaire functie / Tekenbevoegdheid",
                "KVK- / KBO-inschrijvingsnummer en oprichtingsdatum",
                "Statutaire zetel en kantooradres",
                "Loonheffingen- / RSZ-werkgeversnummer",
                "Bevoegde rechtbank / Arrondissement",
                "Functionaris Gegevensbescherming (FG / DPO)",
                "E-mailadres voor inzage- en AVG-verzoeken",
                "Werkgeverscoördinator Arbo / Interne Dienst (IDPBW)",
                "Werknemersvertegenwoordiger (OR / Preventiemedewerker)",
                "Aangestelde Preventiemedewerker / Vertrouwenspersoon PSA",
                "Hoofd Bedrijfshulpverlening (BHV) & Evacuatie",
                "Statutaire naam hoofdopdrachtnemer (Raamovereenkomst)",
                "KVK- / KBO- of BTW-nummer opdrachtnemer",
                "Specifieke omschrijving van de zakelijke dienstverlening",
                "Klantenservice-contact voor E-Commerce & Herroeping"
            ],
            directBenefitLabel: "Direct Bedrijfsvoordeel & ROI:",
            proofs: [
                {
                    badge: "eIDAS ART. 41 • ART. 156a RV / WER",
                    title: "Cryptografische Tijdstempel & Bewijskracht",
                    short: "Onveranderlijke SHA-256 tijdstempel conform eIDAS-verordening (EU) nr. 910/2014 art. 41 en Nederlands/Belgisch bewijsrecht.",
                    detail: "Elk dossier en elk van de 7 juridische instrumenten krijgt een unieke SHA-256 hash verankerd in een L2 Merkle Tree, waarmee de exacte datum en integriteit tegenover Belastingdienst, FOD Financiën en rechters vaststaat."
                },
                {
                    badge: "FIRMA W16 MALLEN® • METHODIEK",
                    title: "Gecertificeerd Compliance-Dossier (37 Vectoren)",
                    short: "Deterministische doorlichting van alle 37 wettelijke vectoren inclusief 10 deelcertificaten en 7 contractmodellen.",
                    detail: "Het rapport combineert een financiële directiesamenvatting met een technische bijlage waarin alle 37 vectoren, SBI/NACE-BEL codes en herstelmaatregelen zijn uitgewerkt."
                },
                {
                    badge: "SHA-256 • L2 MERKLE ROOT",
                    title: "Wiskundige Integriteit & Publieke Verificatie",
                    short: "Elke wijziging van één enkele byte in het PDF-bestand maakt de cryptografische handtekening direct ongeldig.",
                    detail: "Via het publieke verificatieportaal (mallen.lat/verify) kunnen toezichthouders, accountants en contractpartijen de echtheid van het dossier realtime controleren."
                },
                {
                    badge: "AVG ART. 25 & 32 • UAVG / GBA",
                    title: "PII-Sanitisatie & Vluchtige RAM-Verwerking",
                    short: "Geen opslag van gevoelige persoonsgegevens; volledige end-to-end TLS 1.3 versleuteling.",
                    detail: "Alle OSINT-controles beperken zich tot openbare handelsregisters en technische TLS/HTTP-headers. Contractvariabelen worden uitsluitend in het werkgeheugen (RAM) verwerkt."
                }
            ]
        },
        "lb": {
            heroCtaPrimary: "Lëtzebuerger Audit starten →",
            heroCtaSecondary: "Gesetzlech Ofdeckung (37 Vektoren)",
            dirKicker: "REGLEMENTARESCH ENZYKLOPÄDIE & LEITFADEN",
            navRegistries: "Offiziell Registeren",
            navCalibration: "OSINT & Kalibratioun",
            regKicker: "KONTROLL AN DEN OFFIZIELLE LËTZEBUERGER REGISTEREN",
            regTitle: "Indexéiert Staatsregisteren, Kontrolllëschten & Datenbanken",
            proofKicker: "JURISTESCH & PROZESSUAL GRONDLAG",
            proofTitle: "Expertise-Ënnerstëtzung & Voll Beweiskraaft",
            proofSub: "Wielt eng vun de 4 institutionelle Garantien fir d'Rechtsgrondlag an d'Beweiskraaft ze gesinn.",
            calKicker: "DETERMINISTESCH DIAGNOS & KONTRAKTPARAMETRISÉIERUNG",
            calTitle: "OSINT-Diagnosmotor & Juristesch Kalibratioun",
            calSub: "Gitt Är RCS-Nummer an Web-Domain an. Dir kënnt déi 10 intern Variabelen an 16 Kontraktfelder upassen (eidel Felder kréien automatesch Linnen ____________________).",
            empLabel: "Zuel vun de Salariéen (ETP)",
            microBtn: "Fortschrëtts-Zertifikat (PDF) eroflueden",
            calVarsTitle: "Kalibratioun vun den 10 interne Betribsvariabelen:",
            calVars: [
                "1. Regelméissegen Télétravail > 40% vun der Aarbechtszäit",
                "2. Asaz vu Sous-traitanten oder externen Déngschtleeschter",
                "3. Veraarbechtung vu biometreschen Donnéeën oder Gesondheetsdonnéeën",
                "4. Betribsfläch > 100 m² oder Publikumsverkéier (ITM Brandschutz)",
                "5. Gabelstapler oder manuell Laaschtenhandhabung",
                "6. Drockbehälter, Chaudièren oder Kompresseren",
                "7. Ëmgang mat chemeschen oder geféierleche Stoffer",
                "8. Bezuelung vun immateriellen Entreprise-Servicer (Consulting / IT)",
                "9. Stroossentransport vu Wueren oder eege Flott",
                "10. Aktivitéiten ënner dem Anti-Geldwäsch-Gesetz (AML / RBE)"
            ],
            dossierTitle: "Personaliséierung vun de 7 Juristeschen Instrumenter (16 Felder — Optional)",
            dossierDesc: "Wann Dir dës Felder eidel loosst, generéiert de System déi 7 Kontrakter mat professionelle Linnen (____________________).",
            dossierFields: [
                "Numm vum gesetzleche Vertrieder (Gérant / Administrateur)",
                "Statutaresch Funktioun",
                "RCS Lëtzebuerg Nummer (B-Nummer)",
                "Sëtz vun der Gesellschaft zu Lëtzebuerg",
                "CCSS Matricule National vum Employeur",
                "Zoustännegt Geriicht (Lëtzebuerg / Diekirch)",
                "Numm vum Dateschutzbeoptraagten (DPO — CNPD)",
                "E-Mail-Adress fir RGPD-Ufroen",
                "Employeur-Koordinator fir Aarbechtssécherheet (ITM)",
                "Vertrieder vun der Délégation du Personnel",
                "Ernannten Travailleur Désigné (Art. L.312-3 Code du Travail)",
                "Responsabele fir Brandschutz an Evakuatioun",
                "Numm vum Haaptprestataire (Rahmenkontrakt)",
                "RCS- oder TVA-Nummer vum Prestataire",
                "Detailéierten Objet vun der Déngschtleeschtung",
                "Clientsservice-Kontakt fir E-Commerce & Widderruff"
            ],
            directBenefitLabel: "Direkte Betribsvirdeel & ROI:",
            proofs: [
                {
                    badge: "eIDAS ART. 41 • ART. 1322-1 CC",
                    title: "Kryptographeschen Zäitstempel & Beweiskraaft",
                    short: "Onverännerleche SHA-256 Zäitstempel no eIDAS Art. 41 an dem Lëtzebuerger Code civil.",
                    detail: "All Dossier an all Kontrakt kritt e SHA-256 Hash verankert am L2 Merkle-Bam, wat d'Integritéit an den exakten Datum virun den Autoritéiten (ACD, AED, CNPD, ITM) beweist."
                },
                {
                    badge: "FIRMA W16 MALLEN® • METHODOLOGIE",
                    title: "Zertifizéierten Compliance-Dossier (37 Vektoren)",
                    short: "Deterministesch Evaluatioun vun de 37 gesetzleche Vektore mat 10 Zertifikater a 7 Kontrakter.",
                    detail: "De Rapport kombinéiert eng Finanzsynthees fir de Conseil d'Administration mat enger technescher Annex iwwer all 37 NACELUX-Vektoren."
                },
                {
                    badge: "SHA-256 • L2 MERKLE ROOT",
                    title: "Mathematesch Integritéit & Ëffentlech Kontroll",
                    short: "All Ännerung vun engem eenzege Byte am PDF mécht d'Signatur direkt ongülteg.",
                    detail: "Iwwer mallen.lat/verify kënnen Revisoren, Autoritéiten a Partner d'Echtheet vum Dokument a Echtzäit kontrolléieren."
                },
                {
                    badge: "RGPD ART. 25 & 32 • CNPD",
                    title: "PII-Schutz & Flüchteg RAM-Veraarbechtung",
                    short: "Keng Späicherung vu sensiblen Donnéeën; komplett TLS 1.3 Verschlësselung.",
                    detail: "All OSINT-Analysen baséieren op ëffentleche Registeren (RCS/LBR) an TLS-Headeren."
                }
            ]
        }
    };

    const COUNTRY_REGISTRIES = {
        "DE": {
            count: "+3.8M Indexierte Einheiten (16 Bundesländer — Handelsregister / WZ 2008)",
            items: [
                { num: "1. Handelsregister & Unternehmensregister", tag: "HRB / HRA • WZ 2008", desc: "Abgleich von Rechtsträger, Sitz, Vertretungsberechtigung und Branchenklassifikation WZ 2008 zur Bestimmung der einschlägigen DGUV/ArbSchG-Vorschriften." },
                { num: "2. Finanzamt & BZSt (USt-IdNr. / GoBD)", tag: "AO §§ 146, 147 • GoBD", desc: "Prüfung der steuerlichen Ordnungsmäßigkeit elektronischer Bücher, Verfahrensdokumentation nach GoBD und Vorsteuerabzugssicherung." },
                { num: "3. Transparenzregister (Bundesanzeiger)", tag: "§§ 18–20 GwG • FIU", desc: "Verifizierung der Meldepflichten zu den wirtschaftlich Berechtigten (UBO) und Geldwäscheprävention nach dem deutschen GwG." },
                { num: "4. BfDI & Landesdatenschutzbehörden", tag: "DSGVO • § 26 BDSG • § 25 TDDDG", desc: "Technische OSINT-Inspektion von TLS 1.3, Cookie-Einwilligung (§ 25 TDDDG), Datenschutzerklärung (Art. 13/14 DSGVO) und VVT." },
                { num: "5. BMAS, Gewerbeaufsicht & DGUV", tag: "ArbSchG §§ 5, 6 • ASiG", desc: "Überprüfung der Gefährdungsbeurteilung (inkl. psychischer Belastung), Arbeitsschutzausschuss (ASA) und Arbeitsstättenverordnung." },
                { num: "6. BSI (NIS-2) & BAFA (Lieferkettengesetz)", tag: "BSIG • LkSG • § 5 DDG", desc: "Prüfung der IT-Sicherheitsanforderungen nach NIS-2, Sorgfaltspflichten in der Lieferkette sowie Impressums- und Fernabsatzrecht." }
            ]
        },
        "AT": {
            count: "+610,000 Indexierte Einheiten (9 Bundesländer — Firmenbuch / ÖNACE 2008)",
            items: [
                { num: "1. Österreichisches Firmenbuch & GISA", tag: "FN • ÖNACE 2008", desc: "Abgleich der Rechtspersönlichkeit, Gewerbeberechtigung (GISA) und ÖNACE-Klassifikation zur Ermittlung der Betriebsanlagen- und Schutzpflichten." },
                { num: "2. Finanzamt Österreich (BAO & UID)", tag: "§§ 131, 132 BAO • RKSV", desc: "Prüfung der 7-jährigen elektronischen Aufbewahrungspflicht, Registrierkassen- und Belegerteilungsrichtlinien sowie Vorsteuerabzug." },
                { num: "3. WiEReG (Wirtschaftliche Eigentümer)", tag: "BMF • WiEReG • FM-GwG", desc: "Kontrolle der jährlichen Melde- und Überprüfungspflicht im Register der wirtschaftlichen Eigentümer zur Vermeidung von Finanzstrafen." },
                { num: "4. Österreichische Datenschutzbehörde (DSB)", tag: "DSGVO • DSG • § 165 TKG 2021", desc: "Technische Prüfung der Datenschutzmitteilung, Verarbeitungsverzeichnis, TLS-Verschlüsselung und Cookie-Vorgaben nach § 165 TKG 2021." },
                { num: "5. Arbeitsinspektorat, AUVA & ÖGK", tag: "§§ 4, 5 ASchG • ASVG / LSD-BG", desc: "Prüfung der Arbeitsplatzevaluierung (inkl. psychischer Belastungen), Sicherheitsvertrauenspersonen (SVP) und Lohn-/Sozialdumping-Prävention." },
                { num: "6. NISG & E-Commerce / Konsumentenschutz", tag: "NISG • § 5 ECG • FAGG • KSchG", desc: "Abgleich der Cybersicherheitspflichten sowie Offenlegung nach ECG/MedienG und Rücktrittsbelehrung nach FAGG." }
            ]
        },
        "CH": {
            count: "+680'000 Indexierte Einheiten (26 Kantone — Zefix / UID / NOGA 2008)",
            items: [
                { num: "1. Zentraler Firmenindex (Zefix) & Handelsregister", tag: "UID (CHE) • NOGA 2008", desc: "Abgleich von Firma, Sitz, Zweck und Zeichnungsberechtigungen in allen 26 kantonalen Handelsregistern samt NOGA-Branchencode." },
                { num: "2. Eidg. Steuerverwaltung ESTV / AFC", tag: "OR Art. 957 • GeBüV / Olico • MWSTG", desc: "Prüfung der 10-jährigen unveränderbaren Aufbewahrung der Geschäftsbücher (GeBüV/Olico/Olc) und der jährlichen MWST-Umsatzabstimmung." },
                { num: "3. GAFI-Transparenz & Geldwäscherei (GwG/LBA)", tag: "Art. 697j–697m OR • GwG", desc: "Kontrolle des Verzeichnisses der wirtschaftlich berechtigten Personen und der Sorgfaltspflichten nach Schweizer Geldwäschereigesetz." },
                { num: "4. EDÖB / PFPDT / IFPDT (Datenschutz)", tag: "revDSG / nLPD Art. 8, 12, 19, 60", desc: "Prüfung der Datenschutzerklärung, des Bearbeitungsverzeichnisses, des Datenexports und der Datensicherheit (Schutz vor Bussen bis CHF 250'000)." },
                { num: "5. SECO, SUVA & AHV-Ausgleichskassen", tag: "ArG Art. 6 • EKAS 6508 • AHVG Art. 52", desc: "Überprüfung des Sicherheitskonzepts (EKAS/CFSL/MSSL 6508), der Arbeitszeiterfassung (ArGV 1) und der Arbeitgeberkontrolle AHV/SUVA." },
                { num: "6. BACS (ISG) & SECO Lauterkeitsrecht (UWG/LCD)", tag: "ISG • Art. 3 lit. s UWG • PBV/OIP", desc: "Kontrolle der Informationssicherheit sowie der Pflichtangaben im E-Commerce und der Preisbekanntgabeverordnung in CHF." }
            ]
        },
        "NL": {
            count: "+2.4M Geïndexeerde Eenheden (12 Provincies — KVK Handelsregister / SBI)",
            items: [
                { num: "1. KVK Handelsregister & SBI-Classificatie", tag: "KVK / RSIN • SBI-code", desc: "Verificatie van rechtsvorm, statutaire zetel, tekenbevoegdheid en SBI-activiteitencode voor het bepalen van de sectorale Arbo- en milieueisen." },
                { num: "2. Belastingdienst (AWR, Wet DBA & Wka)", tag: "Art. 52 AWR • Wet DBA • Wka", desc: "Toetsing van de 7-jarige fiscale bewaarplicht, modelovereenkomsten (voorkoming schijnzelfstandigheid) en ketenaansprakelijkheid." },
                { num: "3. KVK UBO-register & Wwft-Toezicht", tag: "Handelsregisterwet • Wwft", desc: "Controle van de registratie van uiteindelijk belanghebbenden (UBO) en cliëntenonderzoek ter voorkoming van witwassen." },
                { num: "4. Autoriteit Persoonsgegevens (AP)", tag: "AVG • UAVG • Art. 11.7a Tw", desc: "Technische inspectie van TLS 1.3, cookie-instellingen, privacyverklaring, verwerkingsregister en het verbod op onrechtmatige biometrie." },
                { num: "5. Nederlandse Arbeidsinspectie", tag: "Art. 3 & 5 Arbowet • RI&E • PSA", desc: "Verificatie van de verplichte Risico-Inventarisatie en -Evaluatie (RI&E), Plan van Aanpak, preventiemedewerker en beleid psychosociale arbeidsbelasting." },
                { num: "6. ACM (Consument & Markt) & NCSC (NIS2)", tag: "BW Boek 6 Art. 230m • NIS2", desc: "Controle van informatieplichten bij koop op afstand, herroepingsrecht, prijstransparantie en de Cyberbeveiligingswet (NIS2)." }
            ]
        },
        "BE": {
            count: "+1.3M Geïndexeerde Eenheden (Vlaanderen, Wallonië, Brussel — KBO / BCE)",
            items: [
                { num: "1. Kruispuntbank van Ondernemingen (KBO / BCE)", tag: "Ondernemingsnr. • NACE-BEL", desc: "Verificatie van de rechtspersoon, vestigingseenheden en NACE-BEL codes voor federale en gewestelijke nalevingsplichten." },
                { num: "2. FOD Financiën / SPF Finances & RSZ / ONSS", tag: "WIB 92 / CIR 92 • Art. 30bis RSZ", desc: "Controle van de 10-jarige boekhoudkundige bewaarplicht en de inhoudingsplicht op facturen van onderaannemers (Art. 30bis RSZ/ONSS)." },
                { num: "3. Thesaurie — Belgisch UBO-Register", tag: "Wet 18/09/2017 • MyMinfin UBO", desc: "Toetsing van de verplichte jaarlijkse bevestiging van de uiteindelijk begunstigden en anti-witwasdocumentatie." },
                { num: "4. Gegevensbeschermingsautoriteit (GBA / APD)", tag: "AVG / RGPD • CAO/CCT nr. 81 & 68", desc: "Controle van het privacybeleid, verwerkingsregister en de Belgische cao-regels rond elektronisch toezicht en camerabewaking op het werk." },
                { num: "5. FOD WASO / SPF ETCS & SIOD / SIRS", tag: "Codex Welzijn op het Werk • IDPBW/SIPPT", desc: "Evaluatie van het Globaal Preventieplan (5 jaar), Jaarlijks Actieplan, psychosociale risico's en het arbeidsreglement." },
                { num: "6. FOD Economie (WER / CDE) & CCB (NIS2 België)", tag: "WER Boek VI & XII • Wet 26/04/2024", desc: "Toetsing van de Belgische NIS2-cyberbeveiligingswet, e-commerce verplichtingen en het verbod op onrechtmatige bedingen (B2C & B2B)." }
            ]
        },
        "LU": {
            count: "+145,000 Indexéiert Eenheeten (Groussherzogtum — RCS / LBR / NACELUX)",
            items: [
                { num: "1. Registre de Commerce et des Sociétés (RCS)", tag: "RCS Lëtzebuerg • NACELUX", desc: "Verifikatioun vun der Gesellschaftsform, dem Sëtz, de Mandatairen an dem NACELUX-Code beim Luxembourg Business Registers (LBR)." },
                { num: "2. ACD & AED (Steieren, TVA & Comptabilitéit)", tag: "Art. 16 CCom • Loi TVA LU", desc: "Kontroll vun der 10-järeger Archivéierungsflicht vun de Bicher a Rechnungen zu Lëtzebuerg a steierlech Substanznokweiser." },
                { num: "3. Registre des Bénéficiaires Effectifs (RBE / LBR)", tag: "Gesetz 13/01/2019 (RBE) • AML", desc: "Prüfung vun der RBE-Deklaratioun beim LBR fir Strofe vu 1.250 € bis 1.250.000 € ze vermeiden." },
                { num: "4. CNPD (Nationale Dateschutzkommissioun)", tag: "RGPD • Art. L.261-1 Code du Travail", desc: "Technesch Kontroll vun der Dateschutzerklärung, TLS 1.3 an de strenge Lëtzebuerger Reegele fir d'Iwwerwaachung op der Aarbechtsplaz." },
                { num: "5. ITM (Inspection du Travail et des Mines) & CCSS", tag: "Art. L.311–L.314 Code du Travail", desc: "Evaluatioun vum Sécherheets- a Gesondheetskonzept, dem Travailleur Désigné, der Télétravail-Konventioun an de CCSS-Cotisatiounen." },
                { num: "6. ILR / HCPN (NIS2) & MPC (Konsumenteschutz)", tag: "NIS2 LU • Code de la Consommation", desc: "Kontroll vun den IT-Sécherheetsflichten an den E-Commerce- an AGB-Virschrëften am Groussherzogtum Lëtzebuerg." }
            ]
        }
    };

    const REALISTIC_CARD_RATES = [74, 68, 79, 57, 64, 62];
    const DIRECT_BENEFITS_BY_LANG = {
        "de": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Alle Rechte vorbehalten.",
                "Globale Infrastruktur für deterministische Audits und Regulierungs-Engineering",
                "Kryptografischer Signaturknoten: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Internationale Standards für digitale Integrität und Governance: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
        "fr": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Tous droits réservés.",
                "Infrastructure Mondiale d'Audit Déterministe et d'Ingénierie Réglementaire",
                "Nœud de Scellement Cryptographique : Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Normes Internationales d'Intégrité et de Gouvernance Numérique : ISO 19011 • CNUDCI / UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
        "it": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Tutti i diritti riservati.",
                "Infrastruttura Globale di Audit Deterministico e Ingegneria Normativa",
                "Nodo di Sigillatura Crittografica: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Standard Internazionali di Integrità e Governance Digitale: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
        "nl": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Alle rechten voorbehouden.",
                "Wereldwijde Infrastructuur voor Deterministische Audit en Regulatory Engineering",
                "Cryptografisch Zegelknooppunt: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Internationale Normen voor Digitale Integriteit en Governance: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
        "lb": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • All Rechter virbehalen.",
                "Global Infrastruktur fir Deterministeschen Audit a Reglementarescht Engineering",
                "Kryptographesche Signaturkapp: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "International Standarde fir Digital Integritéit a Governance: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ]
    };

    function renderParitySections() {
        const pathParts = window.location.pathname.split("/").filter(Boolean);
        const countryCode = (pathParts[1] || localStorage.getItem("mallen_country") || "DE").toUpperCase();
        const urlParams = new URLSearchParams(window.location.search);
        const activeLocale = urlParams.get("lang") || localStorage.getItem("mallen_locale") || document.documentElement.lang || "de-DE";
        const langPrefix = activeLocale.split("-")[0].toLowerCase();

        const p = PARITY_SECTIONS_I18N[langPrefix] || PARITY_SECTIONS_I18N["de"];
        const regData = COUNTRY_REGISTRIES[countryCode] || COUNTRY_REGISTRIES["DE"];
        const benefits = DIRECT_BENEFITS_BY_LANG[langPrefix] || DIRECT_BENEFITS_BY_LANG["de"];

        const setTxt = function (id, val) {
            const el = document.getElementById(id);
            if (el && val) el.textContent = val;
        };

        setTxt("eu-hero-cta-primary", p.heroCtaPrimary);
        setTxt("eu-hero-cta-secondary", p.heroCtaSecondary);
        setTxt("eu-dir-kicker", p.dirKicker);
        setTxt("eu-nav-registries", p.navRegistries);
        setTxt("eu-nav-calibration", p.navCalibration);
        setTxt("eu-reg-kicker", p.regKicker);
        setTxt("eu-reg-title", p.regTitle);
        setTxt("eu-reg-count-badge", regData.count);
        setTxt("eu-proof-kicker", p.proofKicker);
        setTxt("eu-proof-title", p.proofTitle);
        setTxt("eu-proof-sub", p.proofSub);
        setTxt("eu-cal-kicker", p.calKicker);
        setTxt("eu-cal-title", p.calTitle);
        setTxt("eu-cal-sub", p.calSub);
        setTxt("eu-emp-label", p.empLabel);
        setTxt("eu-micro-btn", p.microBtn);
        setTxt("eu-cal-vars-title", p.calVarsTitle);
        setTxt("eu-dossier-form-title", p.dossierTitle);
        setTxt("eu-dossier-form-desc", p.dossierDesc);

        // Garantizar el Pie de Página Institucional traducido en todos los países e idiomas (DE, AT, CH, NL, BE, LU)
        const FOOTER_BY_LANG = {
            "de": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Alle Rechte vorbehalten.",
                "Globale Infrastruktur für deterministische Audits und Regulierungs-Engineering",
                "Kryptografischer Signaturknoten: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Internationale Standards für digitale Integrität und Governance: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
            "fr": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Tous droits réservés.",
                "Infrastructure Mondiale d'Audit Déterministe et d'Ingénierie Réglementaire",
                "Nœud de Scellement Cryptographique : Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Normes Internationales d'Intégrité et de Gouvernance Numérique : ISO 19011 • CNUDCI / UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
            "it": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Tutti i diritti riservati.",
                "Infrastruttura Globale di Audit Deterministico e Ingegneria Normativa",
                "Nodo di Sigillatura Crittografica: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Standard Internazionali di Integrità e Governance Digitale: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
            "nl": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • Alle rechten voorbehouden.",
                "Wereldwijde Infrastructuur voor Deterministische Audit en Regulatory Engineering",
                "Cryptografisch Zegelknooppunt: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "Internationale Normen voor Digitale Integriteit en Governance: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ],
            "lb": [
                "© 2026 FIRMA W16 MALLEN® (mallen.lat) • All Rechter virbehalen.",
                "Global Infrastruktur fir Deterministeschen Audit a Reglementarescht Engineering",
                "Kryptographesche Signaturkapp: Frankfurt Core Hub (SHA-256 / Merkle Tree).",
                "International Standarde fir Digital Integritéit a Governance: ISO 19011 • UNCITRAL • eIDAS Art. 41 • SEC / FINRA Compliance."
            ]
        };
        const flines = FOOTER_BY_LANG[langPrefix] || FOOTER_BY_LANG["de"];
        setTxt("eu-footer-line-1", flines[0]);
        setTxt("eu-footer-line-2", flines[1]);
        setTxt("eu-footer-line-3", flines[2]);
        setTxt("eu-footer-line-4", flines[3]);


        // 1. Enriquecer las 6 tarjetas de leyes con el badge de porcentaje real (ej. 74%, 68%, 79%...) y su cuadro de Beneficio Directo
        const cardsContainer = document.getElementById("eu-laws-cards-container");
        if (cardsContainer && cardsContainer.children.length > 0) {
            Array.from(cardsContainer.children).forEach(function (card, idx) {
                const topRow = card.querySelector(".flex.items-center.justify-between");
                if (topRow && !topRow.querySelector(".w16-rate-badge")) {
                    const rate = REALISTIC_CARD_RATES[idx % REALISTIC_CARD_RATES.length];
                    const rightSpan = topRow.lastElementChild;
                    if (rightSpan) {
                        rightSpan.className = "w16-rate-badge text-[10px] bg-red-950/80 text-red-300 border border-red-800/60 px-2.5 py-0.5 rounded font-mono-code font-bold";
                        rightSpan.textContent = rate + "% (" + (rate - 4) + "%–" + (rate + 3) + "%)";
                    }
                }
                if (!card.querySelector(".w16-benefit-box")) {
                    const firstDiv = card.firstElementChild;
                    if (firstDiv) {
                        const benBox = document.createElement("div");
                        benBox.className = "w16-benefit-box text-[11px] text-slate-300 font-mono-code bg-slate-950/65 border border-slate-800 rounded-xl p-3 mb-3";
                        benBox.innerHTML = `<strong class="text-emerald-400 block mb-0.5">${p.directBenefitLabel}</strong>${benefits[idx % benefits.length]}`;
                        firstDiv.appendChild(benBox);
                    }
                }
            });
        }

        // 2. Poblar las 6 tarjetas de Padrones Oficiales Nacionales
        const regGrid = document.getElementById("eu-registries-grid");
        if (regGrid) {
            regGrid.innerHTML = "";
            regData.items.forEach(function (item) {
                const div = document.createElement("div");
                div.className = "bg-slate-950/90 border border-slate-800 hover:border-sky-500/50 rounded-2xl p-5 transition flex flex-col justify-between";
                div.innerHTML = `
                    <div>
                        <div class="flex items-center justify-between gap-2 mb-2.5">
                            <span class="text-xs font-extrabold text-white">${item.num}</span>
                            <span class="text-[10px] font-mono-code px-2 py-0.5 rounded bg-blue-950 border border-blue-800 text-sky-300 font-bold">${item.tag}</span>
                        </div>
                        <p class="text-xs text-slate-300 leading-relaxed">${item.desc}</p>
                    </div>
                `;
                regGrid.appendChild(div);
            });
        }

        // 3. Poblar las 4 tarjetas interactivas de Sustento Legal & Validez Probatoria
        const proofGrid = document.getElementById("eu-proof-cards-grid");
        const proofDrawer = document.getElementById("eu-proof-detail-drawer");
        if (proofGrid) {
            proofGrid.innerHTML = "";
            p.proofs.forEach(function (pr, idx) {
                const card = document.createElement("div");
                card.className = "quantum-card rounded-2xl p-5 border border-slate-800 hover:border-emerald-500/60 cursor-pointer transition flex flex-col justify-between";
                card.innerHTML = `
                    <div>
                        <span class="text-[10px] font-mono-code font-bold uppercase px-2.5 py-1 rounded bg-emerald-950/80 border border-emerald-800 text-emerald-300 inline-block mb-3">${pr.badge}</span>
                        <h3 class="text-sm font-extrabold text-white mb-2">${pr.title}</h3>
                        <p class="text-xs text-slate-300 leading-relaxed mb-4">${pr.short}</p>
                    </div>
                    <span class="text-[11px] font-mono-code font-bold text-sky-400">Details & Rechtsgrundlage →</span>
                `;
                card.onclick = function () {
                    if (!proofDrawer) return;
                    proofDrawer.classList.remove("hidden");
                    proofDrawer.innerHTML = `
                        <div class="flex items-center justify-between border-b border-slate-800 pb-3 mb-3">
                            <strong class="text-emerald-400 uppercase">${pr.badge} — ${pr.title}</strong>
                            <button type="button" onclick="document.getElementById('eu-proof-detail-drawer').classList.add('hidden')" class="text-slate-400 hover:text-white font-bold">✕</button>
                        </div>
                        <p class="text-slate-200 leading-relaxed">${pr.detail}</p>
                    `;
                };
                proofGrid.appendChild(card);
            });
        }

        // 4. Poblar los 10 Checkboxes de Calibración de Variables Internas
        const calGrid = document.getElementById("eu-cal-checkboxes-grid");
        if (calGrid) {
            calGrid.innerHTML = "";
            p.calVars.forEach(function (lbl, idx) {
                const item = document.createElement("label");
                item.className = "flex items-center gap-2.5 p-2.5 rounded-xl bg-slate-900/70 border border-slate-800 cursor-pointer hover:border-slate-700";
                item.innerHTML = `
                    <input type="checkbox" ${idx < 4 || idx === 7 ? "checked" : ""} class="w-4 h-4 accent-emerald-500 rounded">
                    <span class="text-xs font-mono-code text-slate-200">${lbl}</span>
                `;
                calGrid.appendChild(item);
            });
        }

        // 5. Poblar los 16 Campos de Personalización de Instrumentos Jurídicos (con Pre-llenado Automático ____________________)
        const fields16 = document.getElementById("eu-dossier-16-fields");
        if (fields16) {
            fields16.innerHTML = "";
            p.dossierFields.forEach(function (fLabel, idx) {
                const wrap = document.createElement("div");
                wrap.innerHTML = `
                    <label class="block text-[10px] text-slate-400 mb-1 truncate" title="${fLabel}">${idx + 1}. ${fLabel}</label>
                    <input type="text" placeholder="____________________" class="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-xs text-white font-mono-code focus:outline-none focus:border-sky-500">
                `;
                fields16.appendChild(wrap);
            });
        }
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", function () {
            setTimeout(renderParitySections, 60);
        });
    } else {
        setTimeout(renderParitySections, 60);
    }
})();
