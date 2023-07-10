use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PairInfo {
    pub contractAddr: String,
    pub TokenInfo1: TokenInfo,
    pub TokenInfo2: TokenInfo,
}

pub enum TokenInfo {
    Token {
        contractAddr: String,
    },
}