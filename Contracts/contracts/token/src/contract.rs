//! NFT core logic.
//!
//! Each token is identified by a monotonically-increasing u64 `token_id`.
//! Token metadata is stored as a URI string pointing to off-chain JSON.

use soroban_sdk::{Address, Env, String};

use crate::errors::TokenError;
use crate::events::TokenEvents;
use crate::storage_types::StorageKey;

pub struct NftImpl;

impl NftImpl {
    // ─── Mint ──────────────────────────────────────────────────────────────

    /// Mint a new NFT, returns the new `token_id`.
    pub fn mint(env: &Env, to: &Address, uri: &String) -> u64 {
        let token_id: u64 = env
            .storage()
            .instance()
            .get(&StorageKey::NftCounter)
            .unwrap_or(0u64);

        // Store ownership & URI
        env.storage()
            .persistent()
            .set(&StorageKey::NftOwner(token_id), to);
        env.storage()
            .persistent()
            .set(&StorageKey::NftUri(token_id), uri);

        // Increment owner balance
        let balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::NftBalance(to.clone()))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::NftBalance(to.clone()), &(balance + 1));

        // Advance counter
        env.storage()
            .instance()
            .set(&StorageKey::NftCounter, &(token_id + 1));

        TokenEvents::nft_minted(env, to, token_id, uri);
        token_id
    }

    // ─── Transfer ──────────────────────────────────────────────────────────

    /// Transfer an NFT; caller must be the owner.
    pub fn transfer(env: &Env, from: &Address, to: &Address, token_id: u64) {
        let owner = Self::require_owner(env, token_id);
        if owner != *from {
            panic!("{}", TokenError::NftNotOwner as u32);
        }
        Self::do_transfer(env, from, to, token_id);
    }

    /// Transfer an NFT on behalf of the owner (approved spender or operator).
    pub fn transfer_from(
        env: &Env,
        spender: &Address,
        from: &Address,
        to: &Address,
        token_id: u64,
    ) {
        let owner = Self::require_owner(env, token_id);
        if owner != *from {
            panic!("{}", TokenError::NftNotOwner as u32);
        }
        // Verify spender is approved for this token
        let approved: Option<Address> = env
            .storage()
            .temporary()
            .get(&StorageKey::NftApproved(token_id));
        match approved {
            Some(a) if a == *spender => {}
            _ => panic!("{}", TokenError::NftNotApproved as u32),
        }
        // Clear approval
        env.storage()
            .temporary()
            .remove(&StorageKey::NftApproved(token_id));
        Self::do_transfer(env, from, to, token_id);
    }

    fn do_transfer(env: &Env, from: &Address, to: &Address, token_id: u64) {
        // Update owner
        env.storage()
            .persistent()
            .set(&StorageKey::NftOwner(token_id), to);

        // Decrement sender balance
        let from_balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::NftBalance(from.clone()))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::NftBalance(from.clone()), &(from_balance.saturating_sub(1)));

        // Increment receiver balance
        let to_balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::NftBalance(to.clone()))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::NftBalance(to.clone()), &(to_balance + 1));

        TokenEvents::nft_transferred(env, from, to, token_id);
    }

    // ─── Approve ───────────────────────────────────────────────────────────

    pub fn approve(env: &Env, owner: &Address, approved: &Address, token_id: u64) {
        let actual_owner = Self::require_owner(env, token_id);
        if actual_owner != *owner {
            panic!("{}", TokenError::NftNotOwner as u32);
        }
        env.storage()
            .temporary()
            .set(&StorageKey::NftApproved(token_id), approved);
        TokenEvents::nft_approved(env, owner, approved, token_id);
    }

    // ─── Burn ──────────────────────────────────────────────────────────────

    pub fn burn(env: &Env, from: &Address, token_id: u64) {
        let owner = Self::require_owner(env, token_id);
        if owner != *from {
            panic!("{}", TokenError::NftNotOwner as u32);
        }
        env.storage()
            .persistent()
            .remove(&StorageKey::NftOwner(token_id));
        env.storage()
            .persistent()
            .remove(&StorageKey::NftUri(token_id));
        env.storage()
            .temporary()
            .remove(&StorageKey::NftApproved(token_id));

        let balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::NftBalance(from.clone()))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::NftBalance(from.clone()), &(balance.saturating_sub(1)));

        TokenEvents::nft_burned(env, from, token_id);
    }

    // ─── Queries ───────────────────────────────────────────────────────────

    pub fn owner_of(env: &Env, token_id: u64) -> Address {
        Self::require_owner(env, token_id)
    }

    pub fn token_uri(env: &Env, token_id: u64) -> String {
        env.storage()
            .persistent()
            .get(&StorageKey::NftUri(token_id))
            .unwrap_or_else(|| panic!("{}", TokenError::NftNotFound as u32))
    }

    pub fn balance_of(env: &Env, owner: &Address) -> u64 {
        env.storage()
            .persistent()
            .get(&StorageKey::NftBalance(owner.clone()))
            .unwrap_or(0u64)
    }

    pub fn total_supply(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&StorageKey::NftCounter)
            .unwrap_or(0u64)
    }

    // ─── Internal ──────────────────────────────────────────────────────────

    fn require_owner(env: &Env, token_id: u64) -> Address {
        env.storage()
            .persistent()
            .get(&StorageKey::NftOwner(token_id))
            .unwrap_or_else(|| panic!("{}", TokenError::NftNotFound as u32))
    }
}