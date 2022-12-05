use actix_web::middleware::Logger;
use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, ResponseError, Result};
use std::net::TcpListener;
use thiserror::Error;

use env_logger;
use log::info;

pub mod config;
mod core_client;
use core_client::{error::CoreClientError, CoreStorageClient};

mod fake_core_storage;

pub fn build_and_start_server(
    listener: TcpListener,
    conf: Option<config::Config>,
) -> Result<Server, std::io::Error> {
    // if not specific provided, let's
    let cfg = conf.or(Some(config::load_config())).unwrap();
    if cfg.enable_logging {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    }
    let server = HttpServer::new(move || {
        let storage_client = CoreStorageClient::new(&cfg.host_core_storage);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(storage_client))
            .service(fake_core_storage::get_record)
            .service(get_well_log)
            // probes
            .route("/health_z", web::get().to(health_check))
            .route("/liveness", web::get().to(health_check))
            // testing
            .route("/error", web::get().to(get_error))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Error, Debug)]
pub enum RecordStoreError {
    #[error("core storage failure: {0}")]
    CoreStorage(#[from] std::io::Error),
    #[error("record invalid due to the following errors: {0}")]
    Validation(String),
    #[error("unknown storage error")]
    Unknown,
}

fn do_stuff() -> Result<(), RecordStoreError> {
    use std::fs::File;
    File::open("invali545445d")?;
    Ok(())
}

fn validate_record() -> Result<(), RecordStoreError> {
    do_stuff()?;
    Err(RecordStoreError::Validation(
        "id cannot be null".to_string(),
    ))
}

impl ResponseError for RecordStoreError {}

async fn get_error() -> Result<HttpResponse, actix_web::Error> {
    use std::fs::File;
    info!("stuff");
    validate_record()?;
    if File::open("invali545445d").is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().finish())
}

#[get("/welllog/{record_id}")]
async fn get_well_log(
    record_id: web::Path<String>,
    storage_client: web::Data<CoreStorageClient>,
) -> Result<HttpResponse, CoreClientError> {
    let r_id = record_id.into_inner();
    let response = storage_client.get_record(r_id).await?;

    Ok(response.into())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_me() {
        use std::fs::File;
        assert!(File::open("invali545445d").is_err());
    }
}
