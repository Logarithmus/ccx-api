use http::{HeaderValue, StatusCode};

use crate::api::error::FineryApiError;

pub type FineryError = ccx_lib::Error<FineryApiError>;

#[derive(Debug)]
pub struct FineryResponseWithMeta<T> {
    pub meta: FineryResponseMeta,
    pub payload: T,
}

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("{error}")]
pub struct FineryErrorWithMeta {
    pub meta: Option<FineryResponseMeta>,
    pub error: FineryError,
}

#[derive(Debug)]
pub struct FineryResponseMeta {
    pub http_status: StatusCode,
    pub trace_id: Option<String>,
    pub rate_limit: RateLimitMeta,
}

#[derive(Debug)]
pub struct RateLimitMeta {
    pub remain: u32,
    pub limit: u32,
}

impl<T> FineryResponseWithMeta<T> {
    pub fn new(payload: T, meta: FineryResponseMeta) -> Self {
        FineryResponseWithMeta { meta, payload }
    }

    pub fn into_parts(self) -> (FineryResponseMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> FineryResponseMeta {
        self.meta
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

fn parse_num(header: Option<&HeaderValue>) -> Option<u32> {
    header
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.parse().ok())
}

impl FineryResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();

        let trace_id = resp
            .headers()
            .get("x-finery-trace-id")
            .and_then(|h| h.to_str().ok())
            .map(ToString::to_string);

        let rate_limit = RateLimitMeta {
            remain: parse_num(resp.headers().get("x-finery-ratelimit-requests-remain"))
                .unwrap_or_default(),
            limit: parse_num(resp.headers().get("x-finery-ratelimit-limit")).unwrap_or_default(),
        };

        FineryResponseMeta {
            http_status,
            trace_id,
            rate_limit,
        }
    }

    pub fn error(self, error: impl Into<FineryError>) -> FineryErrorWithMeta {
        FineryErrorWithMeta {
            error: error.into(),
            meta: Some(self),
        }
    }

    pub fn response<T>(self, payload: T) -> FineryResponseWithMeta<T> {
        FineryResponseWithMeta {
            payload,
            meta: self,
        }
    }
}

impl<T> From<T> for FineryErrorWithMeta
where
    T: Into<FineryError>,
{
    fn from(error: T) -> Self {
        Self {
            error: error.into(),
            meta: None,
        }
    }
}
