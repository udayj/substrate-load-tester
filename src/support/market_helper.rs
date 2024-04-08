use super::converter::*;
use crate::polkadot::polkadot;

use polkadot::runtime_types::pallet_support::types::market::*;

use sp_arithmetic::{fixed_point::FixedI128};
use sp_core::bounded_vec::BoundedVec;
use sp_core::ConstU32;

pub fn btc_usdc() -> ExtendedMarket {
    let mut metadata_url: BoundedVec<u8, ConstU32<25>> = BoundedVec::new();
    for &byte in b"https://x.com/zkxprotocol" {
        if let Err(_) = metadata_url.try_push(byte) {
            break; // If we reach the bound, stop adding elements.
        }
    }

    ExtendedMarket {
        market: Market {
            id: 18669996633965635,
            version: 1,
            asset: 4346947,
            asset_collateral: 1431520323,
            is_tradable: true,
            is_archived: false,
            ttl: 3600,
            tick_size: convert_to_fixed_i128(1.into()),
            tick_precision: 1,
            step_size: convert_to_fixed_i128(1.into()),
            step_precision: 1,
            minimum_order_size: convert_to_fixed_i128(1.into()),
            minimum_leverage: convert_to_fixed_i128(1.into()),
            maximum_leverage: convert_to_fixed_i128(10.into()),
            currently_allowed_leverage: convert_to_fixed_i128(8.into()),
            maintenance_margin_fraction: convert_to_fixed_i128(FixedI128::from_inner(
                75000000000000000,
            )),
            initial_margin_fraction: convert_to_fixed_i128(1.into()),
            incremental_initial_margin_fraction: convert_to_fixed_i128(1.into()),
            incremental_position_size: convert_to_fixed_i128(1.into()),
            baseline_position_size: convert_to_fixed_i128(1.into()),
            maximum_position_size: convert_to_fixed_i128(25000.into()),
        },
        metadata_url: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            metadata_url.to_vec(),
        ),
    }
}
