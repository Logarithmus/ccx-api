use serde::Serialize;

use crate::client::ready::ReadyRequest;
use crate::client::signer::FinerySigner;
use crate::proto::SignedRequest;
use crate::types::timestamp::Timestamp;

use super::conversion::to_request_content;
use super::meta::FineryError;
use super::ready::SignData;
use super::signer::FinerySignerPayload;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct Stamped<T> {
    #[serde(flatten)]
    request: T,
    #[serde(flatten)]
    timestamp: Timestamp,
}

impl<T> Stamped<T> {
    pub fn new(request: T, timestamp: Timestamp) -> Self {
        Stamped { request, timestamp }
    }
}

impl<T> Stamped<T>
where
    T: SignedRequest + Send,
{
    #[tracing::instrument(skip_all)]
    pub async fn sign(self, signer: impl FinerySigner) -> Result<ReadyRequest<T>, FineryError> {
        let path = self.request.path();
        let content = to_request_content(&self.request)?;

        let signer_payload = FinerySignerPayload::builder()
            .method(T::HTTP_METHOD)
            .path(&path)
            .query(content.query.as_deref())
            .body(content.body.as_deref())
            .timestamp(self.timestamp)
            .build();

        let sign = signer.sign_request(signer_payload).await?;

        let sign_data = SignData::builder()
            .sign(sign)
            .timestamp(self.timestamp)
            .api_key(signer.api_key().to_string())
            .build();

        Ok(ReadyRequest::builder()
            .path(path.into())
            .query(content.query)
            .body(content.body)
            .sign_data(sign_data)
            .build())
    }
}
