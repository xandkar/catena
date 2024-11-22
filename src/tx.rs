#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Tx {
    Msg(String),
    Coinbase {
        dst: String,
        amount: u64,
    },
    Pay {
        src: String,
        dst: String,
        amount: u64,
    },
    SetDifficulty(usize),
}
