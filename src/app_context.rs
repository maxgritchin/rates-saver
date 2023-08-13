use std::{sync::{Arc, Mutex}, collections::{HashSet, HashMap}};

use my_service_bus_shared::queue::TopicQueueType;

use crate::{storages::{StorageWriter, PersistingType, self, StorageReader}, models::BidAsk, app_config::AppConfig};

pub trait Context {
    fn get_storage_writer(&self) -> Arc<dyn StorageWriter + Send + Sync>;
    fn get_storage_reader(&self) -> Arc<dyn StorageReader + Send + Sync>;
    fn get_type_of_service_bus_queue(&self) -> TopicQueueType;
    fn get_decimal_places_limit(&self) -> Option<u8>;
    fn is_instrument_in_filter_set(&self, instrument_id: &str) -> bool;
    fn check_if_rate_has_no_cahnges_since_last_time(&self, rate: &BidAsk) -> bool;
    fn remember_last_rate(&self, rate: &BidAsk);
    fn get_config(&self) -> Arc<AppConfig>;
}

pub struct AppContext {
    config: Arc<AppConfig>,
    storage_writer: Option<Arc<dyn StorageWriter + Send + Sync>>,
    storage_reader: Option<Arc<dyn StorageReader + Send + Sync>>,
    type_of_service_bus_queue: TopicQueueType,
    bidask_filter: Option<Arc<HashSet<String>>>,
    last_rates: Arc<Mutex<HashMap<String, f64>>>,
    
}

impl AppContext {
    
    pub fn new(config: &AppConfig) -> Self {
        let mut ctx = AppContext { 
            config: Arc::new(config.clone()),
            storage_writer: None,
            storage_reader: None,
            type_of_service_bus_queue: TopicQueueType::DeleteOnDisconnect,
            bidask_filter: None,
            last_rates: Arc::new(Mutex::new(HashMap::new()))
        };

        if config.instruments_filter.is_some() {
            ctx.with_instruments_filter(&config.instruments_filter.clone().unwrap());
        }
       
        Self {
            config: ctx.config,
            storage_writer: ctx.storage_writer,
            storage_reader: ctx.storage_reader,
            type_of_service_bus_queue: ctx.type_of_service_bus_queue,
            bidask_filter: ctx.bidask_filter,
            last_rates: ctx.last_rates
        }
    }

    pub fn with_sb_queue_type(&mut self, qt: TopicQueueType) -> &mut Self {
        self.type_of_service_bus_queue = qt;
        self
    }

    pub fn with_instruments_filter(&mut self, filter: &[String]) -> &mut Self {
        let lowercased: HashSet<String> = filter.iter()
            .map(|s| s.to_lowercase())
            .collect();

        self.bidask_filter = Some(Arc::new(lowercased));
        self
    }

    pub async fn build_with_custom_storage(&self, 
        sw: Box<dyn StorageWriter + Send + Sync>,
        sr: Box<dyn StorageReader + Send + Sync>) -> Self {
        // if need to prepare such a make migrations 
        sw.prepare().await;
        
        // return
        Self {
            config: self.config.clone(),
            storage_writer: Some(Arc::from(sw)),
            storage_reader: Some(Arc::from(sr)),
            type_of_service_bus_queue: self.type_of_service_bus_queue,
            bidask_filter: self.bidask_filter.clone(),
            last_rates: self.last_rates.clone()
        }
    }

    pub async fn build(&self, storage_type: &PersistingType) -> Self {
        let sw = storages::create_storage_writer(storage_type, self);
        let sr = storages::create_storage_reader(storage_type, self);
        self.build_with_custom_storage(sw, sr).await
    }

}

impl Context for AppContext {

    fn get_storage_writer(&self) -> Arc<dyn StorageWriter + Send + Sync> {
        match &self.storage_writer {
            Some(sw) => sw.clone(),
            None => 
                panic!("Storage writer have not been setup properly!")
        }
    }

    fn get_storage_reader(&self) -> Arc<dyn StorageReader + Send + Sync>{
        match &self.storage_reader {
            Some(sr) => sr.clone(),
            None => 
                panic!("Storage reader have not been setup properly!")
        }
    }

    fn get_type_of_service_bus_queue(&self) -> TopicQueueType {
        self.type_of_service_bus_queue
    }

    fn get_decimal_places_limit(&self) -> Option<u8> {
        self.config.max_decimal_places_limit
    }

    fn is_instrument_in_filter_set(&self, instrument_id: &str) -> bool { 
        match &self.bidask_filter {
            None => true,
            Some(hs) => hs.contains(&instrument_id.to_lowercase())
        }
    }

    fn check_if_rate_has_no_cahnges_since_last_time(&self, rate: &BidAsk) -> bool {
        let last_rates = self.last_rates.lock().unwrap();
        
        // check if has changes for price since last rate
        let key = rate.id.to_lowercase();
        match last_rates.contains_key(&key) {
            true => *last_rates.get(&key).unwrap() == rate.price,
            false => false
        }
    }

    fn remember_last_rate(&self, rate: &BidAsk) {
        let mut last_rates = self.last_rates.lock().unwrap();
        last_rates.insert(rate.id.to_lowercase(), rate.price);
    }

    fn get_config(&self) -> Arc<AppConfig> {
        self.config.clone()
    }
}