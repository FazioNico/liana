use liana::miniscript::bitcoin::OutPoint;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Menu {
    Home,
    Receive,
    PSBTs,
    Transactions,
    Settings,
    Coins,
    CreateSpendTx,
    Recovery,
    RefreshCoins(Vec<OutPoint>),
}
