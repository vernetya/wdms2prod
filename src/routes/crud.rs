use crate::CoreStorageClient;
use actix_web::{web, HttpResponse};

pub async fn get_record(
    record_id: web::Path<String>,
    storage_client: web::Data<CoreStorageClient>,
) -> Result<HttpResponse, actix_web::Error> {
    let r_id = record_id.into_inner();
    let response = storage_client.get_record(r_id).await?;

    Ok(response.into())
}
