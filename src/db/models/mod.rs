use diesel::prelude::*;
use serde_derive::Serialize;

use crate::models::BidAsk;

#[derive(Queryable, Selectable, Insertable, Debug, QueryableByName)]
#[diesel(table_name = super::schema::rates_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rate {
    pub id: String,
    pub datetime: chrono::NaiveDateTime,
    pub bid: f64,
    pub ask: f64,
    pub open: f64,
    pub close: f64,
    pub price: f64,
}

impl Rate {

    pub fn from(bidask: &BidAsk) -> Self {
        Self { 
            id: bidask.id.clone(), 
            datetime: chrono::NaiveDateTime::from_timestamp_millis(bidask.date_time as i64).unwrap(), 
            bid: bidask.bid, 
            ask: bidask.ask, 
            open: bidask.open, 
            close: bidask.close, 
            price: bidask.price 
        }
    }

    pub fn to_bidask(&self) -> BidAsk {
        BidAsk { 
            id: self.id.clone(), 
            date_time: self.datetime.timestamp_millis(), 
            bid: self.bid, 
            ask: self.ask, 
            open: self.open, 
            close: self.close, 
            price: self.price
        }
    }
}