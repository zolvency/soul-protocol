use crate::storage;
use crate::{
    AdminTransferredEvent, DataKey, Error, MintedEvent, RecoveryEvent, RelayerEvent, SoulData,
};
use soroban_sdk::{Address, Bytes, BytesN, Env};

pub fn mint(
    env: &Env,
    relayer: Address,
    owner: Address,
    passkey: BytesN<65>,
    recovery_pubkey: BytesN<65>,
) -> Result<u32, Error> {
    relayer.require_auth();
    storage::extend_instance(env);

    let stored_relayer = storage::get_relayer(env)?;
    if relayer != stored_relayer {
        return Err(Error::NotAuthorized);
    }

    if storage::get_soul_id_by_address(env, &owner).is_some() {
        return Err(Error::SoulAlreadyExists);
    }

    if storage::get_soul_id_by_passkey(env, &passkey).is_some() {
        return Err(Error::SoulAlreadyExists);
    }

    let id = storage::increment_total_souls(env);
    let soul_data = SoulData {
        id,
        owner,
        passkey,
        recovery_pubkey,
        minted_at: env.ledger().timestamp(),
        nonce: 0,
    };

    storage::set_soul(env, &soul_data);

    MintedEvent { soul_id: id }.publish(env);

    Ok(id)
}

pub fn recover_soul(
    env: &Env,
    relayer: Address,
    old_passkey: BytesN<65>,
    new_passkey: BytesN<65>,
    recovery_signature: BytesN<64>,
) -> Result<(), Error> {
    relayer.require_auth();
    storage::extend_instance(env);

    let stored_relayer = storage::get_relayer(env)?;
    if relayer != stored_relayer {
        return Err(Error::NotAuthorized);
    }

    if old_passkey == new_passkey {
        return Err(Error::SoulAlreadyExists);
    }

    if storage::get_soul_id_by_passkey(env, &new_passkey).is_some() {
        return Err(Error::SoulAlreadyExists);
    }

    let soul_id = storage::get_soul_id_by_passkey(env, &old_passkey).ok_or(Error::SoulNotFound)?;
    let mut soul_data = storage::get_soul_by_id(env, soul_id).unwrap();

    let mut msg = Bytes::new(env);
    msg.append(&old_passkey.clone().into());
    msg.append(&new_passkey.clone().into());
    let nonce_bytes = soul_data.nonce.to_be_bytes();
    msg.append(&Bytes::from_array(env, &nonce_bytes));
    let msg_hash = env.crypto().sha256(&msg);

    env.crypto()
        .secp256r1_verify(&soul_data.recovery_pubkey, &msg_hash, &recovery_signature);

    storage::remove_passkey_mapping(env, &old_passkey);
    soul_data.passkey = new_passkey.clone();
    soul_data.nonce += 1;
    storage::set_soul(env, &soul_data);

    RecoveryEvent {
        soul_id,
        new_passkey: new_passkey.clone(),
    }
    .publish(env);

    Ok(())
}

pub fn update_relayer(env: &Env, admin: Address, new_relayer: Address) -> Result<(), Error> {
    storage::extend_instance(env);
    admin.require_auth();

    let stored_admin = storage::get_admin(env)?;
    if admin != stored_admin {
        return Err(Error::NotAuthorized);
    }

    storage::set_relayer(env, &new_relayer);

    RelayerEvent {
        new_relayer: new_relayer.clone(),
    }
    .publish(env);

    Ok(())
}

pub fn renew_soul(env: &Env, soul_id: u32) -> Result<(), Error> {
    let soul_data = storage::get_soul_by_id(env, soul_id).ok_or(Error::SoulNotFound)?;

    storage::extend_persistent(env, &DataKey::SoulById(soul_id));
    storage::extend_persistent(env, &DataKey::SoulByPasskey(soul_data.passkey));
    storage::extend_persistent(env, &DataKey::SoulByAddress(soul_data.owner));

    Ok(())
}

pub fn transfer_admin(env: &Env, admin: Address, new_admin: Address) -> Result<(), Error> {
    storage::extend_instance(env);
    admin.require_auth();

    let stored_admin = storage::get_admin(env)?;
    if admin != stored_admin {
        return Err(Error::NotAuthorized);
    }

    env.storage()
        .instance()
        .set(&DataKey::PendingAdmin, &new_admin);

    Ok(())
}

pub fn accept_admin(env: &Env, new_admin: Address) -> Result<(), Error> {
    storage::extend_instance(env);
    new_admin.require_auth();

    let pending_admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::PendingAdmin)
        .ok_or(Error::NotAuthorized)?;
    if new_admin != pending_admin {
        return Err(Error::NotAuthorized);
    }

    let old_admin = storage::get_admin(env)?;
    storage::set_admin(env, &new_admin);
    env.storage().instance().remove(&DataKey::PendingAdmin);

    AdminTransferredEvent {
        old_admin,
        new_admin: new_admin.clone(),
    }
    .publish(env);

    Ok(())
}
