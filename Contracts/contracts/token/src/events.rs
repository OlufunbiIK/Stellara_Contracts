//! Soroban event helpers.
//!
//! All events are emitted as structured topic/data pairs so they can be
//! indexed and filtered by off-chain tooling.

use soroban_sdk::{Address, Env, String, symbol_short};

pub struct TokenEvents;

impl TokenEvents {
    // ── Lifecycle ────────────────────────────────────────────────────

    pub fn initialized(env: &Env, admin: &Address, name: &String, symbol: &String) {
        env.events().publish(
            (symbol_short!("init"),),
            (admin.clone(), name.clone(), symbol.clone()),
        );
    }

    pub fn admin_changed(env: &Env, new_admin: &Address) {
        env.events().publish(
            (symbol_short!("admin"),),
            new_admin.clone(),
        );
    }

    // ── NFT events ───────────────────────────────────────────────────

    pub fn nft_minted(env: &Env, to: &Address, token_id: u64, uri: &String) {
        env.events().publish(
            (symbol_short!("nft_mint"), token_id),
            (to.clone(), uri.clone()),
        );
    }

    pub fn nft_transferred(env: &Env, from: &Address, to: &Address, token_id: u64) {
        env.events().publish(
            (symbol_short!("nft_xfr"), token_id),
            (from.clone(), to.clone()),
        );
    }

    pub fn nft_approved(env: &Env, owner: &Address, approved: &Address, token_id: u64) {
        env.events().publish(
            (symbol_short!("nft_appr"), token_id),
            (owner.clone(), approved.clone()),
        );
    }

    pub fn nft_burned(env: &Env, from: &Address, token_id: u64) {
        env.events().publish(
            (symbol_short!("nft_burn"), token_id),
            from.clone(),
        );
    }

    // ── SFT events ───────────────────────────────────────────────────

    pub fn sft_class_created(env: &Env, class_id: u64, name: &String, max_supply: u64) {
        env.events().publish(
            (symbol_short!("sft_cls"), class_id),
            (name.clone(), max_supply),
        );
    }

    pub fn sft_minted(env: &Env, to: &Address, class_id: u64, amount: u64) {
        env.events().publish(
            (symbol_short!("sft_mint"), class_id),
            (to.clone(), amount),
        );
    }

    pub fn sft_transferred(
        env: &Env,
        from: &Address,
        to: &Address,
        class_id: u64,
        amount: u64,
    ) {
        env.events().publish(
            (symbol_short!("sft_xfr"), class_id),
            (from.clone(), to.clone(), amount),
        );
    }

    pub fn sft_burned(env: &Env, from: &Address, class_id: u64, amount: u64) {
        env.events().publish(
            (symbol_short!("sft_burn"), class_id),
            (from.clone(), amount),
        );
    }

    // ── Extension events ─────────────────────────────────────────────

    pub fn paused(env: &Env) {
        env.events().publish((symbol_short!("paused"),), ());
    }

    pub fn unpaused(env: &Env) {
        env.events().publish((symbol_short!("unpaused"),), ());
    }

    pub fn royalty_set(env: &Env, receiver: &Address, basis_points: u32) {
        env.events().publish(
            (symbol_short!("royalty"),),
            (receiver.clone(), basis_points),
        );
    }

    pub fn whitelist_changed(env: &Env, enabled: bool) {
        env.events().publish((symbol_short!("wl_toggle"),), enabled);
    }

    pub fn whitelist_updated(env: &Env, addr: &Address, added: bool) {
        env.events().publish(
            (symbol_short!("wl_upd"),),
            (addr.clone(), added),
        );
    }
}