#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, BytesN, Env,
};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    SoulAlreadyExists = 3,
    NotInitialized = 4,
    CounterOverflow = 5,
    SoulNotFound = 6,
    InvalidRecoverySignature = 7,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    PendingAdmin,
    Relayer,
    TotalSouls,
    SoulById(u32),
    SoulByPasskey(BytesN<65>),
    SoulByAddress(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SoulData {
    pub id: u32,
    pub owner: Address,
    pub passkey: BytesN<65>,
    pub recovery_pubkey: BytesN<65>,
    pub minted_at: u64,
    pub nonce: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintedEvent {
    pub soul_id: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryEvent {
    pub soul_id: u32,
    pub new_passkey: BytesN<65>,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerEvent {
    pub new_relayer: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdminTransferredEvent {
    pub old_admin: Address,
    pub new_admin: Address,
}


mod logic;
mod storage;

#[cfg(test)]
mod test;

#[contract]
pub struct ZolvencySoulContract;

#[contractimpl]
impl ZolvencySoulContract {
    pub fn initialize(env: Env, admin: Address, relayer: Address) -> Result<(), Error> {
        if storage::get_admin(&env).is_ok() {
            return Err(Error::AlreadyInitialized);
        }
        storage::set_admin(&env, &admin);
        storage::set_relayer(&env, &relayer);
        Ok(())
    }

    pub fn mint(
        env: Env,
        relayer: Address,
        owner: Address,
        passkey: BytesN<65>,
        recovery_pubkey: BytesN<65>,
    ) -> Result<u32, Error> {
        logic::mint(&env, relayer, owner, passkey, recovery_pubkey)
    }

    pub fn recover_soul(
        env: Env,
        relayer: Address,
        old_passkey: BytesN<65>,
        new_passkey: BytesN<65>,
        signature: BytesN<64>,
    ) -> Result<(), Error> {
        logic::recover_soul(&env, relayer, old_passkey, new_passkey, signature)
    }

    pub fn get_soul(env: Env, id: u32) -> Option<SoulData> {
        storage::get_soul_by_id(&env, id)
    }

    pub fn get_soul_id_by_address(env: Env, address: Address) -> Option<u32> {
        storage::get_soul_id_by_address(&env, &address)
    }

    pub fn get_soul_by_passkey(env: Env, passkey: BytesN<65>) -> Option<SoulData> {
        let id = storage::get_soul_id_by_passkey(&env, &passkey)?;
        storage::get_soul_by_id(&env, id)
    }

    pub fn get_soul_id_by_passkey(env: Env, passkey: BytesN<65>) -> Option<u32> {
        storage::get_soul_id_by_passkey(&env, &passkey)
    }

    pub fn admin(env: Env) -> Result<Address, Error> {
        storage::get_admin(&env)
    }

    pub fn relayer(env: Env) -> Result<Address, Error> {
        storage::get_relayer(&env)
    }

    pub fn total_souls(env: Env) -> u32 {
        storage::get_total_souls(&env)
    }

    pub fn has_soul(env: Env, address: Address) -> bool {
        storage::get_soul_id_by_address(&env, &address).is_some()
    }

    pub fn update_relayer(env: Env, admin: Address, new_relayer: Address) -> Result<(), Error> {
        logic::update_relayer(&env, admin, new_relayer)
    }

    pub fn renew_soul(env: Env, id: u32) -> Result<(), Error> {
        logic::renew_soul(&env, id)
    }

    pub fn transfer_admin(env: Env, admin: Address, new_admin: Address) -> Result<(), Error> {
        logic::transfer_admin(&env, admin, new_admin)
    }

    pub fn accept_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        logic::accept_admin(&env, new_admin)
    }

    pub fn upgrade(env: Env, admin: Address, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
        admin.require_auth();
        if admin != storage::get_admin(&env)? {
            return Err(Error::NotAuthorized);
        }
        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
