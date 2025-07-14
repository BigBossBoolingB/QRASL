use crate::primitives::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Validator {
    pub address: Address,
    pub stake: u128,
    pub nominators: Vec<Nominator>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Nominator {
    pub address: Address,
    pub stake: u128,
}

pub fn stake(
    validators: &mut HashMap<Address, Validator>,
    staker: Address,
    amount: u128,
) {
    let validator = validators.entry(staker).or_insert(Validator {
        address: staker,
        stake: 0,
        nominators: vec![],
    });
    validator.stake += amount;
}

pub fn nominate(
    validators: &mut HashMap<Address, Validator>,
    nominator_address: Address,
    validator_address: Address,
    amount: u128,
) {
    if let Some(validator) = validators.get_mut(&validator_address) {
        validator.nominators.push(Nominator {
            address: nominator_address,
            stake: amount,
        });
    }
}

pub fn unstake(
    validators: &mut HashMap<Address, Validator>,
    staker: Address,
    amount: u128,
) {
    if let Some(validator) = validators.get_mut(&staker) {
        validator.stake = validator.stake.saturating_sub(amount);
    }
}
