use std::sync::Arc;

use tokio::task::JoinHandle;
use tonic::transport::Server;
use yh_services_sdk::yh_logger::Logger;

use crate::{endpoints::{rates_saver_grpc::RatesSaverSrvImpl, self, grpc::ratessaver::rates_saver_service_server::RatesSaverServiceServer}, app_context::AppContext};

pub fn run(ctx: Arc<AppContext>) -> JoinHandle<Result<(), tonic::transport::Error>> {
    Logger::log_info("Run gRPC service".to_owned(), "grpc-server-run".to_owned());
    let addr = "[::1]:8081".parse().unwrap();
    let rates_saver_service = RatesSaverSrvImpl::new(ctx.clone());

    println!("gRPC server listening on {}", addr);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(endpoints::grpc::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let server = Server::builder()
        .add_service(RatesSaverServiceServer::new(rates_saver_service))
        .add_service(reflection_service) 
        .serve(addr);
    
    tokio::spawn(server)
}