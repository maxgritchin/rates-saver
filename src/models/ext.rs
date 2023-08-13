use prost::DecodeError;
use serde::ser::{Serialize, Serializer, SerializeStruct};


use super::BidAsk;

impl BidAsk {
    
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        ::prost::Message::decode(payload)
    }

    pub fn normalize_decimal_places(&mut self, decimal_places: usize) {
        // price 
        let fmt_val = format!("{:.*}", decimal_places, self.price);
        self.price = fmt_val.parse::<f64>().unwrap();
    }

}

impl Serialize for BidAsk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BidAsk", 7)?;
        s.serialize_field("Id", &self.id)?;
        s.serialize_field("DateTime", &self.date_time)?;
        s.serialize_field("Bid", &self.bid)?;
        s.serialize_field("Ask", &self.ask)?;
        s.serialize_field("Open", &self.open)?;
        s.serialize_field("Close", &self.close)?;
        s.serialize_field("Price", &self.price)?;
        s.end()
    }
}