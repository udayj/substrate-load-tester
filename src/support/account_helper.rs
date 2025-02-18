use crate::polkadot::polkadot;
use polkadot::runtime_types::pallet_support::types::trading_account::TradingAccountMinimal;
use primitive_types::U256;

pub fn alice() -> TradingAccountMinimal {
    TradingAccountMinimal {
        account_address: polkadot::runtime_types::primitive_types::U256(U256::from(100_u8).0),
        index: 0,
        pub_key: polkadot::runtime_types::primitive_types::U256(
            U256::from_dec_str(
                "1628448741648245036800002906075225705100596136133912895015035902954123957052",
            )
            .unwrap()
            .0,
        ),
    }
}

pub fn bob() -> TradingAccountMinimal {
    TradingAccountMinimal {
        account_address: polkadot::runtime_types::primitive_types::U256(U256::from(101_u8).0),
        index: 0,
        pub_key: polkadot::runtime_types::primitive_types::U256(
            U256::from_dec_str(
                "2734587570975953215033319696922164262260826928445675130094490350860110775927",
            )
            .unwrap()
            .0,
        ),
    }
}
