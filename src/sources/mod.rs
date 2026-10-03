pub mod inegi;
pub mod inegi;

use crate::engine::models::EmpresaB2B;

/// Generisches Interface für weltweite Datenquellen (DENUE, Handelsregister, etc.).
///
/// Jede Implementierung ist dafür verantwortlich, eine rohe Zeile ihrer
/// Quelldaten in ein normalisiertes [`EmpresaB2B`]-Record zu überführen.
pub trait GlobalSource {
    /// Ländercode (ISO-3166-1 alpha-2), den diese Quelle standardmäßig liefert.
    fn default_country_code(&self) -> &'static str;

    /// Parst eine einzelne Rohzeile (z. B. CSV-Zeile) in ein [`EmpresaB2B`].
    ///
    /// Gibt `None` zurück, wenn die Zeile ungültig, leer oder ein Header ist.
    fn parse_row(&self, row: &str) -> Option<EmpresaB2B>;
}
