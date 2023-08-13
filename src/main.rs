use std::{error::Error, sync::Arc};
use my_logger::LogLevel;
use yh_services_sdk::{ register_seq_logger, yh_logger::Logger };

use crate::{app_config::reader, common::logger::StrLogger, endpoints::grpc};

mod app_config;
mod storages;
mod models;
mod service_bus;
mod app_context;
mod app_constants;
mod common;
mod db;
mod endpoints;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>  {
    
    // read .ENV file if exists 
    dotenv::dotenv().ok();

    //
    const PROC_NAME: &str = "MAIN";
    println!("App Name: {}", app_config::APP_NAME);

    // read app configs
    println!("Read application settings");
    let app_settings = reader::load();

    // logger
    print!("Register Seq logger");
    register_seq_logger!(app_config::APP_NAME, app_settings);

    format!("App settings: {:?}", app_settings).log(PROC_NAME, LogLevel::Debug);

    // generate context 
    let type_of_storage = storages::PersistingType::Postgres;
    let ctx = Arc::new(
        app_context::AppContext::new(&app_settings)
            .build(&type_of_storage)
            .await);

    // service bus subscriber 
    //service_bus::subscribe_to_service_bus_topics(&app_settings.sb_host, ctx.clone()).await;

    // endpoints 
    let http_server = endpoints::http::server::run(ctx.clone()).await;
    let grpc_server = grpc::server::run(ctx.clone());

    http_server.await??;
    grpc_server.await??;

    //
    Ok(())

}
