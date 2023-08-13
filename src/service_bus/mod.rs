mod subscribers;
mod handlers;
mod tests; 

use std::sync::Arc;

use my_logger::LogLevel;
use my_service_bus_shared::queue::TopicQueueType;
use yh_services_sdk::yh_service_bus::manager::{ServiceBusManager, ServiceBusSubscriber};

use crate::{app_context::AppContext, app_constants, common::logger::StrLogger};

pub const TOPIC_ID: &'static str = "bid-ask";

pub async fn subscribe_to_service_bus_topics(url: &str, ctx: Arc<AppContext>) {
    
    let process = "subscribe_to_service_bus_topics";
    "Initialize ServiceBus subscibers"
        .log(process, LogLevel::Info);
    
    // init
    let sb = ServiceBusManager::init_connection(url, app_constants::APP_NAME);

    // subscribe to create new Alert Price  
    sb.subscribe_event(
        TOPIC_ID,
        app_constants::APP_NAME,
        TopicQueueType::DeleteOnDisconnect,
        Arc::new(subscribers::BidAskEventSubscriber {
            ctx: ctx.clone()
        })
    ).await;

    // start listener 
    "Start ServiceBus topics listeners"
        .log(process, LogLevel::Info);
    sb.start().await

}