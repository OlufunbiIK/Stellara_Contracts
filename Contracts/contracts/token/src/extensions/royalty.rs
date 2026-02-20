//! Royalty extension (EIP-2981 inspired).
//!
//! Stores a single (receiver, basis_points) pair applicable to all tokens.
//! Marketplaces should call `royalty_amount` at settlement time and forward
//! the result to the `receiver` address.
//!
//! Basis points: 100 bp = 1 %, 10 000 bp = 100 %.

use soroban_sdk::{Address, Env};

use crate::errors::TokenError;
use crate::events::TokenEvents;
use crate::storage_types::StorageKey;

pub struct RoyaltyImpl;

impl RoyaltyImpl {
    /// Set royalty parameters.  `basis_points` must be ≤ 10 000.
    pub fn set_royalty(env: &Env, receiver: &Address, basis_points: u32) {
        if basis_points > 10_000 {
            panic!("{}", TokenError::InvalidBasisPoints as u32);
        }
        env.storage()
            .instance()
            .set(&StorageKey::RoyaltyReceiver, receiver);
        env.storage()
            .instance()
            .set(&StorageKey::RoyaltyBasisPoints, &basis_points);
        TokenEvents::royalty_set(env, receiver, basis_points);
    }

    /// Return (receiver, basis_points).
    pub fn get_royalty(env: &Env) -> (Address, u32) {
        let receiver: Address = env
            .storage()
            .instance()
            .get(&StorageKey::RoyaltyReceiver)
            .unwrap_or_else(|| panic!("{}", TokenError::RoyaltyNotSet as u32));
        let bps: u32 = env
            .storage()
            .instance()
            .get(&StorageKey::RoyaltyBasisPoints)
            .unwrap_or(0u32);
        (receiver, bps)
    }

    /// Calculate the royalty amount for a given `sale_price`.
    pub fn calculate(env: &Env, sale_price: u64) -> u64 {
        let bps: u32 = env
            .storage()
            .instance()
            .get(&StorageKey::RoyaltyBasisPoints)
            .unwrap_or(0u32);
        (sale_price * bps as u64) / 10_000
    }
}