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
use super::market_helper::*;


pub trait FieldElementExt {
	fn to_u256(&self) -> U256;
}


#[derive(Clone, Default, PartialEq, TypeInfo)]
pub struct Order {
	pub account_id: U256,
	pub order_id: U256,
	pub market_id: u128,
	pub order_type: OrderType,
	pub direction: Direction,
	pub side: Side,
	pub price: FixedI128,
	pub size: FixedI128,
	pub leverage: FixedI128,
	pub slippage: FixedI128,
	pub post_only: bool,
	pub time_in_force: TimeInForce,
	pub signature_info: SignatureInfo,
	pub timestamp: u64,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo)]
pub enum OrderSide {
	#[default]
	Maker,
	Taker,
}

#[derive(
	Clone, Copy, Default, PartialEq, TypeInfo
)]
pub enum Side {
	#[default]
	Buy,
	Sell,
}

#[derive(
	Clone, Copy, Default, PartialEq, TypeInfo
)]
pub enum Direction {
	#[default]
	Long,
	Short,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo)]
pub enum OrderType {
	#[default]
	Limit,
	Market,
	Forced,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo)]
pub enum TimeInForce {
	#[default]
	GTC,
	IOC,
	FOK,
}

#[derive(
	Clone, Copy, Default, PartialEq, TypeInfo
)]
pub enum ForceClosureFlag {
	#[default]
	Deleverage,
	Liquidate,
}


#[derive(Clone, Default, PartialEq, TypeInfo)]
pub struct SignatureInfo {
	pub liquidator_pub_key: U256,
	pub hash_type: HashType,
	pub sig_r: U256,
	pub sig_s: U256,
}

#[derive(Clone, Default, PartialEq, TypeInfo, Eq)]
pub enum HashType {
	#[default]
	Pedersen,
	Poseidon,
}

#[derive(Clone, Copy,Default, PartialEq, TypeInfo)]
pub struct BaseFee {
	pub volume: FixedI128,
	pub fee: FixedI128,
}

pub mod general_conversion_error {
	#[derive(Debug)]
	pub enum GeneralConversionError {
		U256ToFieldElementError,
		EnumToFieldElementError,
	}
}

pub struct Chains;


