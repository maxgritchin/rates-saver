#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAsk {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub date_time: i64,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
    #[prost(double, tag = "5")]
    pub open: f64,
    #[prost(double, tag = "6")]
    pub close: f64,
    #[prost(double, tag = "7")]
    pub price: f64,
}
