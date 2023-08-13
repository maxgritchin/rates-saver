use std::sync::Arc;

use tonic::Response;

use crate::{app_context::{AppContext, Context}, storages::{RatesFilter, DateRange}, common::NormalizationOption};

use super::grpc::{ratessaver::{GetSavedRatesRequest, GetSavedRatesResponse, rates_saver_service_server::RatesSaverService}, bidask};

const PROC_NAME: &str = "RatesSaverSrv";

pub struct RatesSaverSrvImpl {
    ctx: Arc<AppContext>,
}

impl RatesSaverSrvImpl {

    pub fn new(ctx: Arc<AppContext>) -> Self {
        RatesSaverSrvImpl { ctx }
    }

}

#[tonic::async_trait]
impl RatesSaverService for RatesSaverSrvImpl {

    async fn get_saved_rates(
        &self,
        request: tonic::Request<GetSavedRatesRequest>,
    ) -> std::result::Result<
        tonic::Response<GetSavedRatesResponse>,
        tonic::Status,
    > {

        // create parameters 
        let req = request.into_inner();
        let filter = RatesFilter { 
            id: req.id.normalize_instrument_id(),
            date_range: match req.date_range {
                None => None,
                Some(tr) => Some(DateRange {
                    from: tr.from as u64,
                    to: tr.to as u64,
                })
            }
        };

        // call reader
        let reader = self.ctx.get_storage_reader().clone();
        let resp = reader.get_rates(filter).await;

        //
        Ok(Response::new(GetSavedRatesResponse {
            rates: resp.into_iter().map(|x| bidask::BidAsk {
                id: x.id,
                bid: x.bid,
                ask: x.ask,
                close: x.close,
                open: x.open,
                date_time: x.date_time,
                price: x.price
            }).collect(),
        }))

    }

}