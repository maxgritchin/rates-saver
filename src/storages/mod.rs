use async_trait::async_trait;
use serde_derive::{Serialize, Deserialize};
use yh_services_sdk::yh_logger::Logger;

use crate::{models::BidAsk, app_context::{AppContext, Context}};

/// The type of storage where data will be persisted
#[derive(Debug)]
pub enum PersistingType {
    Postgres,
    Console
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(rename = "From")]
    pub from: u64,
    #[serde(rename = "To")]
    pub to: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatesFilter {
    #[serde(rename = "Id")]
    pub id: Option<String>, 
    #[serde(rename = "DateRange")]
    pub date_range: Option<DateRange>,
}

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait StorageWriter: Send + Sync {
    async fn write(&self, rate: &BidAsk);
    async fn prepare(&self);
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait StorageReader {
    async fn get_rates(&self, filter: RatesFilter) -> Vec<BidAsk>;
}

// export
mod console;
mod postgres;

pub fn create_storage_writer(pt: &PersistingType, ctx: &AppContext) -> Box<dyn StorageWriter + Send + Sync> {

    Logger::log_info(
        format!("Storage type '{:?}' is used to persist incoming tikers", pt), 
        "create_storage_writer".to_owned());

    match pt {
        PersistingType::Console =>
            Box::new(console::ConsoleStorage::new()),
        PersistingType::Postgres => {
            let pgdb = ctx.get_config().pgdb.clone().unwrap();
            Box::new(postgres::PostgresStorage::new(&pgdb))
        },  
        _ => 
            panic!("'{:?}' does not supported yet!", pt)
    }

}

pub fn create_storage_reader(pt: &PersistingType, ctx: &AppContext) -> Box<dyn StorageReader + Send + Sync> {

    Logger::log_info(
        format!("Storage type '{:?}' is used to persist incoming tikers", pt), 
        "create_storage_reader".to_owned());

    match pt {
        PersistingType::Console =>
            Box::new(console::ConsoleStorage::new()),
        PersistingType::Postgres => {
            let pgdb = ctx.get_config().pgdb.clone().unwrap();
            Box::new(postgres::PostgresStorage::new(&pgdb))
        },  
        _ => 
            panic!("'{:?}' does not supported yet!", pt)
    }

}