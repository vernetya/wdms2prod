use std::marker::PhantomData;

/// ho god, everything is always complex in Rust, just a "simple" middleware looks insane
/// as I don't understand yet how it's suppose to work
/// go another way using FromRequest trait instead
///
/// changes on the part needed ...
use actix_web::error::ErrorBadRequest;
use actix_web::{
    dev, http::header, http::header::HeaderMap, http::header::HeaderName, Error, FromRequest,
    HttpRequest,
};
use futures::future;

pub const DATA_PARTITION: HeaderName = HeaderName::from_static("data-partition-id");

pub trait ReprArray<T> {
    fn get_array() -> &'static [T];
}

pub struct CaptureHeaders<T: ReprArray<HeaderName>> {
    _map: HeaderMap,
    _marker: PhantomData<T>,
}

pub struct EnsureHeaders<T: ReprArray<HeaderName>> {
    _marker: PhantomData<T>,
}

impl<T: ReprArray<HeaderName>> Into<HeaderMap> for CaptureHeaders<T> {
    fn into(self) -> HeaderMap {
        self._map
    }
}

/// Generates an empty struct that implements trait `ReprArray<HeaderName>`. Uses as follow:
/// ``` ignore
/// DeclareHeaderList!(MyHeaderList, 2, header::ACCEPT, header::AUTHORIZATION)
/// ```
///
/// This expands to:
///
/// ``` ignore
///pub struct MyHeaderList;
///impl ReprArray<HeaderName> for MyHeaderList{
///    #[inline]
///   fn get_array() -> &'static [HeaderName] {
///        static INNER_LIST: [HeaderName; 2] = [header::ACCEPT, header::AUTHORIZATION];
///        &INNER_LIST
///    }
///}
/// ```
///
/// Note: size must be provided due to Rust macro system limitations
///
macro_rules! DeclareHeaderList {
    ($name:ident, $size:expr, [$( $x: expr ),+]) => {
        pub struct $name;
        impl ReprArray<HeaderName> for $name {
            #[inline]
            fn get_array() -> &'static [HeaderName] {
                static INNER_LIST: [HeaderName; $size] = [$($x),+];
                &INNER_LIST
            }
        }
    };
}

DeclareHeaderList!(CommonHeaders, 2, [DATA_PARTITION, header::AUTHORIZATION]);

impl<T: ReprArray<HeaderName>> From<&HeaderMap> for CaptureHeaders<T> {
    fn from(source_map: &HeaderMap) -> Self {
        let mut map = HeaderMap::with_capacity(T::get_array().len());
        for hdr in T::get_array() {
            if let Some(value) = source_map.get(hdr) {
                map.insert(hdr.into(), value.clone());
            }
        }
        CaptureHeaders {
            _map: map,
            _marker: PhantomData,
        }
    }
}
impl<T: ReprArray<HeaderName>> CaptureHeaders<T> {
    pub fn headers(&self) -> &HeaderMap {
        &self._map
    }
}

impl<T: ReprArray<HeaderName>> FromRequest for CaptureHeaders<T> {
    type Error = Error;
    type Future = future::Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        future::ready(Ok(req.headers().into()))
    }
}

impl<T: ReprArray<HeaderName>> FromRequest for EnsureHeaders<T> {
    type Error = Error;
    type Future = future::Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let source_map = req.headers();
        for hdr in T::get_array() {
            if !source_map.contains_key(hdr) {
                return future::ready(Err(ErrorBadRequest(format!("missing header: {}", hdr))));
            }
        }
        future::ready(Ok(EnsureHeaders {
            _marker: PhantomData,
        }))
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn toto() {
        let t = CommonHeaders::get_array().len();
        assert_eq!(t, 2);
    }

    #[test]
    fn test_capture_headers() {
        let mut headers = HeaderMap::new();
        headers.append(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("custom-value"),
        );
        headers.append(
            DATA_PARTITION,
            header::HeaderValue::from_static("partition"),
        );
        let captured: CaptureHeaders<CommonHeaders> = (&headers).into();
        assert_eq!(
            captured
                .headers()
                .get(DATA_PARTITION)
                .unwrap()
                .to_str()
                .unwrap(),
            "partition"
        );
        let header_map: HeaderMap = captured.into();
        assert!(!header_map.contains_key(header::CONTENT_TYPE));
        assert!(header_map.contains_key(DATA_PARTITION));
    }

    // #[test]
    // fn missing_header_should_err() {
    //     let mut headers = HeaderMap::new();
    //     headers.append(
    //         header::CONTENT_TYPE,
    //         header::HeaderValue::from_static("custom-value"),
    //     );
    //     let value = get_header_simple(&headers, "X-custom-header");
    //     assert!(value.is_err());
    // }
}
