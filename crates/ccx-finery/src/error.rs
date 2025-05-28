use crate::api::error::FineryApiError;
pub use crate::client::meta::{FineryErrorWithMeta, FineryResponseWithMeta};

impl ccx_lib::CcxApiError for FineryApiError {}

pub type FineryResult<T> = Result<FineryResponseWithMeta<T>, FineryErrorWithMeta>;
