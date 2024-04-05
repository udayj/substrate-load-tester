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

pub mod account_helper;
pub mod asset_helper;
pub mod converter;
pub mod market_helper;
pub mod traits;
pub mod types;

pub use account_helper::*;
pub use asset_helper::*;
pub use converter::*;
pub use market_helper::*;
pub use traits::*;
pub use types::*;

pub mod utilities {
    use super::*;
    pub fn get_private_key(pub_key: U256) -> FieldElement {
        if pub_key == U256(alice().pub_key.0) {
            FieldElement::from(12345_u128)
        } else if pub_key == U256(bob().pub_key.0) {
            FieldElement::from(12346_u128)
        } else {
            FieldElement::from(0_u128)
        }
    }

    pub fn get_trading_account_id(trading_account: TradingAccountMinimal) -> U256 {
        let mut result: [u8; 33] = [0; 33];
        U256(trading_account.account_address.0).to_little_endian(&mut result[0..32]);
        result[32] = trading_account.index;

        blake2_256(&result).into()
    }

    pub fn setup_fee() -> (Vec<BaseFee>, Vec<BaseFee>) {
        // TODO(merkle-groot): Using manual pushing because vec! has some issues in support pallet
        // fee tiers
        let mut fee_tiers = Vec::<u8>::new();
        fee_tiers.push(1_u8);
        fee_tiers.push(2_u8);
        fee_tiers.push(3_u8);

        // fee details
        let mut fee_details_maker = Vec::<BaseFee>::new();
        let base_fee1 = BaseFee {
            volume: 0.into(),
            fee: FixedI128::from_inner(20000000000000000),
        };
        fee_details_maker.push(base_fee1);
        let base_fee2 = BaseFee {
            volume: 1000.into(),
            fee: FixedI128::from_inner(15000000000000000),
        };
        fee_details_maker.push(base_fee2);
        let base_fee3 = BaseFee {
            volume: 5000.into(),
            fee: FixedI128::from_inner(10000000000000000),
        };
        fee_details_maker.push(base_fee3);

        let mut fee_details_taker: Vec<BaseFee> = Vec::new();
        let base_fee1 = BaseFee {
            volume: 0.into(),
            fee: FixedI128::from_inner(50000000000000000),
        };
        let base_fee2 = BaseFee {
            volume: 1000.into(),
            fee: FixedI128::from_inner(40000000000000000),
        };
        let base_fee3 = BaseFee {
            volume: 5000.into(),
            fee: FixedI128::from_inner(35000000000000000),
        };
        fee_details_taker.push(base_fee1);
        fee_details_taker.push(base_fee2);
        fee_details_taker.push(base_fee3);

        (fee_details_maker, fee_details_taker)
    }
}

pub use utilities::*;
