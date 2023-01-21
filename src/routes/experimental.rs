use actix_web::{get, HttpResponse, ResponseError};
use log::info;
use thiserror::Error;

#[derive(Error, Debug)]
enum ExpCustomError {
    #[error("core storage failure: {0}")]
    CoreStorage(#[from] std::io::Error),
    #[error("record invalid due to the following errors: {0}")]
    Validation(String),
    // #[error("unknown storage error")]
    // Unknown,
}

fn do_stuff() -> Result<(), ExpCustomError> {
    use std::fs::File;
    File::open("invali545445d")?;
    Ok(())
}

fn validate_record() -> Result<(), ExpCustomError> {
    do_stuff()?;
    Err(ExpCustomError::Validation("id cannot be null".to_string()))
}

impl ResponseError for ExpCustomError {}

#[get("/error")]
pub async fn get_error() -> Result<HttpResponse, actix_web::Error> {
    use std::fs::File;
    info!("stuff");
    validate_record()?;
    if File::open("invali545445d").is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().finish())
}
