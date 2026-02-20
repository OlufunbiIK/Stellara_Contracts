//! # Stellara Advanced Token Contract
//!
//! This contract extends the base fungible token (SEP-41) to support:
//! - **NFTs** (Non-Fungible Tokens): unique tokens with metadata & ownership
//! - **Semi-Fungible Tokens (SFT)**: ERC-1155-style token classes with supply
//! - **Custom Extensions**: pausable transfers, royalties, whitelisting
//!
//! ## Architecture
//!
//! ```
//! contracts/token/src/
//! ├── lib.rs                      ← this file (contract entry, routing)
//! ├── storage_types.rs            ← all StorageKey enums
//! ├── errors.rs                   ← contract error codes
//! ├── events.rs                   ← emitted events
//! ├── admin.rs                    ← admin / access control
//! ├── nft/
//! │   ├── mod.rs                  ← NFT module
//! │   ├── contract.rs             ← NFT contract trait impl
//! │   └── metadata.rs             ← NFT metadata helpers
//! ├── semi_fungible/
//! │   ├── mod.rs                  ← SFT module
//! │   └── contract.rs             ← SFT contract trait impl
//! └── extensions/
//!     ├── mod.rs                  ← extensions module
//!     ├── pausable.rs             ← pausable transfers extension
//!     ├── royalty.rs              ← royalty extension
//!     └── whitelist.rs            ← whitelist extension
//! ```

#![no_std]

mod admin;
mod errors;
mod events;
mod nft;
mod semi_fungible;
mod extensions;
mod storage_types;



use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec,
};

use nft::contract::NftImpl;
use semi_fungible::contract::SftImpl;
use extensions::pausable::PausableImpl;
use extensions::royalty::RoyaltyImpl;
use extensions::whitelist::WhitelistImpl;
use storage_types::StorageKey;
use errors::TokenError;
use events::TokenEvents;

// ─────────────────────────────────────────────────────────────────
// Contract struct
// ─────────────────────────────────────────────────────────────────

#[contract]
pub struct AdvancedTokenContract;

// ─────────────────────────────────────────────────────────────────
// Contract implementation
// ─────────────────────────────────────────────────────────────────

#[contractimpl]
impl AdvancedTokenContract {

    // ──────────────────────────────────────────
    // Initialisation
    // ──────────────────────────────────────────

