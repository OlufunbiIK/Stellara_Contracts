//! Centralised storage key definitions.
//!
//! Every persistent / instance / temporary key used across the contract
//! must be declared here to prevent accidental key collisions.

use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    // ── Global ──────────────────────────────────────────────────────
    Admin,
    Name,
    Symbol,
    Paused,

    // ── NFT ─────────────────────────────────────────────────────────
    /// Monotonically-increasing counter; equals the next token_id to mint.
    NftCounter,
    /// Owner of a specific NFT.                       key: token_id
    NftOwner(u64),
    /// Metadata URI of a specific NFT.                key: token_id
    NftUri(u64),
    /// Approved spender for a specific NFT.           key: token_id
    NftApproved(u64),
    /// Number of NFTs held by an address.             key: owner
    NftBalance(Address),

    // ── SFT ─────────────────────────────────────────────────────────
    /// Monotonically-increasing counter; equals the next class_id.
    SftClassCounter,
    /// Metadata URI for a class.                      key: class_id
    SftClassUri(u64),
    /// Display name for a class.                      key: class_id
    SftClassName(u64),
    /// Maximum supply allowed for a class.            key: class_id
    SftClassMaxSupply(u64),
    /// Total minted supply of a class.                key: class_id
    SftClassSupply(u64),
    /// Balance of (owner, class).                     key: (owner, class_id)
    SftBalance(Address, u64),

    // ── Extensions ──────────────────────────────────────────────────
    /// Whether whitelist mode is on.
    WhitelistEnabled,
    /// Membership in the whitelist.                   key: address
    Whitelisted(Address),
    /// Royalty receiver address.
    RoyaltyReceiver,
    /// Royalty in basis points (0-10 000).
    RoyaltyBasisPoints,
}