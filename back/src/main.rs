#![allow(dead_code)]

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use modules::{api::config, Service};
use utils::s3;

use crate::utils::database::PostgresRepository;

mod error;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();

    let repo = Arc::new(PostgresRepository::new().await);
    let bukcet_service = Arc::new(s3::S3Repository::new().await.unwrap());
    let service = Arc::new(Service::new(repo, bukcet_service));

    log::info!("Starting HTTP server on 0.0.0.0:4200...");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(web::scope("/api").configure(config))
            .app_data(web::Data::new(service.clone()))
    })
    .bind("0.0.0.0:4200")?
    .run()
    .await
}
