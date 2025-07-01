use anchor_lang::prelude::*;


pub const  SEED_CONFIG_ACCOUNT : &[u8] = b"config";
pub const SEED_MINT_ACCOUNT : &[u8] = b"mint";
pub const SEED_COLLATERAL_ACCOUNT : &[U8] = b"collateral";


pub const MINT_DECIMALS : u8 = 9;
pub const LIQUIDATION_THRESHOLD: u64 = 50;
pub const LIQUIDATION_BONUS : u64 = 10;
pub const MIN_HEALTH_FACTOR : u64 = 1;