    /// Initialise the contract.
    ///
    /// Must be called once immediately after deployment.
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
    ) {
        if env.storage().instance().has(&StorageKey::Admin) {
            panic!("already initialised");
        }
        env.storage().instance().set(&StorageKey::Admin, &admin);
        env.storage().instance().set(&StorageKey::Name, &name);
        env.storage().instance().set(&StorageKey::Symbol, &symbol);
        env.storage().instance().set(&StorageKey::Paused, &false);
        env.storage().instance().set(&StorageKey::NftCounter, &0u64);
        env.storage().instance().set(&StorageKey::SftClassCounter, &0u64);

        TokenEvents::initialized(&env, &admin, &name, &symbol);
    }

    // ──────────────────────────────────────────
    // Admin
    // ──────────────────────────────────────────

    pub fn get_admin(env: Env) -> Address {
        admin::require_admin(&env);
        env.storage().instance().get(&StorageKey::Admin).unwrap()
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        admin::require_admin(&env);
        env.storage().instance().set(&StorageKey::Admin, &new_admin);
        TokenEvents::admin_changed(&env, &new_admin);
    }

    // ──────────────────────────────────────────
    // NFT Interface
    // ──────────────────────────────────────────

    /// Mint a new NFT to `to` with a URI pointing to off-chain metadata.
    pub fn nft_mint(env: Env, to: Address, uri: String) -> u64 {
        admin::require_admin(&env);
        extensions::pausable::require_not_paused(&env);
        NftImpl::mint(&env, &to, &uri)
    }

    /// Transfer an NFT from `from` to `to`.
    pub fn nft_transfer(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();
        extensions::pausable::require_not_paused(&env);
        if extensions::whitelist::is_enabled(&env) {
            extensions::whitelist::require_whitelisted(&env, &to);
        }
        NftImpl::transfer(&env, &from, &to, token_id);
    }

    /// Approve a spender to manage a specific NFT.
    pub fn nft_approve(env: Env, owner: Address, approved: Address, token_id: u64) {
        owner.require_auth();
        NftImpl::approve(&env, &owner, &approved, token_id);
    }

    /// Transfer an NFT on behalf of the owner (requires prior approval).
    pub fn nft_transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u64) {
        spender.require_auth();
        extensions::pausable::require_not_paused(&env);
        NftImpl::transfer_from(&env, &spender, &from, &to, token_id);
    }

    /// Burn (destroy) an NFT.
    pub fn nft_burn(env: Env, from: Address, token_id: u64) {
        from.require_auth();
        NftImpl::burn(&env, &from, token_id);
    }

    /// Return the owner of an NFT.
    pub fn nft_owner_of(env: Env, token_id: u64) -> Address {
        NftImpl::owner_of(&env, token_id)
    }

    /// Return the metadata URI for an NFT.
    pub fn nft_token_uri(env: Env, token_id: u64) -> String {
        NftImpl::token_uri(&env, token_id)
    }

    /// Return how many NFTs `owner` holds.
    pub fn nft_balance_of(env: Env, owner: Address) -> u64 {
        NftImpl::balance_of(&env, &owner)
    }

    /// Return total number of NFTs minted.
    pub fn nft_total_supply(env: Env) -> u64 {
        NftImpl::total_supply(&env)
    }

    // ──────────────────────────────────────────
    // Semi-Fungible Token (SFT) Interface
    // ──────────────────────────────────────────

    /// Create a new SFT class, returning its class_id.
    pub fn sft_create_class(
        env: Env,
        name: String,
        uri: String,
        max_supply: u64,
    ) -> u64 {
        admin::require_admin(&env);
        SftImpl::create_class(&env, &name, &uri, max_supply)
    }

    /// Mint `amount` of `class_id` tokens to `to`.
    pub fn sft_mint(env: Env, to: Address, class_id: u64, amount: u64) {
        admin::require_admin(&env);
        extensions::pausable::require_not_paused(&env);
        SftImpl::mint(&env, &to, class_id, amount);
    }

    /// Transfer `amount` of `class_id` tokens from `from` to `to`.
    pub fn sft_transfer(env: Env, from: Address, to: Address, class_id: u64, amount: u64) {
        from.require_auth();
        extensions::pausable::require_not_paused(&env);
        if extensions::whitelist::is_enabled(&env) {
            extensions::whitelist::require_whitelisted(&env, &to);
        }
        SftImpl::transfer(&env, &from, &to, class_id, amount);
    }

    /// Batch-transfer multiple classes at once.
    pub fn sft_batch_transfer(
        env: Env,
        from: Address,
        to: Address,
        class_ids: Vec<u64>,
        amounts: Vec<u64>,
    ) {
        from.require_auth();
        extensions::pausable::require_not_paused(&env);
        SftImpl::batch_transfer(&env, &from, &to, &class_ids, &amounts);
    }

    /// Burn `amount` of `class_id` from `from`.
    pub fn sft_burn(env: Env, from: Address, class_id: u64, amount: u64) {
        from.require_auth();
        SftImpl::burn(&env, &from, class_id, amount);
    }

    /// Return the balance of `class_id` tokens for `owner`.
    pub fn sft_balance_of(env: Env, owner: Address, class_id: u64) -> u64 {
        SftImpl::balance_of(&env, &owner, class_id)
    }

    /// Return the total minted supply of a class.
    pub fn sft_class_supply(env: Env, class_id: u64) -> u64 {
        SftImpl::class_supply(&env, class_id)
    }

    /// Return the metadata URI for a class.
    pub fn sft_class_uri(env: Env, class_id: u64) -> String {
        SftImpl::class_uri(&env, class_id)
    }

    // ──────────────────────────────────────────
    // Extension: Pausable
    // ──────────────────────────────────────────

    /// Pause all token transfers.
    pub fn pause(env: Env) {
        admin::require_admin(&env);
        PausableImpl::pause(&env);
    }

    /// Resume token transfers.
    pub fn unpause(env: Env) {
        admin::require_admin(&env);
        PausableImpl::unpause(&env);
    }

    /// Return whether the contract is currently paused.
    pub fn is_paused(env: Env) -> bool {
        PausableImpl::is_paused(&env)
    }

    // ──────────────────────────────────────────
    // Extension: Royalty
    // ──────────────────────────────────────────

    /// Set the royalty receiver and basis-points (max 10 000 = 100 %).
    pub fn set_royalty(env: Env, receiver: Address, basis_points: u32) {
        admin::require_admin(&env);
        RoyaltyImpl::set_royalty(&env, &receiver, basis_points);
    }

    /// Return the royalty info: (receiver, basis_points).
    pub fn get_royalty(env: Env) -> (Address, u32) {
        RoyaltyImpl::get_royalty(&env)
    }

    /// Calculate the royalty amount for a given sale price.
    pub fn royalty_amount(env: Env, sale_price: u64) -> u64 {
        RoyaltyImpl::calculate(&env, sale_price)
    }

    // ──────────────────────────────────────────
    // Extension: Whitelist
    // ──────────────────────────────────────────

    /// Enable the transfer whitelist.
    pub fn enable_whitelist(env: Env) {
        admin::require_admin(&env);
        WhitelistImpl::enable(&env);
    }

    /// Disable the transfer whitelist.
    pub fn disable_whitelist(env: Env) {
        admin::require_admin(&env);
        WhitelistImpl::disable(&env);
    }

    /// Add an address to the whitelist.
    pub fn add_to_whitelist(env: Env, addr: Address) {
        admin::require_admin(&env);
        WhitelistImpl::add(&env, &addr);
    }

    /// Remove an address from the whitelist.
    pub fn remove_from_whitelist(env: Env, addr: Address) {
        admin::require_admin(&env);
        WhitelistImpl::remove(&env, &addr);
    }

    /// Check whether an address is whitelisted.
    pub fn is_whitelisted(env: Env, addr: Address) -> bool {
        WhitelistImpl::is_whitelisted(&env, &addr)
    }

    // ──────────────────────────────────────────
    // Metadata (shared)
    // ──────────────────────────────────────────

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&StorageKey::Name).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&StorageKey::Symbol).unwrap()
    }
}