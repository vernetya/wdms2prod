use crate::error;
use actix_web::dev::{Decompress, Payload};
use actix_web::{http::header::HeaderMap, http::StatusCode, web::Bytes, HttpMessage}; // note Decompress == Decoder
use awc::error::{PayloadError, SendRequestError};
use awc::http::header;
use awc::ClientRequest;
use std::time::Duration;
use thiserror::Error;

pub struct CoreClientResponse {
    pub content_type: String,
    pub payload: Bytes,
}

///  Core storage client
pub struct CoreStorageClient {
    http_client: awc::Client,
    base_url: String,
}

const USER_AGENT_VALUE: header::HeaderValue = header::HeaderValue::from_static("wdms2prod/1.0");
const JSON_MIME: header::HeaderValue = header::HeaderValue::from_static("application/json");

impl CoreStorageClient {
    pub fn new(base_url: &str) -> Self {
        let http_client = awc::Client::builder()
            .timeout(Duration::from_secs(10))
            .add_default_header((awc::http::header::USER_AGENT, USER_AGENT_VALUE))
            .finish();
        CoreStorageClient {
            http_client,
            base_url: format!("{}/api/v2/storage", base_url),
        }
    }

    fn add_headers(rq: &mut ClientRequest, headers: HeaderMap) {
        let target_headers = rq.headers_mut();
        for (name, value) in headers {
            target_headers.append(name, value);
        }
    }

    /// fetch a record given a record id
    pub async fn get_record(
        &self,
        headers: HeaderMap,
        r_id: &str,
    ) -> Result<CoreClientResponse, error::WDMSError> {
        let url = format!("{}/records/{}", self.base_url, r_id);
        let mut request = self.http_client.get(url);
        Self::add_headers(&mut request, headers);
        let res = request
            .insert_header((header::ACCEPT, JSON_MIME))
            .send()
            .await
            .map_err(CoreStorageClientError::from)?;
        match read_client_response(res).await {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
    // pub async fn get_record(
    //     &self,
    //     data_partition: &str,
    //     token: &str,
    //     r_id: &str,
    // ) -> Result<CoreClientResponse, error::WDMSError> {
    //     let url = format!("{}/records/{}", self.base_url, r_id);
    //     let res = self
    //         .http_client
    //         .get(url)
    //         .append_header(("data-partition-id", data_partition))
    //         .insert_header((header::AUTHORIZATION, token))
    //         .insert_header((header::ACCEPT, "application/json"))
    //         .send()
    //         .await
    //         .map_err(CoreStorageClientError::from)?;
    //     match read_client_response(res).await {
    //         Ok(r) => Ok(r),
    //         Err(e) => Err(e.into()),
    //     }
    // }
}

impl From<CoreStorageClientError> for error::WDMSError {
    fn from(value: CoreStorageClientError) -> Self {
        let category = error::ErrorCategory::Dependency(error::Dependency::CoreStorage);
        let (code, description) = match &value {
            CoreStorageClientError::SendError(_) => (
                error::ErrorCode::SendRequest,
                "communication error".to_string(),
            ),
            CoreStorageClientError::PayloadError(_) => {
                (error::ErrorCode::Payload, "communication error".to_string())
            }
            CoreStorageClientError::ErrorStatusCode(c, d) => {
                (error::ErrorCode::ErrorStatus(c.as_u16()), d.clone())
            }
        };
        error::WDMSError {
            category,
            code,
            description,
            source: Some(Box::new(value)),
        }
    }
}

async fn read_client_response(
    mut response: awc::ClientResponse<Decompress<Payload>>,
) -> Result<CoreClientResponse, CoreStorageClientError> {
    let status_code = response.status();
    let content_type = response.content_type().to_string();
    match response.body().await {
        Err(e) => Err(CoreStorageClientError::PayloadError(e)),
        Ok(data) => {
            if status_code.is_success() {
                Ok(CoreClientResponse {
                    content_type,
                    payload: data,
                })
            } else {
                Err(CoreStorageClientError::ErrorStatusCode(
                    status_code,
                    String::from_utf8_lossy(&data).into_owned(),
                ))
            }
        }
    }
}

#[derive(Error, Debug)]
enum CoreStorageClientError {
    #[error("internal core client failure")]
    SendError(#[from] SendRequestError),

    #[error("internal core client failure")]
    PayloadError(#[from] PayloadError),

    #[error("error {0}: {1}")]
    ErrorStatusCode(StatusCode, String),
}
