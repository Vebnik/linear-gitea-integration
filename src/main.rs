use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer, web};
use log::info;

mod services;
mod tests;
mod error;
mod router;
mod config;

use router::init_api_service;
use error::{Result, CustomError};
use config::{Config, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    // init vars
    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();
    info!("Inited enviroment variables");

    let config = Arc::new(envy::from_env::<Config>().unwrap());
    let bind_addr: (String, u16) = (config.host.clone(), config.port);
    info!("Inited enviroment config");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        info!("Inited CORS");

        info!("Create app instance");
        App::new()
            .app_data(web::Data::new(AppState { config: Arc::clone(&config) }))
            .wrap(Logger::default())
            .wrap(cors)
            .service(init_api_service(web::scope("/service")))
    })
    .workers(1)
    .bind(bind_addr)
    .map_err(|_| CustomError::UbError)?
    .run()
    .await
    .map_err(|_| CustomError::UbError)?;

    Ok(())
}

