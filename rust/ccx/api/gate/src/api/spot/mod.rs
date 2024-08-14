mod accounts;
mod currency;
mod currency_pair;
mod tickers;
mod order_book;

pub use accounts::*;
pub use currency::*;
pub use currency_pair::*;
pub use order_book::*;
use ref_cast::RefCast;
pub use tickers::*;

use super::GateApi;

/// Spot trading
#[derive(RefCast, Clone)]
#[repr(transparent)]
pub struct SpotApi<S>(GateApi<S>);
