mod schema; 
mod models;
mod db_manager;

use chrono::NaiveDateTime;
pub use db_manager::DbManager;
pub use models::Rate;
pub use schema::rates_history;

use diesel_async::RunQueryDsl;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use deadpool::managed::{Pool, Object};
use my_logger::LogLevel;

use crate::common::logger::StrLogger;
use crate::storages::RatesFilter;

use self::schema::rates_history::dsl::*;

pub type PgPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type PgPooledConnection = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub fn get_connection_pool(conn_str: &str) -> PgPool {
    "Create new connection Pool".log("get_connection_pool", LogLevel::Debug);

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(conn_str);
    Pool::builder(manager).build().expect("Could not build connection pool")
}

pub async fn batch_insert(connection: &mut AsyncPgConnection, objs: &Vec<Rate>) {

    format!("Insert {} rates to DB", objs.len()).as_str().log(
        "batch_insert", 
        LogLevel::Debug);

    diesel::insert_into(rates_history)
        .values(objs)
        .execute(connection)
        .await
        .expect("Error saving new rates to DB");
}

pub async fn get_rates(connection: &mut AsyncPgConnection, filter: RatesFilter) -> Vec<Rate> {

    format!("Get data from storage by filter: {:?}", filter).as_str().log(
        "get_rates", 
        LogLevel::Debug);
    
    // query 
    let mut query = String::from("select id,datetime,bid,ask,open,close,price from rates_history");
    
    let mut where_str = String::new();
    // by id 
    if filter.id.is_some() {
        where_str.push_str(&format!(" id = '{}' ", filter.id.unwrap()));
    }
    // by date range
    if filter.date_range.is_some() {
        let dr = filter.date_range.unwrap();

        let from = NaiveDateTime::from_timestamp_millis(dr.from as i64).unwrap();
        let to = NaiveDateTime::from_timestamp_millis(dr.to as i64).unwrap();

        let dr_where = format!(" datetime between '{}' and '{}' ", from, to);
        if !where_str.is_empty() {
            where_str.push_str(" and ")
        }
        where_str.push_str(&dr_where);
    }

    if !where_str.is_empty() {
        query.push_str(" where ");
        query.push_str(&where_str);
    }
    else {
        query.push_str(" order by datetime desc limit 10"); // last values
    }

    //
    println!("SQL query ==> {}", query);
    let res: Vec<Rate> = diesel::sql_query(query)
        .load::<Rate>(connection)
        .await
        .expect("Error executing the SQL query");

    res
}