use std::sync::Arc;

use async_trait::async_trait;
use my_logger::LogLevel;
use my_service_bus_tcp_client::subscribers::{MessagesReader, SubscriberCallback};

use crate::{app_context::AppContext, common::logger::StrLogger, models::BidAsk};

use super::handlers;

const PROCESS_NAME: &'static str = "BidAskEventSubscriber";

//
pub struct BidAskEventSubscriber {
    pub ctx: Arc<AppContext>
}

#[async_trait]
impl SubscriberCallback for BidAskEventSubscriber {
    async fn new_events(&self, mut messages_reader: MessagesReader) {
        // "Got event to create a new price alert".log(
        //     &format!("{}-{}", PROCESS_NAME, "new_events"),
        //     LogLevel::Debug
        // );

        for msg in messages_reader.get_messages() {
            match BidAsk::parse(&msg.content[1..]) {
                Ok(event) => {
                    handlers::handle_bid_ask_incoming(event, self.ctx.clone()).await;
                }
                Err(error_mess) => {
                    format!("Error when reading event for bidask: {}", error_mess).as_str().log(
                        &format!("{}-{}", PROCESS_NAME, "new_events"),
                        LogLevel::Error
                    );
                }
            };
            messages_reader.handled_ok(&msg);
        }
    }
}
