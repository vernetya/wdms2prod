use actix_web::{get, HttpResponse, ResponseError};
use log::info;
use thiserror::Error;

use crate::model::version;

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

    let _t = version::SemVerToken::Some(8);
    let _s = _t.to_string();
    let _pp: version::SemVer = "1.0.5".parse().expect("");
    //let _v: version::SemVer = "1.0.4".parse().expect();
    //let _v = version::SemVer::from_str("1.0.5").unwrap();
    info!("stuff");
    validate_record()?;
    if File::open("invali545445d").is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().finish())
}
