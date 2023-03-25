use crate::extractors::{CaptureHeaders, CommonHeaders};
use crate::CoreStorageClient;
use actix_web::{http::StatusCode, web, HttpResponse};

use crate::model::record::Record;

pub async fn get_record(
    record_id: web::Path<String>,
    storage_client: web::Data<CoreStorageClient>,
    headers: CaptureHeaders<CommonHeaders>,
) -> Result<HttpResponse, actix_web::Error> {
    let r_id = record_id.into_inner();
    let response = storage_client.get_record(headers.into(), &r_id).await?;
    Record::from_slice(&response.payload)?;
    return Ok(HttpResponse::build(StatusCode::OK)
        .content_type(response.content_type)
        .body(response.payload));
}

// pub async fn get_record(
//     record_id: web::Path<String>,
//     storage_client: web::Data<CoreStorageClient>,
//     basic_headers: BasicHeaders,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let r_id = record_id.into_inner();
//     let response = storage_client
//         .get_record(
//             &basic_headers.data_partition,
//             &basic_headers.authorization,
//             &r_id,
//         )
//         .await?;
//     Record::from_slice(&response.payload)?;
//     return Ok(HttpResponse::build(StatusCode::OK)
//         .content_type(response.content_type)
//         .body(response.payload));
// }
