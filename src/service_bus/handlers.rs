use std::sync::Arc;

use my_logger::LogLevel;

use crate::{models::BidAsk, app_context::Context, common::{logger::StrLogger, Normalization}};

//
pub async fn handle_bid_ask_incoming(rate: BidAsk, ctx: Arc<dyn Context + Send + Sync>) {

    // filter 
    if ctx.is_instrument_in_filter_set(&rate.id) {
        
        let mut rate = rate.clone();

        // normalize rate 
        if ctx.get_decimal_places_limit().is_some() {
            rate.normalize_decimal_places(ctx.get_decimal_places_limit().unwrap() as usize)
        }
        
        // check if has changes for price since last rate
        let has_changes = !ctx.check_if_rate_has_no_cahnges_since_last_time(&rate);

        // persist to storage 
        if has_changes {
            // normalization 
            rate.id = rate.id.normalize_instrument_id();

            // write 
            let writer = ctx.get_storage_writer().clone();
            writer.write(&rate).await;

            // track last change 
            ctx.remember_last_rate(&rate);

            // 
            format!("BidAsk written to the storage for '{}'", rate.id)
                .log("handle_bid_ask_incoming", LogLevel::Debug);
        }
        else {
            format!("No changes since the last rate for '{}'", rate.id)
                .log("handle_bid_ask_incoming", LogLevel::Debug);
        }
    
    }

}