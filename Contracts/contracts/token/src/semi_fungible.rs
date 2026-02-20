fn mint_sft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    to: String,
    amount: u128,
) -> StdResult<Response> {

    let config = CONFIG.load(deps.storage)?;
    if !config.features.enable_sft {
        return Err(StdError::generic_err("SFT disabled"));
    }

    let addr = deps.api.addr_validate(&to)?;
    let balance = SFT_BALANCES
        .may_load(deps.storage, (&token_id, &addr))?
        .unwrap_or(0);

    SFT_BALANCES.save(
        deps.storage,
        (&token_id, &addr),
        &(balance + amount),
    )?;

    Ok(Response::new())
}
