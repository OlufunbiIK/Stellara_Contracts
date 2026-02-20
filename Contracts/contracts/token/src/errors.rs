//! Contract-wide error codes.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    // ── General ─────────────────────────────────
    NotInitialized       = 1,
    AlreadyInitialized   = 2,
    Unauthorized         = 3,
    Paused               = 4,

    // ── NFT ─────────────────────────────────────
    NftNotFound          = 100,
    NftNotOwner          = 101,
    NftNotApproved       = 102,

    // ── SFT ─────────────────────────────────────
    SftClassNotFound     = 200,
    SftInsufficientBalance = 201,
    SftMaxSupplyExceeded = 202,
    SftBatchLengthMismatch = 203,

    // ── Extensions ──────────────────────────────
    NotWhitelisted       = 300,
    InvalidBasisPoints   = 301,
    RoyaltyNotSet        = 302,
}