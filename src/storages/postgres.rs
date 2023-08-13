use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

use async_trait::async_trait;
use chrono::prelude::*;

use crate::{models::BidAsk, db::{DbManager, self, Rate}, common::logger::StrLogger};

use super::{StorageWriter, StorageReader, RatesFilter};

pub struct PostgresStorage {
    db_manager: Arc<DbManager>,
    queue: Arc<Mutex<Vec<BidAsk>>>,
 }

impl PostgresStorage {

    pub fn new(conn_str: &str) -> Self {
        Self {
            db_manager: Arc::new(DbManager::new(conn_str)),
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

}

#[async_trait]
impl StorageReader for PostgresStorage {

    async fn get_rates(&self, filter: RatesFilter) -> Vec<BidAsk> {
    
        println!("===> Got request to get rates from history: {:?}", filter);
        
        let mut db_connection = self.db_manager.get_connection().await;
        let res = db::get_rates(&mut db_connection, filter).await;

        res.into_iter().map(|x| x.to_bidask()).collect()
    }

}

#[async_trait]
impl StorageWriter for PostgresStorage {

    async fn write(&self, rate: &BidAsk) {
        
        // log 
        let dt = Local::now();
        println!("===> {} :: {:?}", dt, rate);

        // add the BidAsk to the internal queue
        self.queue.lock().unwrap().push(rate.clone());
    }

    async fn prepare(&self) {

        // make migrations 
        self.db_manager.apply_migrations().await.expect("Exception iccured during DB migrations");

        // start the background task
        let queue = self.queue.clone();
        let db_manager = self.db_manager.clone();
        tokio::spawn(async move {
            background_flush(queue, db_manager).await;
        });
    }

}

async fn background_flush(queue: Arc<Mutex<Vec<BidAsk>>>, db_manager: Arc<DbManager>) {
    loop {
        sleep(Duration::from_secs(3)).await; // wait before processing the queue

        // get data to flush
        let data = {
            let mut queue = queue.lock().unwrap();
            
            // skip if the queue is empty
            if queue.is_empty() {
                continue; 
            }
            
            // clone the data to be processed outside the lock
            // Mutex cannot be Copy in other thred 
            // need to release quick
            let to_proceed = queue.clone();
            queue.clear();

            to_proceed
        };

        format!("Flush into DB...")
            .log("postgres - write", my_logger::LogLevel::Debug);
        
        // persist
        let mut db_connection = db_manager.get_connection().await;
        let insert_model: Vec<Rate> = data.into_iter().map(|rate| Rate::from(&rate)).collect();

        format!("{:?} objs to persist", insert_model)
            .log("postgres - write", my_logger::LogLevel::Debug);
        db::batch_insert(&mut db_connection, &insert_model).await;

        // log
        let ids: Vec<String> = insert_model.into_iter().map(|x| x.id).collect();
        format!("{:?} persisted into DB", ids)
            .log("postgres - write", my_logger::LogLevel::Debug);
    }
}