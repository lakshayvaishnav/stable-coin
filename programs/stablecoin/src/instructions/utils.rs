use anchor_lang::{ prelude::*, solana_program::native_token::LAMPORTS_PER_SOL };
use pyth_solana_receiver_sdk::price_update::{ get_feed_id_from_hex, PriceUpdateV2 };

use crate::{
    constants::{ FEED_ID, MAXMIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT },
    state::{ Collateral, Config },
    CustomErrorCode,
};

pub fn check_health_factor(
    collateral: &Account<Collateral>,
    price_feed: &Account<PriceUpdateV2>,
    config: &Account<Config>
) -> Result<()> {
    Ok(())
}

pub fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXMIMUM_AGE, &feed_id)?;

    // check price is positive
    require!(price.price > 0, CustomErrorCode::InvalidPrice);

    let price_in_usd = (price.price as u128) * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_usd = ((*amount_in_lamports as u128) * price_in_usd) / (LAMPORTS_PER_SOL as u128);

    Ok(amount_in_usd as u64)
}
