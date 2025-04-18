#![no_std]
#![allow(non_snake_case)]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, Vec, Address, log};

#[contracttype]
#[derive(Clone)]
pub struct VestingInfo {
    pub beneficiary: Address,
    pub total_amount: u64,
    pub released_amount: u64,
    pub start_time: u64,
    pub duration: u64,
}

#[contract]
pub struct VestingTokenContract;

#[contractimpl]
impl VestingTokenContract {
    // Store vesting schedule
    pub fn set_vesting_schedule(
        env: Env,
        beneficiary: Address,
        total_amount: u64,
        duration: u64,
    ) {
        let timestamp = env.ledger().timestamp();
        let info = VestingInfo {
            beneficiary: beneficiary.clone(),
            total_amount,
            released_amount: 0,
            start_time: timestamp,
            duration,
        };
        env.storage().persistent().set(&beneficiary, &info);
        log!(&env, "Vesting set for: {}", beneficiary);
    }

    // View vesting details
    pub fn get_vesting_info(env: Env, beneficiary: Address) -> VestingInfo {
        env.storage()
            .persistent()
            .get(&beneficiary)
            .expect("No vesting found")
    }

    // Claim vested tokens
    pub fn claim_tokens(env: Env, beneficiary: Address) -> u64 {
        let mut info = Self::get_vesting_info(env.clone(), beneficiary.clone());
        let current_time = env.ledger().timestamp();

        let elapsed = current_time - info.start_time;
        if elapsed > info.duration {
            info.duration = elapsed; // Allow claiming full amount after duration
        }

        let vested = (info.total_amount * elapsed) / info.duration;
        let claimable = vested.saturating_sub(info.released_amount);
        if claimable > 0 {
            info.released_amount += claimable;
            env.storage().persistent().set(&beneficiary, &info);
            log!(&env, "Claimed {} tokens", claimable);
            return claimable;
        }

        0
    }
}
