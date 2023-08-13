use std::sync::Arc;

use actix_web::{HttpServer, App, get, web, HttpResponse};
use tokio::task::JoinHandle;

use crate::{app_context::{AppContext, Context},storages::RatesFilter};

const PORT: u16 = 8080; 

pub async fn run(ctx: Arc<AppContext>) -> JoinHandle<Result<(), std::io::Error>> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .service(rates)
    })
    .bind(("0.0.0.0", PORT))
    .expect("Error binding HTTP server to port")
    .run();
    
    tokio::spawn(server)
}

#[get("/rates")]
pub async fn rates(filter: web::Json<RatesFilter>, data: web::Data<Arc<AppContext>>) -> HttpResponse {

    let filter_inner = filter.into_inner();

    let reader = data.as_ref().get_storage_reader();
    let rates_list = reader.get_rates(filter_inner).await;

    HttpResponse::Ok().json(rates_list)

}