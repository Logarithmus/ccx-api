use super::order_book::OrderBookSnapshot;
use serde::{de::IgnoredAny, Deserialize};

/// Gate WebSocket API response
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WsResponse {
    /// Request timestamp in seconds
    pub time: i64,
    /// Request ID extracted from the client request payload if client request has one
    pub id: Option<i64>,
    /// Channel-dependent fields of the response
    #[serde(flatten)]
    pub inner: WsResponseInner,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "channel")]
#[cfg_attr(test, derive(PartialEq))]
pub enum WsResponseInner {
    #[serde(rename = "spot.pong")]
    Pong,
    #[serde(rename = "spot.order_book")]
    OrderBook(WsResponseEvent<OrderBookSnapshot>),
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum WsResult<T> {
    #[serde(rename = "error")]
    Err(WsErr),
    #[serde(rename = "result")]
    Ok(T),
}

impl<T> From<WsResult<T>> for Result<T, WsErr> {
    fn from(result: WsResult<T>) -> Self {
        match result {
            WsResult::Err(err) => Err(err),
            WsResult::Ok(ok) => Ok(ok),
        }
    }
}

/// Gate WebSocket API error
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WsErr {
    pub code: WsErrCode,
    pub message: String,
}

/// Represents error codes returned by the server.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum WsErrCode {
    /// Invalid request body format.
    InvalidRequestBody = 1,
    /// Invalid argument provided.
    InvalidArgument = 2,
    /// Server-side error happened.
    ServerError = 3,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase", tag = "event")]
#[cfg_attr(test, derive(PartialEq))]
pub enum WsResponseEvent<T> {
    Subscribe(WsResult<IgnoredAny>),
    Unsubscribe(WsResult<IgnoredAny>),
    Update(WsResult<T>),
}

#[cfg(test)]
mod tests {
    use super::WsResponseInner;
    use crate::websocket::{
        order_book::OrderBookSnapshot,
        response::{WsResponse, WsResponseEvent, WsResult},
    };
    use rust_decimal_macros::dec;
    use similar_asserts::assert_eq;

    #[test]
    fn deserialize_pong() {
        let json = r#"{
  "time": 1545404023,
  "channel": "spot.pong",
  "event": "",
  "error": null,
  "result": null
}"#;
        let expected = WsResponse::new(1545404023, WsResponseInner::Pong);
        assert_eq!(expected, serde_json::from_str(json).unwrap())
    }

    #[test]
    fn deserialize_order_book() {
        let json = r#"{
  "time": 1606295412,
  "time_ms": 1606295412213,
  "channel": "spot.order_book",
  "event": "update",
  "result": {
    "t": 1606295412123,
    "lastUpdateId": 48791820,
    "s": "BTC_USDT",
    "bids": [
      ["19079.55", "0.0195"],
      ["19079.07", "0.7341"],
      ["19076.23", "0.00011808"],
      ["19073.9", "0.105"],
      ["19068.83", "0.1009"]
    ],
    "asks": [
      ["19080.24", "0.1638"],
      ["19080.91", "0.1366"],
      ["19080.92", "0.01"],
      ["19081.29", "0.01"],
      ["19083.8", "0.097"]
    ]
  }
}"#;
        let expected = WsResponse::new(
            1606295412,
            WsResponseInner::OrderBook(WsResponseEvent::Update(WsResult::Ok(OrderBookSnapshot {
                update_time_ms: 1606295412123,
                last_update_id: 48791820,
                currency_pair: "BTC_USDT".into(),
                bids: vec![
                    (dec!(19079.55), dec!(0.0195)).into(),
                    (dec!(19079.07), dec!(0.7341)).into(),
                    (dec!(19076.23), dec!(0.00011808)).into(),
                    (dec!(19073.9), dec!(0.105)).into(),
                    (dec!(19068.83), dec!(0.1009)).into(),
                ],
                asks: vec![
                    (dec!(19080.24), dec!(0.1638)).into(),
                    (dec!(19080.91), dec!(0.1366)).into(),
                    (dec!(19080.92), dec!(0.01)).into(),
                    (dec!(19081.29), dec!(0.01)).into(),
                    (dec!(19083.8), dec!(0.097)).into(),
                ],
            }))),
        );
        let jd = &mut serde_json::Deserializer::from_str(json);
        assert_eq!(expected, serde_path_to_error::deserialize(jd).unwrap());
    }

    impl WsResponse {
        fn new(time: i64, inner: WsResponseInner) -> Self {
            Self {
                time,
                inner,
                id: None,
            }
        }
    }
}
