use serde::Deserialize;
use serde::Serialize;
use strum::AsRefStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "with_diesel",
    derive(diesel::AsExpression, diesel::FromSqlRow)
)]
#[cfg_attr(feature = "with_diesel", diesel(sql_type = diesel::sql_types::Text))]
#[derive(AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum BizType {
    /// Notification of non-address payment order status change to payment success PAY_SUCCESS,
    /// timeout, failure, or payment error, etc.
    Pay,
    /// Notification of refund order status change, refund success or failure.
    PayRefund,
    /// Notification of batch reward order status change.
    PayBatch,
    /// Notification of address payment order status change
    TransferAddress,
    /// Notification of delayed payment order processing for address payments
    ReceivedConvertDelayAddress,
    /// Notification of a payment order for revenue currency specified by the merchant.
    PayActually,
}


crate::enum_from_name!(BizType);
#[cfg(feature = "with_diesel")]
crate::enum_diesel_sql!(BizType);
