use super::traits::*;
use super::types::*;
use crate::polkadot::polkadot;
use polkadot::runtime_types::pallet_support::types::asset::*;
use primitive_types::U256;
use sp_core::bounded_vec::BoundedVec;
use sp_core::ConstU32;

pub fn eth() -> ExtendedAsset {
    let mut metadata_url: BoundedVec<u8, ConstU32<25>> = BoundedVec::<u8, ConstU32<25>>::new();
    for &byte in b"https://x.com/zkxprotocol" {
        if let Err(_) = metadata_url.try_push(byte) {
            break; // If we reach the bound, stop adding elements.
        }
    }

    let asset_addresses: BoundedVec<AssetAddress, ConstU32<25>> = BoundedVec::new();

    ExtendedAsset {
        asset: Asset {
            id: 4543560,
            version: 1,
            short_name: polkadot::runtime_types::primitive_types::U256(
                U256::from("0x457468657265756D").0,
            ),
            is_tradable: true,
            is_collateral: false,
            decimals: 18,
        },
        asset_addresses: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            asset_addresses.to_vec(),
        ),

        metadata_url: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            metadata_url.to_vec(),
        ),
    }
}

pub fn btc() -> ExtendedAsset {
    let mut metadata_url: BoundedVec<u8, ConstU32<25>> = BoundedVec::new();
    b"https://x.com/zkxprotocol"
        .iter()
        .for_each(|&byte| metadata_url.force_push(byte));

    let asset_addresses: BoundedVec<AssetAddress, ConstU32<25>> = BoundedVec::new();

    ExtendedAsset {
        asset: Asset {
            id: 4346947,
            version: 1,
            short_name: polkadot::runtime_types::primitive_types::U256(
                U256::from("0x426974636F696E").0,
            ),
            is_tradable: true,
            is_collateral: false,
            decimals: 6,
        },
        asset_addresses: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            asset_addresses.to_vec(),
        ),

        metadata_url: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            metadata_url.to_vec(),
        ),
    }
}

pub fn usdc() -> ExtendedAsset {
    let mut metadata_url: BoundedVec<u8, ConstU32<25>> = BoundedVec::new();
    b"https://x.com/zkxprotocol"
        .iter()
        .for_each(|&byte| metadata_url.force_push(byte));

    let mut asset_addresses: BoundedVec<AssetAddress, ConstU32<25>> = BoundedVec::new();
    asset_addresses.force_push(AssetAddress {
        chain: Chains::starknet_chain(),
        address: polkadot::runtime_types::primitive_types::U256(U256::from(200).0),
    });
    asset_addresses.force_push(AssetAddress {
        chain: Chains::zkx_sync_chain(),
        address: polkadot::runtime_types::primitive_types::U256(U256::from(201).0),
    });

    ExtendedAsset {
        asset: Asset {
            id: 1431520323,
            version: 1,
            short_name: polkadot::runtime_types::primitive_types::U256(
                U256::from("0x55534420436972636C65").0,
            ),
            is_tradable: false,
            is_collateral: true,
            decimals: 6,
        },
        asset_addresses: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            asset_addresses.to_vec(),
        ),

        metadata_url: polkadot::runtime_types::bounded_collections::bounded_vec::BoundedVec(
            metadata_url.to_vec(),
        ),
    }
}
