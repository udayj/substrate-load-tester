use crate::polkadot::polkadot;
use polkadot::runtime_types::pallet_support::types::asset::*;
use polkadot::runtime_types::pallet_support::types::market::*;
use polkadot::runtime_types::pallet_support::types::trading_account::TradingAccountMinimal;
use sp_core::ConstU32;
use primitive_types::U256;
use sp_core::bounded_vec::BoundedVec;
use sp_arithmetic::{fixed_point::FixedI128, traits::CheckedDiv, FixedPointNumber};
use scale_info::TypeInfo;
use starknet_crypto::{sign, FieldElement};
use starknet_crypto::poseidon_hash_many;
use starknet_core::crypto::compute_hash_on_elements;
use starknet_ff::FromByteSliceError;
use sp_io::hashing::blake2_256;
use super::converter::*;

pub fn btc_usdc() -> ExtendedMarket {
	let mut metadata_url:BoundedVec<u8,ConstU32<25>> = BoundedVec::new();
	for &byte in b"https://x.com/zkxprotocol" {
		if let Err(_) = metadata_url.try_push(byte) {
			break // If we reach the bound, stop adding elements.
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
			tick_size: convertToFixedI128(1.into()),
			tick_precision: 1,
			step_size: convertToFixedI128(1.into()),
			step_precision: 1,
			minimum_order_size: convertToFixedI128(1.into()),
			minimum_leverage: convertToFixedI128(1.into()),
			maximum_leverage: convertToFixedI128(10.into()),
			currently_allowed_leverage: convertToFixedI128(8.into()),
			maintenance_margin_fraction: convertToFixedI128(FixedI128::from_inner(75000000000000000)),
			initial_margin_fraction: convertToFixedI128(1.into()),
			incremental_initial_margin_fraction: convertToFixedI128(1.into()),
			incremental_position_size: convertToFixedI128(1.into()),
			baseline_position_size: convertToFixedI128(1.into()),
			maximum_position_size: convertToFixedI128(25000.into()),
		},
		metadata_url: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(metadata_url.to_vec()),
	}
}
