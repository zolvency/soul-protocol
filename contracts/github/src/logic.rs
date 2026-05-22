use soroban_sdk::{token, Address, Env, IntoVal, Symbol};
use crate::storage;
use crate::{Config, Error, GithubData, MintParams, CrossChainParams, Tier};

pub fn mint(
    env: &Env,
    caller: Address,
    soul_id: u32,
    params: MintParams,
    cross_chain: Option<CrossChainParams>,
) -> Result<u64, Error> {
    caller.require_auth();

    let config = storage::get_config(env)?;



    let res = env.try_invoke_contract::<soroban_sdk::Val, soroban_sdk::Error>(
        &config.soul_contract,
        &Symbol::new(env, "get_soul"),
        soroban_sdk::vec![env, soul_id.into_val(env)],
    );

    match res {
        Ok(Ok(val)) => {
            if val.is_void() {
                return Err(Error::Unauthorized);
            }
        },
        _ => return Err(Error::Unauthorized),
    }

    if storage::has_identity(env, soul_id) {
        return Err(Error::AlreadyHasIdentity);
    }

    if storage::get_sybil_token(env, &params.external_id).is_some() {
        return Err(Error::SybilConflict);
    }

    let expected_nonce = storage::get_nonce(env, soul_id);
    if params.nonce != expected_nonce {
        return Err(Error::InvalidNonce);
    }

    if config.mint_fee > 0 {
        let token_client = token::Client::new(env, &config.fee_token);
        token_client.transfer(&caller, &config.treasury, &config.mint_fee);
    }

    storage::increment_nonce(env, soul_id);

    let token_id = storage::get_next_token_id(env);
    storage::increment_token_counter(env);

    let now = env.ledger().timestamp();
    let data = GithubData {
        soul_id,
        username: params.username,
        external_id: params.external_id.clone(),
        contributions: params.contributions,
        tier: Tier::from_contributions(params.contributions),
        minted_at: now,
        updated_at: now,
        expires_at: 0, // SBT
    };

    storage::set_token_data(env, token_id, &data);
    storage::set_holder_token(env, soul_id, token_id);
    storage::set_has_identity(env, soul_id, true);
    storage::set_sybil_mapping(env, &params.external_id, token_id);



    env.events().publish(
        (Symbol::new(env, "GithubMinted"), soul_id),
        (token_id, data.contributions)
    );

    Ok(token_id)
}

pub fn upgrade(env: Env, admin: Address, new_wasm_hash: soroban_sdk::BytesN<32>) -> Result<(), Error> {
    admin.require_auth();
    let config = storage::get_config(&env)?;
    if admin != config.admin {
        return Err(Error::NotAdmin);
    }
    env.deployer().update_current_contract_wasm(new_wasm_hash);
    Ok(())
}
