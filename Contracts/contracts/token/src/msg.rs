use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub enable_ft: bool,
    pub enable_nft: bool,
    pub enable_sft: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    // FT
    TransferFT { to: String, amount: u128 },
    MintFT { to: String, amount: u128 },

    // NFT
    MintNFT { token_id: String, to: String },
    TransferNFT { token_id: String, to: String },

    // SFT
    MintSFT { token_id: String, to: String, amount: u128 },
    TransferSFT { token_id: String, to: String, amount: u128 },
}
