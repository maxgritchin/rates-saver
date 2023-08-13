#[cfg(test)]
mod sb_handler_tests {
    use std::sync::Arc;

    use mockall::predicate::{*, self};

    use crate::app_config::AppConfig;
    use crate::app_context::AppContext;
    use crate::storages::{MockStorageWriter, MockStorageReader};
    use crate::models::BidAsk;
    use crate::service_bus::handlers::handle_bid_ask_incoming;
    
    #[tokio::test]
    async fn test_that_not_write_to_storage_if_no_changes_in_rates() {
        // arrange 
        let rate = BidAsk {
            id: "btcusdt".to_string(),
            price: 123.456,
            ..Default::default() 
        };

        let mut sw_mock = MockStorageWriter::new();
        sw_mock.expect_write()
            .times(1)
            .returning(|_| ()); // only one time to write as rate not changed

        sw_mock.expect_prepare()
            .times(1)
            .returning(|| ());

        let mut sr_mock = MockStorageReader::new();

        let config = AppConfig {
            instruments_filter: Some(vec!["ethusdt".to_string(), "btcusdt".to_string()]), 
            max_decimal_places_limit: Some(15),
            ..Default::default()
        };
        let app_ctx = AppContext::new(&config)
            .build_with_custom_storage(Box::new(sw_mock), Box::new(sr_mock))
            .await;

        let ctx = Arc::new(app_ctx);

        // act 
        for _ in 0..10  {
            handle_bid_ask_incoming(rate.clone(), ctx.clone()).await;
        }
    }

    #[tokio::test]
    async fn test_that_write_to_storage_several_rates_with_diff_price() {
        // arrange 
        let mut rate = BidAsk {
            id: "btcusdt".to_string(),
            price: 123.456,
            ..Default::default() 
        };

        let mut sw_mock = MockStorageWriter::new();
        sw_mock.expect_write()
            .times(2)
            .returning(|_| ()); // only one time to write as rate not changed

        sw_mock.expect_prepare()
            .times(1)
            .returning(|| ());

        let mut sr_mock = MockStorageReader::new();

        let config = AppConfig {
            instruments_filter: Some(vec!["ethusdt".to_string(), "btcusdt".to_string()]), 
            max_decimal_places_limit: Some(15),
            ..Default::default()
        };
        let app_ctx = AppContext::new(&config)
            .build_with_custom_storage(Box::new(sw_mock), Box::new(sr_mock))
            .await;

        let ctx = Arc::new(app_ctx);

        // act 
        handle_bid_ask_incoming(rate.clone(), ctx.clone()).await;

        rate.price = rate.price + 10_f64;
        handle_bid_ask_incoming(rate.clone(), ctx.clone()).await;

    }

    #[tokio::test]
    async fn test_that_skip_rates_that_not_in_filter_set() {
        // arrange 
        let rate = BidAsk {
            id: "btcusdt".to_string(),
            price: 123.456,
            ..Default::default() 
        };

        let mut sw_mock = MockStorageWriter::new();
        sw_mock.expect_write()
            .times(0)
            .returning(|_| ()); // only one time to write as rate not changed

        sw_mock.expect_prepare()
            .times(1)
            .returning(|| ());

        let mut sr_mock = MockStorageReader::new();

        let config = AppConfig {
            instruments_filter: Some(vec!["ethusdt".to_string()]), 
            max_decimal_places_limit: Some(15),
            ..Default::default()
        };
        let app_ctx = AppContext::new(&config)
            .build_with_custom_storage(Box::new(sw_mock), Box::new(sr_mock))
            .await;

        let ctx = Arc::new(app_ctx);

        // act 
        handle_bid_ask_incoming(rate.clone(), ctx.clone()).await;
    }

    #[tokio::test]
    async fn test_that_round_decimal_part_to_max_limit_from_config() {
        // arrange 
        let rate = BidAsk {
            id: "btcusdt".to_string(),
            price: 123.123456789,
            ..Default::default() 
        };

        let mut sw_mock = MockStorageWriter::new();
        sw_mock.expect_write()
            .withf(move |rate: &BidAsk| rate.price == 123.1235) // rounded value
            .times(1)
            .returning(|_| ()); // only one time to write as rate not changed

        sw_mock.expect_prepare()
            .times(1)
            .returning(|| ());

        let mut sr_mock = MockStorageReader::new();
        
        let config = AppConfig {
            instruments_filter: Some(vec!["btcusdt".to_string()]), 
            max_decimal_places_limit: Some(4),
            ..Default::default()
        };
        let app_ctx = AppContext::new(&config)
            .build_with_custom_storage(Box::new(sw_mock), Box::new(sr_mock))
            .await;

        let ctx = Arc::new(app_ctx);

        // act 
        handle_bid_ask_incoming(rate, ctx.clone()).await;
    }

}