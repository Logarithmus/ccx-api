use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;
use crate::util::dt_gate::DtGate;
use crate::util::maybe_str;

#[derive(Debug, Clone)]
pub struct AllCurrenciesRequest;

impl Request for AllCurrenciesRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    const IS_PUBLIC: bool = true;
    type Response = Vec<Currency>;
}

#[derive(Debug, Clone)]
pub struct CurrencyRequest;

impl Request for CurrencyRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    const IS_PUBLIC: bool = true;
    type Response = Currency;
}

/// Represents the details of a currency.
#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    /// Currency name
    pub currency: SmartString,
    /// Whether currency is de-listed
    pub delisted: bool,
    /// Whether currency's withdrawal is disabled
    pub withdraw_disabled: bool,
    /// Whether currency's withdrawal is delayed
    pub withdraw_delayed: bool,
    /// Whether currency's deposit is disabled
    pub deposit_disabled: bool,
    /// Whether currency's trading is disabled
    pub trade_disabled: bool,
    /// Fixed fee rate. Only for fixed rate currencies, not valid for normal currencies
    pub fixed_rate: Option<SmartString>,
    /// Chain of currency
    pub chain: SmartString,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::api::spot::SpotApi;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;

    impl<S: GateSigner> SpotApi<S> {
        /// List all currencies' details
        ///
        /// `GET /spot/currencies`
        ///
        /// Currency has two forms:
        /// * Only currency name, e.g., `BTC`, `USDT`
        /// * `<currency>_<chain>`, e.g., `HT_ETH`
        ///
        /// ## Parameters
        /// None
        pub async fn all_currencies(&self) -> Result<AllCurrenciesRequest::Response, RequestError> {
            self.request("/spot/currencies", &AllCurrenciesRequest)
                .await
        }

        /// Get details of a specific currency
        ///
        /// `GET /spot/currencies/{name}`
        ///
        /// Get details of a specific currency
        pub async fn currency(
            &self,
            name: &str,
        ) -> Result<CurrencyRequest::Response, RequestError> {
            self.request(format!("/spot/currencies/{name}"), &CurrencyRequest)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn deserialize_currency() {
        let json = r#"{
     "currency": "GT",
     "delisted": false,
     "withdraw_disabled": false,
     "withdraw_delayed": false,
     "deposit_disabled": false,
     "trade_disabled": false,
     "chain": "GT"
  }"#;
        let expected = Currency {
            currency: "GT",
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: None,
            chain: "GT",
        };
        assert_eq!(serde_json::from_str(json).unwrap().as_slice(), expected);
    }
}
