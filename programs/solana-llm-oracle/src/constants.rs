use anchor_lang::prelude::*;

#[constant]
// Sol.College lab patch: the oracle identity is the lab deployer (upstream oRcjdxYJ...).
pub const ORACLE_IDENTITY: Pubkey = pubkey!("BP1uwSaTHXvgN8sPQJgaor6iBoT3KYjnZNnkvJr71zDv");
