use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TokenStandard {
    Fungible,
    NonFungible,
    SemiFungible,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenFeatures {
    pub enable_ft: bool,
    pub enable_nft: bool,
    pub enable_sft: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub features: TokenFeatures,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// FT balances
pub const BALANCES: Map<&Addr, u128> = Map::new("balances");

/// NFT ownership: token_id -> owner
pub const NFT_OWNERS: Map<&str, Addr> = Map::new("nft_owners");

/// SFT balances: (token_id, owner) -> amount
pub const SFT_BALANCES: Map<(&str, &Addr), u128> = Map::new("sft_balances");
