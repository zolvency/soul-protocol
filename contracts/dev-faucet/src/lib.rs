#![no_std]

use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, Env, String, Symbol, IntoVal};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    Unauthorized = 2,
    MissingGithubSBT = 3,
    AlreadyClaimed = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Tier {
    Novice,
    Pro,
    Architect,
    Legend,
    Singularity,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GithubData {
    pub contributions: u32,
    pub expires_at: u64,
    pub external_id: String,
    pub minted_at: u64,
    pub soul_id: u32,
    pub tier: Tier,
    pub updated_at: u64,
    pub username: String,
}

#[contract]
pub struct DevFaucetContract;

#[contractimpl]
impl DevFaucetContract {
    pub fn initialize(env: Env, admin: Address, github_identity_contract: Address) -> Result<(), Error> {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            return Err(Error::NotInitialized); // Or already initialized
        }
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "github_contract"), &github_identity_contract);
        Ok(())
    }

    pub fn claim(env: Env, caller: Address, soul_id: u32) -> Result<u64, Error> {
        caller.require_auth();

        let github_contract: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "github_contract"))
            .unwrap();

        // 1. Try to get the token ID from the user's Soul ID
        // The cross contract call `get_holder_token(soul_id)` returns Result<u64, Error>
        // If it fails, they don't have the token.
        let token_res = env.try_invoke_contract::<u64, soroban_sdk::Error>(
            &github_contract,
            &Symbol::new(&env, "get_holder_token"),
            soroban_sdk::vec![&env, soul_id.into_val(&env)],
        );

        let token_id: u64 = match token_res {
            Ok(Ok(id)) => id,
            _ => return Err(Error::MissingGithubSBT),
        };

        // 2. Check if already claimed
        let claimed_key = (Symbol::new(&env, "claimed"), token_id); // unique key for this token
        if env.storage().persistent().has(&claimed_key) {
            return Err(Error::AlreadyClaimed);
        }

        // 3. Fetch Tier to calculate dynamic reward
        let data_res = env.try_invoke_contract::<GithubData, soroban_sdk::Error>(
            &github_contract,
            &Symbol::new(&env, "get_token_data"),
            soroban_sdk::vec![&env, token_id.into_val(&env)],
        );

        let reward_amount: u64 = match data_res {
            Ok(Ok(data)) => {
                match data.tier {
                    Tier::Novice => 100,
                    Tier::Pro => 500,
                    Tier::Architect => 1500,
                    Tier::Legend => 5000,
                    Tier::Singularity => 10000,
                }
            },
            _ => 100, // fallback
        };

        // Mark as claimed
        env.storage().persistent().set(&claimed_key, &true);

        // (In a real world, this would transfer actual XLM/Tokens. 
        // For our demo, it just successfully returns the amount claimed!)
        Ok(reward_amount)
    }

    pub fn has_claimed(env: Env, soul_id: u32) -> bool {
        let github_contract: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "github_contract"))
            .unwrap();

        let token_res = env.try_invoke_contract::<u64, soroban_sdk::Error>(
            &github_contract,
            &Symbol::new(&env, "get_holder_token"),
            soroban_sdk::vec![&env, soul_id.into_val(&env)],
        );

        let token_id: u64 = match token_res {
            Ok(Ok(id)) => id,
            _ => return false,
        };

        let claimed_key = (Symbol::new(&env, "claimed"), token_id);
        env.storage().persistent().has(&claimed_key)
    }
}
