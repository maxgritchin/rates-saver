use async_trait::async_trait;
use chrono::prelude::*;

use crate::models::BidAsk;

use super::{StorageWriter, StorageReader, RatesFilter};

pub struct ConsoleStorage { }

impl ConsoleStorage {

    pub fn new() -> Self {
        Self {}
    }

}

#[async_trait]
impl StorageWriter for ConsoleStorage {

    async fn write(&self, rate: &BidAsk) {
        
        let dt = Local::now();
        println!("===> {} :: {:?}", dt, rate);

    }

    async fn prepare(&self) {
        // nothing to do here
    }

}

#[async_trait]
impl StorageReader for ConsoleStorage {

    async fn get_rates(&self, filter: RatesFilter) -> Vec<BidAsk>{
    
        println!("===> Got request to get rates from history: {:?}", filter);
        vec![]
    }

}