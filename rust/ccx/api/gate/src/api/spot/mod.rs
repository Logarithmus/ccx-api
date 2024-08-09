mod accounts;
mod currency;
mod currency_pair;
mod tickers;

pub use accounts::*;
pub use currency::*;
pub use currency_pair::*;
pub use tickers::*;

use super::GateApi;
use ref_cast::RefCast;

/// Spot trading
#[derive(RefCast, Clone)]
#[repr(transparent)]
pub struct SpotApi<S>(GateApi<S>);
