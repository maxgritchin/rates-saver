use super::BidAsk;


#[test]
fn test_normalize_decimal_places() {
    // assert
    let mut rate = BidAsk {
        ..Default::default()
    };

    // act
    rate.price = 123.456789;
    rate.normalize_decimal_places(2);
    assert_eq!(rate.price, 123.46);
    
    rate.price = 123.456789;
    rate.normalize_decimal_places(4);
    assert_eq!(rate.price, 123.4568);
    
    rate.price = 123.456789;
    rate.normalize_decimal_places(6);
    assert_eq!(rate.price, 123.456789);
    
    rate.price = 123.456789;
    rate.normalize_decimal_places(8);
    assert_eq!(rate.price, 123.45678900);
}