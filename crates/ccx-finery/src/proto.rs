use std::borrow::Cow;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client::FineryClient;
use crate::client::meta::FineryError;
use crate::client::ready::ReadyRequest;
use crate::client::signer::FinerySigner;
use crate::client::stamped::Stamped;
use crate::error::FineryResult;
use crate::rate_limiter::RateLimiterError;
use crate::rate_limiter::{RateLimitKey, RateLimiter};
use crate::types::rate_limits::RateLimitType;
use crate::types::timestamp::Timestamp;

pub trait Request: Serialize + Send + Sync {
    type Response: Response;

    const HTTP_METHOD: http::Method;
    /// IMPORTANT: endpoint should define the whole path i.e. starting from /api/v4
    const ENDPOINT: &'static str;

    fn path(&self) -> Cow<'static, str> {
        Self::ENDPOINT.into()
    }

    /// Rate limiter bucket type and score for this request.
    const COSTS: RateLimitType = RateLimitType::Public;

    fn costs(&self) -> &'static [(RateLimitKey, u32)] {
        &[((Self::COSTS, Self::ENDPOINT), 1)]
    }

    #[tracing::instrument(skip_all)]
    fn throttle(
        self,
        rate_limiter: &RateLimiter,
    ) -> impl Future<Output = Result<Self, RateLimiterError>> + Send
    where
        Self: Sized + Send,
    {
        let mut rate_limiter = rate_limiter.clone();
        async move {
            // TODO: based on finery.io docs the rate limits are applied to the
            // endpoints separately. Based on testing the limits are created
            // separately for the same endpoint with different path arguments
            // (like currency_pairs for different pairs)
            // but that will require dynamic allocations that
            // is currently not implemented by the base rate_limiter
            rate_limiter.enqueue(0, self.costs().into()).await?;
            Ok(self)
        }
    }
}

pub trait Response: DeserializeOwned + Send + Sync {}

impl<T> Response for Vec<T> where T: Response {}
impl<T, const N: usize> Response for smallvec::SmallVec<[T; N]> where T: Response {}

pub trait PublicRequest: Request {}

pub trait SignedRequest: Request + Send {
    fn stamp(self, timestamp: Timestamp) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, timestamp)
    }

    fn now(self) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, Timestamp::now())
    }

    #[tracing::instrument(skip_all)]
    fn sign_now(
        self,
        signer: impl FinerySigner,
    ) -> impl Future<Output = Result<ReadyRequest<Self>, FineryError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await }
    }

    #[tracing::instrument(skip_all)]
    fn sign_now_and_send(
        self,
        signer: impl FinerySigner,
        client: &FineryClient,
    ) -> impl Future<Output = FineryResult<Self::Response>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await?.send(client).await }
    }
}

pub trait RequestReadyToSend<T>
where
    T: Request,
{
    fn send(self, client: &FineryClient) -> impl Future<Output = FineryResult<T::Response>> + Send
    where
        Self: Sized + Send + Sync;
}
