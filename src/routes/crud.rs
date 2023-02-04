use crate::CoreStorageClient;
use actix_web::{http::StatusCode, web, HttpResponse};

use crate::model::record::{Record, RecordError};

pub async fn get_record(
    record_id: web::Path<String>,
    storage_client: web::Data<CoreStorageClient>,
) -> Result<HttpResponse, actix_web::Error> {
    let r_id = record_id.into_inner();
    let response = storage_client.get_record(r_id).await?;

    if response.status_code.is_success() {
        if let Err(e) = Record::from_slice(&response.payload) {
            let msg = match e {
                RecordError::Parsing(_) => String::from("invalid format"),
                RecordError::Validation(errors) => errors.join(", "),
                RecordError::UnknownKind(kind) => {
                    format!("kind {:?} is not a known wellbore related entity", kind)
                }
            };
            return Ok(HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
                .content_type("text/plain")
                .body(msg));
        }
    }

    Ok(response.into())
}
