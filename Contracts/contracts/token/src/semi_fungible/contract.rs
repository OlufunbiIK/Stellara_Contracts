//! Semi-Fungible Token (SFT) logic.
//!
//! Inspired by ERC-1155 but adapted for Soroban's storage model.
//!
//! ## Concepts
//! - A **class** represents a token type (e.g. "Gold Sword", "Event Ticket #A").
//! - Each class has a `max_supply` cap, a display `name`, and a metadata `uri`.
//! - Multiple holders can own balances of the same class.
//! - A `batch_transfer` lets callers move multiple classes in one transaction.

use soroban_sdk::{Address, Env, String, Vec};

use crate::errors::TokenError;
use crate::events::TokenEvents;
use crate::storage_types::StorageKey;

pub struct SftImpl;

impl SftImpl {
    // ─── Class management ──────────────────────────────────────────────────

    /// Create a new token class and return its `class_id`.
    pub fn create_class(
        env: &Env,
        name: &String,
        uri: &String,
        max_supply: u64,
    ) -> u64 {
        let class_id: u64 = env
            .storage()
            .instance()
            .get(&StorageKey::SftClassCounter)
            .unwrap_or(0u64);

        env.storage()
            .persistent()
            .set(&StorageKey::SftClassName(class_id), name);
        env.storage()
            .persistent()
            .set(&StorageKey::SftClassUri(class_id), uri);
        env.storage()
            .persistent()
            .set(&StorageKey::SftClassMaxSupply(class_id), &max_supply);
        env.storage()
            .persistent()
            .set(&StorageKey::SftClassSupply(class_id), &0u64);

        env.storage()
            .instance()
            .set(&StorageKey::SftClassCounter, &(class_id + 1));

        TokenEvents::sft_class_created(env, class_id, name, max_supply);
        class_id
    }

    // ─── Mint ──────────────────────────────────────────────────────────────

    pub fn mint(env: &Env, to: &Address, class_id: u64, amount: u64) {
        Self::require_class_exists(env, class_id);

        let current_supply: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::SftClassSupply(class_id))
            .unwrap_or(0u64);
        let max_supply: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::SftClassMaxSupply(class_id))
            .unwrap_or(u64::MAX);

        if max_supply > 0 && current_supply + amount > max_supply {
            panic!("{}", TokenError::SftMaxSupplyExceeded as u32);
        }

        // Update supply
        env.storage()
            .persistent()
            .set(&StorageKey::SftClassSupply(class_id), &(current_supply + amount));

        // Update holder balance
        let balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::SftBalance(to.clone(), class_id))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::SftBalance(to.clone(), class_id), &(balance + amount));

        TokenEvents::sft_minted(env, to, class_id, amount);
    }

    // ─── Transfer ──────────────────────────────────────────────────────────

    pub fn transfer(
        env: &Env,
        from: &Address,
        to: &Address,
        class_id: u64,
        amount: u64,
    ) {
        Self::require_class_exists(env, class_id);
        Self::deduct_balance(env, from, class_id, amount);
        Self::add_balance(env, to, class_id, amount);
        TokenEvents::sft_transferred(env, from, to, class_id, amount);
    }

    /// Batch-transfer multiple classes in one call.
    pub fn batch_transfer(
        env: &Env,
        from: &Address,
        to: &Address,
        class_ids: &Vec<u64>,
        amounts: &Vec<u64>,
    ) {
        if class_ids.len() != amounts.len() {
            panic!("{}", TokenError::SftBatchLengthMismatch as u32);
        }
        for i in 0..class_ids.len() {
            let class_id = class_ids.get(i).unwrap();
            let amount = amounts.get(i).unwrap();
            Self::require_class_exists(env, class_id);
            Self::deduct_balance(env, from, class_id, amount);
            Self::add_balance(env, to, class_id, amount);
            TokenEvents::sft_transferred(env, from, to, class_id, amount);
        }
    }

    // ─── Burn ──────────────────────────────────────────────────────────────

    pub fn burn(env: &Env, from: &Address, class_id: u64, amount: u64) {
        Self::require_class_exists(env, class_id);
        Self::deduct_balance(env, from, class_id, amount);

        let current_supply: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::SftClassSupply(class_id))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::SftClassSupply(class_id), &(current_supply.saturating_sub(amount)));

        TokenEvents::sft_burned(env, from, class_id, amount);
    }

    // ─── Queries ───────────────────────────────────────────────────────────

    pub fn balance_of(env: &Env, owner: &Address, class_id: u64) -> u64 {
        env.storage()
            .persistent()
            .get(&StorageKey::SftBalance(owner.clone(), class_id))
            .unwrap_or(0u64)
    }

    pub fn class_supply(env: &Env, class_id: u64) -> u64 {
        Self::require_class_exists(env, class_id);
        env.storage()
            .persistent()
            .get(&StorageKey::SftClassSupply(class_id))
            .unwrap_or(0u64)
    }

    pub fn class_uri(env: &Env, class_id: u64) -> String {
        Self::require_class_exists(env, class_id);
        env.storage()
            .persistent()
            .get(&StorageKey::SftClassUri(class_id))
            .unwrap_or_else(|| panic!("{}", TokenError::SftClassNotFound as u32))
    }

    // ─── Internal helpers ──────────────────────────────────────────────────

    fn require_class_exists(env: &Env, class_id: u64) {
        if !env.storage().persistent().has(&StorageKey::SftClassUri(class_id)) {
            panic!("{}", TokenError::SftClassNotFound as u32);
        }
    }

    fn deduct_balance(env: &Env, from: &Address, class_id: u64, amount: u64) {
        let balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::SftBalance(from.clone(), class_id))
            .unwrap_or(0u64);
        if balance < amount {
            panic!("{}", TokenError::SftInsufficientBalance as u32);
        }
        env.storage()
            .persistent()
            .set(&StorageKey::SftBalance(from.clone(), class_id), &(balance - amount));
    }

    fn add_balance(env: &Env, to: &Address, class_id: u64, amount: u64) {
        let balance: u64 = env
            .storage()
            .persistent()
            .get(&StorageKey::SftBalance(to.clone(), class_id))
            .unwrap_or(0u64);
        env.storage()
            .persistent()
            .set(&StorageKey::SftBalance(to.clone(), class_id), &(balance + amount));
    }
}