pub mod logger;

// Normalization of Instrument ID
pub trait Normalization {
    fn normalize_instrument_id(&self) -> String;
}

pub trait NormalizationOption {
    fn normalize_instrument_id(&self) -> Option<String>;
}

impl Normalization for String {

    fn normalize_instrument_id(&self) -> String {
        self.to_lowercase()
    }

}

impl NormalizationOption for Option<String> {

    fn normalize_instrument_id(&self) -> Option<String> {
        match self {
            None => None,
            Some(id) => Some(id.normalize_instrument_id()),
        }
    }

}