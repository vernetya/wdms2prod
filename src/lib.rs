use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer, Result};
use std::net::TcpListener;

use env_logger;

pub mod config;
mod core_client;
mod routes;
use core_client::{CoreStorageClient, error};

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
            // logger tracing
            .wrap(Logger::default())
            // adding dependencies
            .app_data(web::Data::new(storage_client))
            // temporary fake core storage service
            .service(fake_core_storage::get_record)
            // actual endpoints
            .route(
                "/welllogs/{record_id}",
                web::get().to(routes::crud::get_record),
            )
            // probes
            .route("/health_z", web::get().to(routes::probes::health_check))
            .route("/liveness", web::get().to(routes::probes::health_check))
            // experimental
            .service(routes::experimental::get_error)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_me() {
        use std::fs::File;
        assert!(File::open("invali545445d").is_err());
    }
}
