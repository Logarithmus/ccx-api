#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateLimitType {
    Public,
    WalletWithdraw,
    WalletTransferOrBalance,
    WalletOther,
    SpotOrderCreateChange,
    SpotOrderCancel,
    SpotOther,
    Other,
}
