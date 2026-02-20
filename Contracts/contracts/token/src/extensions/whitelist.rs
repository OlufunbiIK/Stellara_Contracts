//! Whitelist extension.
//!
//! When enabled, only whitelisted addresses may receive token transfers.
//! Senders are not checked — only the recipient.

use soroban_sdk::{Address, Env};

use crate::errors::TokenError;
use crate::events::TokenEvents;
use crate::storage_types::StorageKey;

pub struct WhitelistImpl;

impl WhitelistImpl {
    pub fn enable(env: &Env) {
        env.storage()
            .instance()
            .set(&StorageKey::WhitelistEnabled, &true);
        TokenEvents::whitelist_changed(env, true);
    }

    pub fn disable(env: &Env) {
        env.storage()
            .instance()
            .set(&StorageKey::WhitelistEnabled, &false);
        TokenEvents::whitelist_changed(env, false);
    }

    pub fn add(env: &Env, addr: &Address) {
        env.storage()
            .persistent()
            .set(&StorageKey::Whitelisted(addr.clone()), &true);
        TokenEvents::whitelist_updated(env, addr, true);
    }

    pub fn remove(env: &Env, addr: &Address) {
        env.storage()
            .persistent()
            .remove(&StorageKey::Whitelisted(addr.clone()));
        TokenEvents::whitelist_updated(env, addr, false);
    }

    pub fn is_whitelisted(env: &Env, addr: &Address) -> bool {
        env.storage()
            .persistent()
            .get(&StorageKey::Whitelisted(addr.clone()))
            .unwrap_or(false)
    }
}

/// Whether the whitelist feature is currently active.
pub fn is_enabled(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&StorageKey::WhitelistEnabled)
        .unwrap_or(false)
}

/// Panic if the whitelist is enabled and `addr` is not on it.
pub fn require_whitelisted(env: &Env, addr: &Address) {
    if !WhitelistImpl::is_whitelisted(env, addr) {
        panic!("{}", TokenError::NotWhitelisted as u32);
    }
}