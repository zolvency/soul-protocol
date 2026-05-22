#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{BytesN, Env};

#[test]
fn test_soul_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    assert_eq!(client.admin(), admin);
    assert_eq!(client.relayer(), relayer);
    assert_eq!(client.total_souls(), 0);

    let passkey = BytesN::from_array(&env, &[0u8; 65]);
    let recovery_pubkey = BytesN::from_array(&env, &[1u8; 65]);

    let owner = Address::generate(&env);
    let id = client.mint(&relayer, &owner, &passkey, &recovery_pubkey);

    assert_eq!(id, 1);
    assert_eq!(client.total_souls(), 1);

    let soul = client.get_soul(&id).unwrap();
    assert_eq!(soul.passkey, passkey);
    assert_eq!(soul.recovery_pubkey, recovery_pubkey);
    assert_eq!(soul.id, 1);

    assert_eq!(client.get_soul_id_by_passkey(&passkey), Some(1));
}

#[test]
fn test_get_soul_by_passkey() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    let passkey = BytesN::from_array(&env, &[1u8; 65]);
    let recovery_pubkey = BytesN::from_array(&env, &[2u8; 65]);

    let owner = Address::generate(&env);
    client.mint(&relayer, &owner, &passkey, &recovery_pubkey);

    let soul = client.get_soul_by_passkey(&passkey).unwrap();
    assert_eq!(soul.passkey, passkey);
    assert_eq!(soul.id, 1);
}

#[test]
#[should_panic]
fn test_recover_soul_invalid_signature() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    let old_passkey = BytesN::from_array(&env, &[1u8; 65]);
    let recovery_pubkey = BytesN::from_array(&env, &[2u8; 65]);
    let new_passkey = BytesN::from_array(&env, &[3u8; 65]);
    let dummy_sig = BytesN::from_array(&env, &[0u8; 64]);

    let owner = Address::generate(&env);
    client.mint(&relayer, &owner, &old_passkey, &recovery_pubkey);

    client.recover_soul(&relayer, &old_passkey, &new_passkey, &dummy_sig);
}

#[test]
fn test_recover_soul_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    let recovery_pubkey = BytesN::from_array(
        &env,
        &[
            0x4, 0x8c, 0xf1, 0xa4, 0x2c, 0xd5, 0x2c, 0xe5, 0x71, 0x9f, 0x0, 0x36, 0xda, 0x37, 0x6a,
            0x1b, 0x99, 0xf6, 0x10, 0x77, 0x4a, 0x10, 0x64, 0xb4, 0x5d, 0xa9, 0xef, 0xdf, 0x65,
            0xa9, 0x67, 0x59, 0xa1, 0x14, 0x2e, 0xed, 0xf1, 0x85, 0x45, 0xfc, 0x77, 0x2d, 0x9b,
            0xfb, 0x6e, 0x5e, 0x8, 0xc6, 0xa6, 0xa0, 0x4e, 0xd6, 0xac, 0xe0, 0x47, 0xc6, 0xa4,
            0xcb, 0xbf, 0x46, 0x26, 0x95, 0x6a, 0xe, 0xb1,
        ],
    );

    let old_passkey = BytesN::from_array(&env, &[0u8; 65]);
    let new_passkey = BytesN::from_array(&env, &[1u8; 65]);

    let signature = BytesN::from_array(
        &env,
        &[
            0xd9, 0x9a, 0x85, 0x9b, 0x4f, 0x4a, 0x72, 0xb8, 0xb4, 0x25, 0xa1, 0x27, 0x97, 0x92,
            0x47, 0x4d, 0x6, 0xa3, 0xaa, 0x27, 0x82, 0x92, 0xc0, 0x9c, 0xd, 0x64, 0x6c, 0x30, 0xb7,
            0xe9, 0xdf, 0xbc, 0x7b, 0x58, 0x71, 0xfe, 0x3c, 0xec, 0x40, 0x16, 0x18, 0x53, 0x54,
            0xb9, 0x64, 0xf, 0xbd, 0xae, 0xea, 0xe5, 0x9d, 0x5c, 0x63, 0x76, 0x59, 0x14, 0x4c,
            0x15, 0xc2, 0xee, 0x43, 0xf6, 0xe9, 0x16,
        ],
    );

    let owner = Address::generate(&env);
    let id = client.mint(&relayer, &owner, &old_passkey, &recovery_pubkey);
    assert_eq!(id, 1);

    client.recover_soul(&relayer, &old_passkey, &new_passkey, &signature);

    let soul = client.get_soul(&id).unwrap();
    assert_eq!(soul.passkey, new_passkey);

    assert_eq!(client.get_soul_id_by_passkey(&old_passkey), None);
    assert_eq!(client.get_soul_id_by_passkey(&new_passkey), Some(1));
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_recover_soul_unauthorized_relayer() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);
    let wrong_relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    let old_passkey = BytesN::from_array(&env, &[1u8; 65]);
    let recovery_pubkey = BytesN::from_array(&env, &[2u8; 65]);
    let new_passkey = BytesN::from_array(&env, &[3u8; 65]);
    let dummy_sig = BytesN::from_array(&env, &[0u8; 64]);

    let owner = Address::generate(&env);
    client.mint(&relayer, &owner, &old_passkey, &recovery_pubkey);

    client.recover_soul(&wrong_relayer, &old_passkey, &new_passkey, &dummy_sig);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")] // SoulAlreadyExists
fn test_soul_already_exists() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    let passkey = BytesN::from_array(&env, &[0u8; 65]);
    let recovery_pubkey = BytesN::from_array(&env, &[1u8; 65]);

    let owner = Address::generate(&env);
    client.mint(&relayer, &owner, &passkey, &recovery_pubkey);
    client.mint(&relayer, &owner, &passkey, &recovery_pubkey);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")] // SoulAlreadyExists
fn test_passkey_already_in_use() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);

    client.initialize(&admin, &relayer);

    let passkey = BytesN::from_array(&env, &[0u8; 65]);
    let recovery_pubkey = BytesN::from_array(&env, &[1u8; 65]);

    let owner1 = Address::generate(&env);
    client.mint(&relayer, &owner1, &passkey, &recovery_pubkey);

    let owner2 = Address::generate(&env);
    client.mint(&relayer, &owner2, &passkey, &recovery_pubkey);
}

#[test]
fn test_initialize_already_initialized() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin, &admin);
    let res = client.try_initialize(&admin, &admin);
    assert_eq!(res, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn test_unauthorized_mint() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(ZolvencySoulContract, ());
    let client = ZolvencySoulContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let relayer = Address::generate(&env);
    let attacker = Address::generate(&env);
    client.initialize(&admin, &relayer);

    let owner = Address::generate(&env);
    let res = client.try_mint(
        &attacker,
        &owner,
        &BytesN::from_array(&env, &[0u8; 65]),
        &BytesN::from_array(&env, &[0u8; 65]),
    );
    assert_eq!(res, Err(Ok(Error::NotAuthorized)));
}
