//! Pausable extension.
//!
//! When paused, all token transfers are blocked until the admin calls unpause.

use soroban_sdk::Env;

use crate::errors::TokenError;
use crate::events::TokenEvents;
use crate::storage_types::StorageKey;

pub struct PausableImpl;

impl PausableImpl {
    pub fn pause(env: &Env) {
        env.storage().instance().set(&StorageKey::Paused, &true);
        TokenEvents::paused(env);
    }

    pub fn unpause(env: &Env) {
        env.storage().instance().set(&StorageKey::Paused, &false);
        TokenEvents::unpaused(env);
    }

    pub fn is_paused(env: &Env) -> bool {
        env.storage()
            .instance()
            .get(&StorageKey::Paused)
            .unwrap_or(false)
    }
}

/// Convenience guard — panics with `TokenError::Paused` when transfers are paused.
pub fn require_not_paused(env: &Env) {
    if PausableImpl::is_paused(env) {
        panic!("{}", TokenError::Paused as u32);
    }
}