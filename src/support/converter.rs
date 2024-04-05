use super::types::*;
use crate::polkadot::polkadot;
use polkadot::runtime_types::pallet_support::types::asset::*;
use polkadot::runtime_types::pallet_support::types::market::*;
use polkadot::runtime_types::pallet_support::types::trading_account::TradingAccountMinimal;
use primitive_types::U256;
use scale_info::TypeInfo;
use sp_arithmetic::{fixed_point::FixedI128, traits::CheckedDiv, FixedPointNumber};
use sp_core::bounded_vec::BoundedVec;
use sp_core::ConstU32;
use sp_io::hashing::blake2_256;
use starknet_core::crypto::compute_hash_on_elements;
use starknet_crypto::poseidon_hash_many;
use starknet_crypto::{sign, FieldElement};
use starknet_ff::FromByteSliceError;

pub fn convertToFixedI128(
    num: FixedI128,
) -> polkadot::runtime_types::sp_arithmetic::fixed_point::FixedI128 {
    polkadot::runtime_types::sp_arithmetic::fixed_point::FixedI128(num.into_inner())
}

pub fn convert_to_u256(num: U256) -> polkadot::runtime_types::primitive_types::U256 {
    polkadot::runtime_types::primitive_types::U256(num.0)
}

pub fn convert_to_polkadot_order(
    order: Order,
) -> polkadot::runtime_types::pallet_support::types::trading::Order {
    polkadot::runtime_types::pallet_support::types::trading::Order {
        account_id: convert_to_u256(order.account_id),
        order_id: convert_to_u256(order.order_id),
        market_id: order.market_id,
        order_type: convert_to_order_type(order.order_type),
        direction: convert_to_direction(order.direction),
        side: convert_to_side(order.side),
        price: convertToFixedI128(order.price),
        size: convertToFixedI128(order.size),
        leverage: convertToFixedI128(order.leverage),
        slippage: convertToFixedI128(order.slippage),
        post_only: order.post_only,
        time_in_force: convert_to_time_in_force(order.time_in_force),
        signature_info: convert_to_signature_info(order.signature_info),
        timestamp: order.timestamp,
    }
}

pub fn convert_to_order_type(
    order_type: OrderType,
) -> polkadot::runtime_types::pallet_support::types::trading::OrderType {
    match order_type {
        OrderType::Limit => {
            polkadot::runtime_types::pallet_support::types::trading::OrderType::Limit
        }
        OrderType::Market => {
            polkadot::runtime_types::pallet_support::types::trading::OrderType::Market
        }
        _ => polkadot::runtime_types::pallet_support::types::trading::OrderType::Forced,
    }
}

pub fn convert_to_direction(
    direction: Direction,
) -> polkadot::runtime_types::pallet_support::types::trading::Direction {
    match direction {
        Direction::Long => polkadot::runtime_types::pallet_support::types::trading::Direction::Long,
        _ => polkadot::runtime_types::pallet_support::types::trading::Direction::Short,
    }
}

pub fn convert_to_side(
    side: Side,
) -> polkadot::runtime_types::pallet_support::types::trading::Side {
    match side {
        Side::Buy => polkadot::runtime_types::pallet_support::types::trading::Side::Buy,
        _ => polkadot::runtime_types::pallet_support::types::trading::Side::Sell,
    }
}

pub fn convert_to_time_in_force(
    tif: TimeInForce,
) -> polkadot::runtime_types::pallet_support::types::trading::TimeInForce {
    match tif {
        TimeInForce::FOK => {
            polkadot::runtime_types::pallet_support::types::trading::TimeInForce::FOK
        }
        TimeInForce::GTC => {
            polkadot::runtime_types::pallet_support::types::trading::TimeInForce::GTC
        }
        _ => polkadot::runtime_types::pallet_support::types::trading::TimeInForce::IOC,
    }
}

pub fn convert_to_signature_info(
    sig: SignatureInfo,
) -> polkadot::runtime_types::pallet_support::types::trading::SignatureInfo {
    polkadot::runtime_types::pallet_support::types::trading::SignatureInfo {
        liquidator_pub_key: convert_to_u256(sig.liquidator_pub_key),
        hash_type: convert_to_hash_type(sig.hash_type),
        sig_r: convert_to_u256(sig.sig_r),
        sig_s: convert_to_u256(sig.sig_s),
    }
}

pub fn convert_to_hash_type(
    hash_type: HashType,
) -> polkadot::runtime_types::pallet_support::types::common::HashType {
    match hash_type {
        HashType::Pedersen => {
            polkadot::runtime_types::pallet_support::types::common::HashType::Pedersen
        }
        _ => polkadot::runtime_types::pallet_support::types::common::HashType::Poseidon,
    }
}

pub fn convert_to_polkadot_base_fee(
    base_fees: Vec<BaseFee>,
) -> Vec<polkadot::runtime_types::pallet_support::types::trading_fees::BaseFee> {
    let mut ret_base_fees = vec![];
    for base_fee in base_fees {
        let converted_base_fee =
            polkadot::runtime_types::pallet_support::types::trading_fees::BaseFee {
                volume: convertToFixedI128(base_fee.volume),
                fee: convertToFixedI128(base_fee.fee),
            };
        ret_base_fees.push(converted_base_fee);
    }
    ret_base_fees
}

pub fn convert_to_u128_pair(
    u256_value: U256,
) -> Result<(FieldElement, FieldElement), FromByteSliceError> {
    let mut buffer: [u8; 32] = [0; 32];
    u256_value.to_big_endian(&mut buffer);

    let low_bytes = &buffer[16..];
    let high_bytes = &buffer[..16];

    let low_bytes_felt: FieldElement = FieldElement::from_byte_slice_be(&low_bytes)?;
    let high_bytes_felt = FieldElement::from_byte_slice_be(&high_bytes)?;

    Ok((low_bytes_felt, high_bytes_felt))
}
