pub const APP_NAME: &str = "abuse-service";

use serde_derive::Deserialize;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    #[serde(rename = "MyServiceBusHost")]
    pub sb_host: String,
    #[serde(rename = "SeqUrl")]
    pub seq_url: String,
    #[serde(rename = "PgDb")]
    pub pgdb: Option<String>,
    #[serde(rename = "InstrumentsFilter")]
    pub instruments_filter: Option<Vec<String>>,
    #[serde(rename = "MaxDecimalPlacesLimit")]
    pub max_decimal_places_limit: Option<u8>,
}

pub mod reader;