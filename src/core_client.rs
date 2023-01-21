use actix_web::dev::{Decompress, Payload};
use actix_web::{http::StatusCode, web::Bytes, HttpMessage, HttpResponse}; // note Decompress == Decoder
use awc::error::SendRequestError;
use error::CoreClientError;
use std::time::Duration;

pub struct CoreClientResponse {
    status_code: StatusCode,
    content_type: String,
    payload: Bytes,
}

impl Into<HttpResponse> for CoreClientResponse {
    fn into(self) -> HttpResponse {
        HttpResponse::build(self.status_code)
            .content_type(self.content_type)
            .body(self.payload)
    }
}

impl CoreClientResponse {
    pub fn is_success(&self) -> bool {
        self.status_code.is_success()
    }
}

///  Core storage client
pub struct CoreStorageClient {
    http_client: awc::Client,
    base_url: String,
}

impl CoreStorageClient {
    pub fn new(base_url: &str) -> Self {
        let http_client = awc::Client::builder()
            .timeout(Duration::from_secs(10))
            .add_default_header((awc::http::header::USER_AGENT, "wdms2prod/1.0"))
            .finish();
        CoreStorageClient {
            http_client,
            base_url: format!("{}/api/v2/storage", base_url),
        }
    }

    /// fetch a record given a record id
    pub async fn get_record(&self, r_id: String) -> Result<CoreClientResponse, CoreClientError> {
        let url = format!("{}/records/{}", self.base_url, r_id);
        let res = self.http_client.get(url).send().await;
        read_client_response(res).await
    }
}

async fn read_client_response(
    res: Result<awc::ClientResponse<Decompress<Payload>>, SendRequestError>,
) -> Result<CoreClientResponse, CoreClientError> {
    if res.is_err() {
        let err = res.err().unwrap();
        return Err(CoreClientError::SendError(err));
    }

    let mut response = res.unwrap();
    let status_code = response.status();
    let content_type = response.content_type().to_string();
    match response.body().await {
        Err(e) => Err(CoreClientError::PayloadError(e)),
        Ok(data) => Ok(CoreClientResponse {
            status_code,
            content_type,
            payload: data,
        }),
    }
}

pub mod error {
    use actix_web::error::ResponseError;
    use awc::error::{PayloadError, SendRequestError};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum CoreClientError {
        #[error("internal core client failure")]
        SendError(#[from] SendRequestError),

        #[error("internal core client failure")]
        PayloadError(#[from] PayloadError),
    }

    // implement trait ResponseError from actix web so it can automatically construct error
    // http response (500 by default). It requires to impl traits fmt::Debug + fmt::Display
    // which is provided by `thiserror` macros
    impl ResponseError for CoreClientError {}
}
