// @generated automatically by Diesel CLI.

diesel::table! {
    rates_history (datetime, id) {
        #[max_length = 255]
        id -> Varchar,
        datetime -> Timestamp,
        bid -> Float8,
        ask -> Float8,
        open -> Float8,
        close -> Float8,
        price -> Float8,
    }
}
