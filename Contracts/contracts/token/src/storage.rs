use soroban_sdk::{contracttype, Address, String};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    Metadata,
    Admin,
    TotalSupply,
    Authorized(Address),

    // NFT
    NftOwner(u64),
    NftMetadata(u64),
    NftSupply,

    // SFT
    SftBalance(u64, Address),
}
