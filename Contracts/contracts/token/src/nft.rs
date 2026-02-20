fn mint_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    to: String,
) -> StdResult<Response> {

    let config = CONFIG.load(deps.storage)?;
    if !config.features.enable_nft {
        return Err(StdError::generic_err("NFT disabled"));
    }

    if NFT_OWNERS.has(deps.storage, &token_id) {
        return Err(StdError::generic_err("Token exists"));
    }

    let addr = deps.api.addr_validate(&to)?;
    NFT_OWNERS.save(deps.storage, &token_id, &addr)?;

    Ok(Response::new())
}
