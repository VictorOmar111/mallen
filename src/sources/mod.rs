use csv::StringRecord;

pub mod inegi;

pub trait GlobalSource {
    fn parse_row(&self, record: &StringRecord) -> Option<crate::engine::models::EmpresaB2B>;
    fn default_country_code(&self) -> &'static str;
}
