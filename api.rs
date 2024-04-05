#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 18usize] = [
        "System",
        "Timestamp",
        "Balances",
        "ValidatorSet",
        "Session",
        "Aura",
        "Grandpa",
        "TransactionPayment",
        "Sudo",
        "TradingAccount",
        "Assets",
        "Markets",
        "SyncFacade",
        "Trading",
        "TradingFees",
        "Prices",
        "RiskManagement",
        "NodeAuthorization",
    ];
    pub static RUNTIME_APIS: [&str; 13usize] = [
        "TradingApi",
        "PricesApi",
        "Core",
        "Metadata",
        "BlockBuilder",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "AuraApi",
        "SessionKeys",
        "GrandpaApi",
        "AccountNonceApi",
        "TransactionPaymentApi",
        "TransactionPaymentCallApi",
    ];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::node_template_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::node_template_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::node_template_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn trading_api(&self) -> trading_api::TradingApi {
                trading_api::TradingApi
            }
            pub fn prices_api(&self) -> prices_api::PricesApi {
                prices_api::PricesApi
            }
            pub fn core(&self) -> core::Core {
                core::Core
            }
            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }
            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }
            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }
            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }
            pub fn aura_api(&self) -> aura_api::AuraApi {
                aura_api::AuraApi
            }
            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }
            pub fn grandpa_api(&self) -> grandpa_api::GrandpaApi {
                grandpa_api::GrandpaApi
            }
            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }
            pub fn transaction_payment_api(
                &self,
            ) -> transaction_payment_api::TransactionPaymentApi {
                transaction_payment_api::TransactionPaymentApi
            }
            pub fn transaction_payment_call_api(
                &self,
            ) -> transaction_payment_call_api::TransactionPaymentCallApi {
                transaction_payment_call_api::TransactionPaymentCallApi
            }
        }
        pub mod trading_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TradingApi;
            impl TradingApi {
                pub fn positions(
                    &self,
                    account_id: runtime_types::primitive_types::U256,
                    collateral_id: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<
                    types::Positions,
                    ::std::vec::Vec<
                        runtime_types::pallet_support::types::trading::PositionExtended,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "positions",
                        types::Positions {
                            account_id,
                            collateral_id,
                        },
                        [
                            116u8, 119u8, 166u8, 7u8, 23u8, 15u8, 243u8, 207u8, 89u8, 125u8, 92u8,
                            98u8, 225u8, 91u8, 127u8, 67u8, 43u8, 82u8, 43u8, 123u8, 79u8, 209u8,
                            183u8, 119u8, 220u8, 176u8, 52u8, 181u8, 35u8, 10u8, 61u8, 127u8,
                        ],
                    )
                }
                pub fn get_margin_info(
                    &self,
                    account_id: runtime_types::primitive_types::U256,
                    collateral_id: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetMarginInfo,
                    runtime_types::pallet_support::types::trading::MarginInfo,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "get_margin_info",
                        types::GetMarginInfo {
                            account_id,
                            collateral_id,
                        },
                        [
                            161u8, 7u8, 28u8, 67u8, 224u8, 139u8, 41u8, 115u8, 2u8, 94u8, 183u8,
                            79u8, 153u8, 13u8, 98u8, 76u8, 62u8, 154u8, 168u8, 221u8, 169u8, 194u8,
                            77u8, 225u8, 135u8, 169u8, 243u8, 200u8, 123u8, 207u8, 9u8, 33u8,
                        ],
                    )
                }
                pub fn get_account_info(
                    &self,
                    account_id: runtime_types::primitive_types::U256,
                    collateral_id: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetAccountInfo,
                    runtime_types::pallet_support::types::trading::AccountInfo,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "get_account_info",
                        types::GetAccountInfo {
                            account_id,
                            collateral_id,
                        },
                        [
                            119u8, 85u8, 28u8, 251u8, 211u8, 147u8, 176u8, 157u8, 73u8, 12u8,
                            221u8, 4u8, 41u8, 114u8, 85u8, 77u8, 237u8, 130u8, 170u8, 7u8, 197u8,
                            0u8, 105u8, 154u8, 83u8, 98u8, 50u8, 169u8, 78u8, 128u8, 159u8, 116u8,
                        ],
                    )
                }
                pub fn get_account_list(
                    &self,
                    start_index: ::core::primitive::u128,
                    end_index: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetAccountList,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "get_account_list",
                        types::GetAccountList {
                            start_index,
                            end_index,
                        },
                        [
                            148u8, 227u8, 235u8, 204u8, 168u8, 240u8, 217u8, 117u8, 61u8, 6u8,
                            93u8, 238u8, 218u8, 100u8, 171u8, 202u8, 42u8, 9u8, 249u8, 7u8, 68u8,
                            249u8, 212u8, 6u8, 234u8, 214u8, 195u8, 13u8, 219u8, 28u8, 41u8, 223u8,
                        ],
                    )
                }
                pub fn get_fee(
                    &self,
                    account_id: runtime_types::primitive_types::U256,
                    market_id: runtime_types::primitive_types::U256,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetFee,
                    (
                        runtime_types::pallet_support::types::trading_fees::FeeRates,
                        ::core::primitive::u64,
                    ),
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "get_fee",
                        types::GetFee {
                            account_id,
                            market_id,
                        },
                        [
                            255u8, 59u8, 30u8, 157u8, 182u8, 244u8, 132u8, 52u8, 56u8, 162u8,
                            243u8, 215u8, 242u8, 77u8, 162u8, 207u8, 41u8, 139u8, 46u8, 166u8,
                            173u8, 110u8, 169u8, 47u8, 239u8, 27u8, 167u8, 210u8, 43u8, 108u8,
                            131u8, 22u8,
                        ],
                    )
                }
                pub fn get_withdrawable_amount(
                    &self,
                    account_id: runtime_types::primitive_types::U256,
                    collateral_id: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetWithdrawableAmount,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "get_withdrawable_amount",
                        types::GetWithdrawableAmount {
                            account_id,
                            collateral_id,
                        },
                        [
                            46u8, 136u8, 214u8, 60u8, 0u8, 167u8, 109u8, 99u8, 84u8, 119u8, 11u8,
                            39u8, 135u8, 210u8, 92u8, 58u8, 198u8, 166u8, 219u8, 128u8, 66u8, 75u8,
                            221u8, 206u8, 237u8, 214u8, 235u8, 91u8, 104u8, 195u8, 88u8, 53u8,
                        ],
                    )
                }
                pub fn get_remaining_trading_cleanup_calls(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetRemainingTradingCleanupCalls,
                    ::core::primitive::u64,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TradingApi",
                        "get_remaining_trading_cleanup_calls",
                        types::GetRemainingTradingCleanupCalls {},
                        [
                            89u8, 25u8, 95u8, 227u8, 48u8, 250u8, 255u8, 15u8, 217u8, 102u8, 162u8,
                            153u8, 95u8, 32u8, 204u8, 137u8, 84u8, 61u8, 146u8, 87u8, 203u8, 53u8,
                            169u8, 255u8, 227u8, 62u8, 101u8, 131u8, 75u8, 33u8, 174u8, 26u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Positions {
                    pub account_id: runtime_types::primitive_types::U256,
                    pub collateral_id: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetMarginInfo {
                    pub account_id: runtime_types::primitive_types::U256,
                    pub collateral_id: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetAccountInfo {
                    pub account_id: runtime_types::primitive_types::U256,
                    pub collateral_id: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetAccountList {
                    pub start_index: ::core::primitive::u128,
                    pub end_index: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetFee {
                    pub account_id: runtime_types::primitive_types::U256,
                    pub market_id: runtime_types::primitive_types::U256,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetWithdrawableAmount {
                    pub account_id: runtime_types::primitive_types::U256,
                    pub collateral_id: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetRemainingTradingCleanupCalls {}
            }
        }
        pub mod prices_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct PricesApi;
            impl PricesApi {
                pub fn get_remaining_markets(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetRemainingMarkets,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_remaining_markets",
                        types::GetRemainingMarkets {},
                        [
                            16u8, 81u8, 96u8, 48u8, 33u8, 29u8, 26u8, 4u8, 237u8, 15u8, 239u8,
                            109u8, 20u8, 63u8, 150u8, 161u8, 212u8, 44u8, 225u8, 87u8, 90u8, 201u8,
                            46u8, 213u8, 36u8, 82u8, 93u8, 173u8, 147u8, 239u8, 40u8, 183u8,
                        ],
                    )
                }
                pub fn get_no_of_batches_for_current_epoch(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetNoOfBatchesForCurrentEpoch,
                    ::core::primitive::u64,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_no_of_batches_for_current_epoch",
                        types::GetNoOfBatchesForCurrentEpoch {},
                        [
                            123u8, 6u8, 158u8, 212u8, 151u8, 18u8, 137u8, 235u8, 87u8, 165u8,
                            119u8, 167u8, 216u8, 213u8, 67u8, 51u8, 19u8, 44u8, 171u8, 204u8, 43u8,
                            79u8, 200u8, 176u8, 39u8, 226u8, 224u8, 141u8, 186u8, 111u8, 164u8,
                            194u8,
                        ],
                    )
                }
                pub fn get_last_abr_timestamp(
                    &self,
                ) -> ::subxt::runtime_api::Payload<types::GetLastAbrTimestamp, ::core::primitive::u64>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_last_abr_timestamp",
                        types::GetLastAbrTimestamp {},
                        [
                            209u8, 79u8, 84u8, 232u8, 64u8, 253u8, 162u8, 68u8, 247u8, 90u8, 200u8,
                            213u8, 158u8, 184u8, 137u8, 234u8, 20u8, 20u8, 99u8, 188u8, 24u8,
                            108u8, 105u8, 205u8, 59u8, 5u8, 84u8, 136u8, 94u8, 63u8, 227u8, 121u8,
                        ],
                    )
                }
                pub fn get_remaining_pay_abr_calls(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetRemainingPayAbrCalls,
                    ::core::primitive::u64,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_remaining_pay_abr_calls",
                        types::GetRemainingPayAbrCalls {},
                        [
                            216u8, 234u8, 156u8, 81u8, 134u8, 194u8, 139u8, 33u8, 136u8, 169u8,
                            226u8, 200u8, 73u8, 174u8, 230u8, 99u8, 97u8, 12u8, 147u8, 171u8,
                            159u8, 6u8, 253u8, 244u8, 41u8, 26u8, 103u8, 98u8, 79u8, 207u8, 252u8,
                            208u8,
                        ],
                    )
                }
                pub fn get_next_abr_timestamp(
                    &self,
                ) -> ::subxt::runtime_api::Payload<types::GetNextAbrTimestamp, ::core::primitive::u64>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_next_abr_timestamp",
                        types::GetNextAbrTimestamp {},
                        [
                            203u8, 71u8, 194u8, 245u8, 137u8, 158u8, 165u8, 254u8, 116u8, 80u8,
                            202u8, 200u8, 238u8, 22u8, 37u8, 119u8, 233u8, 118u8, 242u8, 139u8,
                            185u8, 51u8, 249u8, 11u8, 131u8, 207u8, 148u8, 18u8, 113u8, 206u8,
                            68u8, 245u8,
                        ],
                    )
                }
                pub fn get_previous_abr_values(
                    &self,
                    market_id: runtime_types::primitive_types::U256,
                    start_timestamp: ::core::primitive::u64,
                    end_timestamp: ::core::primitive::u64,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetPreviousAbrValues,
                    ::std::vec::Vec<runtime_types::pallet_support::types::abr::ABRDetails>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_previous_abr_values",
                        types::GetPreviousAbrValues {
                            market_id,
                            start_timestamp,
                            end_timestamp,
                        },
                        [
                            208u8, 72u8, 2u8, 184u8, 13u8, 51u8, 69u8, 120u8, 156u8, 69u8, 231u8,
                            171u8, 179u8, 131u8, 232u8, 143u8, 0u8, 46u8, 201u8, 130u8, 191u8,
                            49u8, 21u8, 112u8, 193u8, 216u8, 160u8, 142u8, 86u8, 95u8, 134u8,
                            198u8,
                        ],
                    )
                }
                pub fn get_intermediary_abr_value(
                    &self,
                    market_id: runtime_types::primitive_types::U256,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetIntermediaryAbrValue,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_intermediary_abr_value",
                        types::GetIntermediaryAbrValue { market_id },
                        [
                            16u8, 145u8, 57u8, 0u8, 253u8, 86u8, 111u8, 134u8, 29u8, 178u8, 71u8,
                            9u8, 133u8, 82u8, 181u8, 3u8, 233u8, 230u8, 118u8, 112u8, 37u8, 78u8,
                            244u8, 52u8, 129u8, 182u8, 217u8, 15u8, 230u8, 156u8, 165u8, 229u8,
                        ],
                    )
                }
                pub fn get_remaining_prices_cleanup_calls(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetRemainingPricesCleanupCalls,
                    ::core::primitive::u64,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "PricesApi",
                        "get_remaining_prices_cleanup_calls",
                        types::GetRemainingPricesCleanupCalls {},
                        [
                            51u8, 161u8, 188u8, 210u8, 117u8, 157u8, 120u8, 78u8, 129u8, 189u8,
                            86u8, 102u8, 129u8, 5u8, 111u8, 237u8, 59u8, 33u8, 79u8, 235u8, 37u8,
                            43u8, 244u8, 253u8, 246u8, 226u8, 16u8, 66u8, 155u8, 108u8, 194u8,
                            180u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetRemainingMarkets {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetNoOfBatchesForCurrentEpoch {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetLastAbrTimestamp {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetRemainingPayAbrCalls {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetNextAbrTimestamp {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetPreviousAbrValues {
                    pub market_id: runtime_types::primitive_types::U256,
                    pub start_timestamp: ::core::primitive::u64,
                    pub end_timestamp: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetIntermediaryAbrValue {
                    pub market_id: runtime_types::primitive_types::U256,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetRemainingPricesCleanupCalls {}
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Core` runtime api that every Substrate runtime needs to implement."]
            pub struct Core;
            impl Core {
                #[doc = " Returns the version of the runtime."]
                pub fn version(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Version,
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "version",
                        types::Version {},
                        [
                            76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
                            125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
                            116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
                            6u8,
                        ],
                    )
                }
                #[doc = " Execute the given block."]
                pub fn execute_block(
                    &self,
                    block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > >,
                ) -> ::subxt::runtime_api::Payload<types::ExecuteBlock, ()> {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "execute_block",
                        types::ExecuteBlock { block },
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }
                #[doc = " Initialize a block with the given header."]
                pub fn initialize_block(
                    &self,
                    header: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::runtime_api::Payload<types::InitializeBlock, ()> {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "initialize_block",
                        types::InitializeBlock { header },
                        [
                            146u8, 138u8, 72u8, 240u8, 63u8, 96u8, 110u8, 189u8, 77u8, 92u8, 96u8,
                            232u8, 41u8, 217u8, 105u8, 148u8, 83u8, 190u8, 152u8, 219u8, 19u8,
                            87u8, 163u8, 1u8, 232u8, 25u8, 221u8, 74u8, 224u8, 67u8, 223u8, 34u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Version {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExecuteBlock { pub block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InitializeBlock {
                    pub header:
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>,
                }
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Metadata` api trait that returns metadata for the runtime."]
            pub struct Metadata;
            impl Metadata {
                #[doc = " Returns the metadata of a runtime."]
                pub fn metadata(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Metadata,
                    runtime_types::sp_core::OpaqueMetadata,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata",
                        types::Metadata {},
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }
                #[doc = " Returns the metadata at a given version."]
                #[doc = ""]
                #[doc = " If the given `version` isn't supported, this will return `None`."]
                #[doc = " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."]
                pub fn metadata_at_version(
                    &self,
                    version: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::MetadataAtVersion,
                    ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        types::MetadataAtVersion { version },
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }
                #[doc = " Returns the supported metadata versions."]
                #[doc = ""]
                #[doc = " This can be used to call `metadata_at_version`."]
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::MetadataVersions,
                    ::std::vec::Vec<::core::primitive::u32>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata_versions",
                        types::MetadataVersions {},
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Metadata {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MetadataAtVersion {
                    pub version: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MetadataVersions {}
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `BlockBuilder` api trait that provides the required functionality for building a block."]
            pub struct BlockBuilder;
            impl BlockBuilder {
                #[doc = " Apply the given extrinsic."]
                #[doc = ""]
                #[doc = " Returns an inclusion outcome which specifies if this extrinsic is included in"]
                #[doc = " this block or not."]
                pub fn apply_extrinsic(
                    &self,
                    extrinsic : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) >,
                ) -> ::subxt::runtime_api::Payload<
                    types::ApplyExtrinsic,
                    ::core::result::Result<
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        types::ApplyExtrinsic { extrinsic },
                        [
                            72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
                            223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
                            97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
                        ],
                    )
                }
                #[doc = " Finish the current block."]
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::FinalizeBlock,
                    runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        types::FinalizeBlock {},
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }
                #[doc = " Generate inherent extrinsics. The inherent data will vary from chain to chain."]                pub fn inherent_extrinsics (& self , inherent : runtime_types :: sp_inherents :: InherentData ,) -> :: subxt :: runtime_api :: Payload < types :: InherentExtrinsics , :: std :: vec :: Vec < :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > >{
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        types::InherentExtrinsics { inherent },
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }
                #[doc = " Check that the inherents are valid. The inherent data will vary from chain to chain."]
                pub fn check_inherents(
                    &self,
                    block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > >,
                    data: runtime_types::sp_inherents::InherentData,
                ) -> ::subxt::runtime_api::Payload<
                    types::CheckInherents,
                    runtime_types::sp_inherents::CheckInherentsResult,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        types::CheckInherents { block, data },
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ApplyExtrinsic { pub extrinsic : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FinalizeBlock {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InherentExtrinsics {
                    pub inherent: runtime_types::sp_inherents::InherentData,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckInherents { pub block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > , pub data : runtime_types :: sp_inherents :: InherentData , }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `TaggedTransactionQueue` api trait for interfering with the transaction queue."]
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                #[doc = " Validate the transaction."]
                #[doc = ""]
                #[doc = " This method is invoked by the transaction pool to learn details about given transaction."]
                #[doc = " The implementation should make sure to verify the correctness of the transaction"]
                #[doc = " against current state. The given `block_hash` corresponds to the hash of the block"]
                #[doc = " that is used as current state."]
                #[doc = ""]
                #[doc = " Note that this call may be performed by the pool multiple times and transactions"]
                #[doc = " might be verified in any possible order."]
                pub fn validate_transaction(
                    &self,
                    source: runtime_types::sp_runtime::transaction_validity::TransactionSource,
                    tx : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) >,
                    block_hash: ::subxt::utils::H256,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidateTransaction,
                    ::core::result::Result<
                        runtime_types::sp_runtime::transaction_validity::ValidTransaction,
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        types::ValidateTransaction {
                            source,
                            tx,
                            block_hash,
                        },
                        [
                            196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
                            143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
                            162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidateTransaction { pub source : runtime_types :: sp_runtime :: transaction_validity :: TransactionSource , pub tx : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > , pub block_hash : :: subxt :: utils :: H256 , }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The offchain worker api."]
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                #[doc = " Starts the off-chain task for given block header."]
                pub fn offchain_worker(
                    &self,
                    header: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::runtime_api::Payload<types::OffchainWorker, ()> {
                    ::subxt::runtime_api::Payload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        types::OffchainWorker { header },
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OffchainWorker {
                    pub header:
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>,
                }
            }
        }
        pub mod aura_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API necessary for block authorship with aura."]
            pub struct AuraApi;
            impl AuraApi {
                #[doc = " Returns the slot duration for Aura."]
                #[doc = ""]
                #[doc = " Currently, only the value provided by this type at genesis will be used."]
                pub fn slot_duration(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::SlotDuration,
                    runtime_types::sp_consensus_slots::SlotDuration,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "AuraApi",
                        "slot_duration",
                        types::SlotDuration {},
                        [
                            233u8, 210u8, 132u8, 172u8, 100u8, 125u8, 239u8, 92u8, 114u8, 82u8,
                            7u8, 110u8, 179u8, 196u8, 10u8, 19u8, 211u8, 15u8, 174u8, 2u8, 91u8,
                            73u8, 133u8, 100u8, 205u8, 201u8, 191u8, 60u8, 163u8, 122u8, 215u8,
                            10u8,
                        ],
                    )
                }
                #[doc = " Return the current set of authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Authorities,
                    ::std::vec::Vec<runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "AuraApi",
                        "authorities",
                        types::Authorities {},
                        [
                            96u8, 136u8, 226u8, 244u8, 105u8, 189u8, 8u8, 250u8, 71u8, 230u8, 37u8,
                            123u8, 218u8, 47u8, 179u8, 16u8, 170u8, 181u8, 165u8, 77u8, 102u8,
                            51u8, 43u8, 51u8, 186u8, 84u8, 49u8, 15u8, 208u8, 226u8, 129u8, 230u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SlotDuration {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Authorities {}
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Session keys runtime api."]
            pub struct SessionKeys;
            impl SessionKeys {
                #[doc = " Generate a set of session keys with optionally using the given seed."]
                #[doc = " The keys should be stored within the keystore exposed via runtime"]
                #[doc = " externalities."]
                #[doc = ""]
                #[doc = " The seed needs to be a valid `utf8` string."]
                #[doc = ""]
                #[doc = " Returns the concatenated SCALE encoded public keys."]
                pub fn generate_session_keys(
                    &self,
                    seed: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateSessionKeys,
                    ::std::vec::Vec<::core::primitive::u8>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        types::GenerateSessionKeys { seed },
                        [
                            96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
                            102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
                            190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
                        ],
                    )
                }
                #[doc = " Decode the given public session keys."]
                #[doc = ""]
                #[doc = " Returns the list of public raw public keys + key type."]
                pub fn decode_session_keys(
                    &self,
                    encoded: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::runtime_api::Payload<
                    types::DecodeSessionKeys,
                    ::core::option::Option<
                        ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            runtime_types::sp_core::crypto::KeyTypeId,
                        )>,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        types::DecodeSessionKeys { encoded },
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateSessionKeys {
                    pub seed: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DecodeSessionKeys {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
        }
        pub mod grandpa_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " APIs for integrating the GRANDPA finality gadget into runtimes."]
            #[doc = " This should be implemented on the runtime side."]
            #[doc = ""]
            #[doc = " This is primarily used for negotiating authority-set changes for the"]
            #[doc = " gadget. GRANDPA uses a signaling model of changing authority sets:"]
            #[doc = " changes should be signaled with a delay of N blocks, and then automatically"]
            #[doc = " applied in the runtime after those N blocks have passed."]
            #[doc = ""]
            #[doc = " The consensus protocol will coordinate the handoff externally."]
            pub struct GrandpaApi;
            impl GrandpaApi {
                #[doc = " Get the current GRANDPA authorities and weights. This should not change except"]
                #[doc = " for when changes are scheduled and the corresponding delay has passed."]
                #[doc = ""]
                #[doc = " When called at block B, it will return the set of authorities that should be"]
                #[doc = " used to finalize descendants of this block (B+1, B+2, ...). The block B itself"]
                #[doc = " is finalized by the authorities from block B-1."]
                pub fn grandpa_authorities(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GrandpaAuthorities,
                    ::std::vec::Vec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "grandpa_authorities",
                        types::GrandpaAuthorities {},
                        [
                            166u8, 76u8, 160u8, 101u8, 242u8, 145u8, 213u8, 10u8, 16u8, 130u8,
                            230u8, 196u8, 125u8, 152u8, 92u8, 143u8, 119u8, 223u8, 140u8, 189u8,
                            203u8, 95u8, 52u8, 105u8, 147u8, 107u8, 135u8, 228u8, 62u8, 178u8,
                            128u8, 33u8,
                        ],
                    )
                }
                #[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
                #[doc = " must provide the equivocation proof and a key ownership proof"]
                #[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
                #[doc = " extrinsic will be unsigned and should only be accepted for local"]
                #[doc = " authorship (not to be broadcast to the network). This method returns"]
                #[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
                #[doc = " reporting is disabled for the given runtime (i.e. this method is"]
                #[doc = " hardcoded to return `None`). Only useful in an offchain context."]
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                ) -> ::subxt::runtime_api::Payload<
                    types::SubmitReportEquivocationUnsignedExtrinsic,
                    ::core::option::Option<()>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        types::SubmitReportEquivocationUnsignedExtrinsic {
                            equivocation_proof,
                            key_owner_proof,
                        },
                        [
                            112u8, 94u8, 150u8, 250u8, 132u8, 127u8, 185u8, 24u8, 113u8, 62u8,
                            28u8, 171u8, 83u8, 9u8, 41u8, 228u8, 92u8, 137u8, 29u8, 190u8, 214u8,
                            232u8, 100u8, 66u8, 100u8, 168u8, 149u8, 122u8, 93u8, 17u8, 236u8,
                            104u8,
                        ],
                    )
                }
                #[doc = " Generates a proof of key ownership for the given authority in the"]
                #[doc = " given set. An example usage of this module is coupled with the"]
                #[doc = " session historical module to prove that a given authority key is"]
                #[doc = " tied to a given staking identity during a specific session. Proofs"]
                #[doc = " of key ownership are necessary for submitting equivocation reports."]
                #[doc = " NOTE: even though the API takes a `set_id` as parameter the current"]
                #[doc = " implementations ignore this parameter and instead rely on this"]
                #[doc = " method being called at the correct block height, i.e. any point at"]
                #[doc = " which the given set id is live on-chain. Future implementations will"]
                #[doc = " instead use indexed data through an offchain worker, not requiring"]
                #[doc = " older states to be available."]
                pub fn generate_key_ownership_proof(
                    &self,
                    set_id: ::core::primitive::u64,
                    authority_id: runtime_types::sp_consensus_grandpa::app::Public,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateKeyOwnershipProof,
                    ::core::option::Option<
                        runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "generate_key_ownership_proof",
                        types::GenerateKeyOwnershipProof {
                            set_id,
                            authority_id,
                        },
                        [
                            40u8, 126u8, 113u8, 27u8, 245u8, 45u8, 123u8, 138u8, 12u8, 3u8, 125u8,
                            186u8, 151u8, 53u8, 186u8, 93u8, 13u8, 150u8, 163u8, 176u8, 206u8,
                            89u8, 244u8, 127u8, 182u8, 85u8, 203u8, 41u8, 101u8, 183u8, 209u8,
                            179u8,
                        ],
                    )
                }
                #[doc = " Get current GRANDPA authority set id."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::runtime_api::Payload<types::CurrentSetId, ::core::primitive::u64>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "current_set_id",
                        types::CurrentSetId {},
                        [
                            42u8, 230u8, 120u8, 211u8, 156u8, 245u8, 109u8, 86u8, 100u8, 146u8,
                            234u8, 205u8, 41u8, 183u8, 109u8, 42u8, 17u8, 33u8, 156u8, 25u8, 139u8,
                            84u8, 101u8, 75u8, 232u8, 198u8, 87u8, 136u8, 218u8, 233u8, 103u8,
                            156u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GrandpaAuthorities {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitReportEquivocationUnsignedExtrinsic {
                    pub equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    pub key_owner_proof:
                        runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateKeyOwnershipProof {
                    pub set_id: ::core::primitive::u64,
                    pub authority_id: runtime_types::sp_consensus_grandpa::app::Public,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CurrentSetId {}
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The API to query account nonce."]
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                #[doc = " Get current account nonce of given `AccountId`."]
                pub fn account_nonce(
                    &self,
                    account: ::subxt::utils::AccountId32,
                ) -> ::subxt::runtime_api::Payload<types::AccountNonce, ::core::primitive::u32>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        types::AccountNonce { account },
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountNonce {
                    pub account: ::subxt::utils::AccountId32,
                }
            }
        }
        pub mod transaction_payment_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentApi;
            impl TransactionPaymentApi {
                pub fn query_info(
                    &self,
                    uxt : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) >,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryInfo,
                    runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                        ::core::primitive::u128,
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_info",
                        types::QueryInfo { uxt, len },
                        [
                            56u8, 30u8, 174u8, 34u8, 202u8, 24u8, 177u8, 189u8, 145u8, 36u8, 1u8,
                            156u8, 98u8, 209u8, 178u8, 49u8, 198u8, 23u8, 150u8, 173u8, 35u8,
                            205u8, 147u8, 129u8, 42u8, 22u8, 69u8, 3u8, 129u8, 8u8, 196u8, 139u8,
                        ],
                    )
                }
                pub fn query_fee_details(
                    &self,
                    uxt : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) >,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryFeeDetails,
                    runtime_types::pallet_transaction_payment::types::FeeDetails<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_fee_details",
                        types::QueryFeeDetails { uxt, len },
                        [
                            117u8, 60u8, 137u8, 159u8, 237u8, 252u8, 216u8, 238u8, 232u8, 1u8,
                            100u8, 152u8, 26u8, 185u8, 145u8, 125u8, 68u8, 189u8, 4u8, 30u8, 125u8,
                            7u8, 196u8, 153u8, 235u8, 51u8, 219u8, 108u8, 185u8, 254u8, 100u8,
                            201u8,
                        ],
                    )
                }
                pub fn query_weight_to_fee(
                    &self,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::runtime_api::Payload<types::QueryWeightToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            206u8, 243u8, 189u8, 83u8, 231u8, 244u8, 247u8, 52u8, 126u8, 208u8,
                            224u8, 5u8, 163u8, 108u8, 254u8, 114u8, 214u8, 156u8, 227u8, 217u8,
                            211u8, 198u8, 121u8, 164u8, 110u8, 54u8, 181u8, 146u8, 50u8, 146u8,
                            146u8, 23u8,
                        ],
                    )
                }
                pub fn query_length_to_fee(
                    &self,
                    length: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<types::QueryLengthToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            92u8, 132u8, 29u8, 119u8, 66u8, 11u8, 196u8, 224u8, 129u8, 23u8, 249u8,
                            12u8, 32u8, 28u8, 92u8, 50u8, 188u8, 101u8, 203u8, 229u8, 248u8, 216u8,
                            130u8, 150u8, 212u8, 161u8, 81u8, 254u8, 116u8, 89u8, 162u8, 48u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryInfo { pub uxt : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > , pub len : :: core :: primitive :: u32 , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryFeeDetails { pub uxt : :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: node_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > , pub len : :: core :: primitive :: u32 , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryWeightToFee {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryLengthToFee {
                    pub length: ::core::primitive::u32,
                }
            }
        }
        pub mod transaction_payment_call_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentCallApi;
            impl TransactionPaymentCallApi {
                #[doc = " Query information of a dispatch class, weight, and fee of a given encoded `Call`."]
                pub fn query_call_info(
                    &self,
                    call: runtime_types::node_template_runtime::RuntimeCall,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryCallInfo,
                    runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                        ::core::primitive::u128,
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_info",
                        types::QueryCallInfo { call, len },
                        [
                            2u8, 48u8, 230u8, 219u8, 120u8, 65u8, 254u8, 102u8, 3u8, 177u8, 168u8,
                            134u8, 240u8, 16u8, 124u8, 79u8, 118u8, 34u8, 234u8, 76u8, 68u8, 15u8,
                            39u8, 87u8, 210u8, 144u8, 214u8, 5u8, 249u8, 158u8, 193u8, 132u8,
                        ],
                    )
                }
                #[doc = " Query fee details of a given encoded `Call`."]
                pub fn query_call_fee_details(
                    &self,
                    call: runtime_types::node_template_runtime::RuntimeCall,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryCallFeeDetails,
                    runtime_types::pallet_transaction_payment::types::FeeDetails<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_fee_details",
                        types::QueryCallFeeDetails { call, len },
                        [
                            197u8, 115u8, 118u8, 43u8, 211u8, 31u8, 228u8, 190u8, 164u8, 253u8,
                            137u8, 199u8, 220u8, 122u8, 246u8, 37u8, 13u8, 98u8, 166u8, 124u8,
                            12u8, 110u8, 169u8, 23u8, 229u8, 128u8, 132u8, 172u8, 14u8, 253u8,
                            150u8, 174u8,
                        ],
                    )
                }
                #[doc = " Query the output of the current `WeightToFee` given some input."]
                pub fn query_weight_to_fee(
                    &self,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::runtime_api::Payload<types::QueryWeightToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            117u8, 91u8, 94u8, 22u8, 248u8, 212u8, 15u8, 23u8, 97u8, 116u8, 64u8,
                            228u8, 83u8, 123u8, 87u8, 77u8, 97u8, 7u8, 98u8, 181u8, 6u8, 165u8,
                            114u8, 141u8, 164u8, 113u8, 126u8, 88u8, 174u8, 171u8, 224u8, 35u8,
                        ],
                    )
                }
                #[doc = " Query the output of the current `LengthToFee` given some input."]
                pub fn query_length_to_fee(
                    &self,
                    length: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<types::QueryLengthToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            246u8, 40u8, 4u8, 160u8, 152u8, 94u8, 170u8, 53u8, 205u8, 122u8, 5u8,
                            69u8, 70u8, 25u8, 128u8, 156u8, 119u8, 134u8, 116u8, 147u8, 14u8,
                            164u8, 65u8, 140u8, 86u8, 13u8, 250u8, 218u8, 89u8, 95u8, 234u8, 228u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryCallInfo {
                    pub call: runtime_types::node_template_runtime::RuntimeCall,
                    pub len: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryCallFeeDetails {
                    pub call: runtime_types::node_template_runtime::RuntimeCall,
                    pub len: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryWeightToFee {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryLengthToFee {
                    pub length: ::core::primitive::u32,
                }
            }
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn node_authorization(&self) -> node_authorization::constants::ConstantsApi {
            node_authorization::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn validator_set(&self) -> validator_set::storage::StorageApi {
            validator_set::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn trading_account(&self) -> trading_account::storage::StorageApi {
            trading_account::storage::StorageApi
        }
        pub fn assets(&self) -> assets::storage::StorageApi {
            assets::storage::StorageApi
        }
        pub fn markets(&self) -> markets::storage::StorageApi {
            markets::storage::StorageApi
        }
        pub fn sync_facade(&self) -> sync_facade::storage::StorageApi {
            sync_facade::storage::StorageApi
        }
        pub fn trading(&self) -> trading::storage::StorageApi {
            trading::storage::StorageApi
        }
        pub fn trading_fees(&self) -> trading_fees::storage::StorageApi {
            trading_fees::storage::StorageApi
        }
        pub fn prices(&self) -> prices::storage::StorageApi {
            prices::storage::StorageApi
        }
        pub fn node_authorization(&self) -> node_authorization::storage::StorageApi {
            node_authorization::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn validator_set(&self) -> validator_set::calls::TransactionApi {
            validator_set::calls::TransactionApi
        }
        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn trading_account(&self) -> trading_account::calls::TransactionApi {
            trading_account::calls::TransactionApi
        }
        pub fn assets(&self) -> assets::calls::TransactionApi {
            assets::calls::TransactionApi
        }
        pub fn markets(&self) -> markets::calls::TransactionApi {
            markets::calls::TransactionApi
        }
        pub fn sync_facade(&self) -> sync_facade::calls::TransactionApi {
            sync_facade::calls::TransactionApi
        }
        pub fn trading(&self) -> trading::calls::TransactionApi {
            trading::calls::TransactionApi
        }
        pub fn trading_fees(&self) -> trading_fees::calls::TransactionApi {
            trading_fees::calls::TransactionApi
        }
        pub fn prices(&self) -> prices::calls::TransactionApi {
            prices::calls::TransactionApi
        }
        pub fn node_authorization(&self) -> node_authorization::calls::TransactionApi {
            node_authorization::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                206u8, 210u8, 12u8, 159u8, 124u8, 234u8, 159u8, 192u8, 166u8, 133u8, 76u8, 243u8,
                3u8, 111u8, 187u8, 164u8, 77u8, 103u8, 89u8, 20u8, 229u8, 236u8, 160u8, 205u8,
                91u8, 48u8, 235u8, 125u8, 56u8, 138u8, 136u8, 224u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Remark {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetHeapPages {
                    pub pages: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCode {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCodeWithoutChecks {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStorage {
                    pub items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillStorage {
                    pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillPrefix {
                    pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                    pub subkeys: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemarkWithEvent {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::remark`]."]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::Remark> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_heap_pages`]."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetHeapPages> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_code`]."]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_code_without_checks`]."]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_storage`]."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::Payload<types::SetStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::kill_storage`]."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::Payload<types::KillStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::kill_prefix`]."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::KillPrefix> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::remark_with_event`]."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::utils::AccountId32,
                pub hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Number",
                        vec![],
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Digest",
                        vec![],
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::node_template_runtime::RuntimeEvent,
                            ::subxt::utils::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Events",
                        vec![],
                        [
                            206u8, 34u8, 193u8, 116u8, 158u8, 28u8, 181u8, 33u8, 201u8, 46u8,
                            154u8, 158u8, 253u8, 5u8, 123u8, 147u8, 204u8, 236u8, 84u8, 89u8,
                            137u8, 245u8, 223u8, 248u8, 23u8, 198u8, 7u8, 33u8, 5u8, 164u8, 236u8,
                            86u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::Phase,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Set {
                    #[codec(compact)]
                    pub now: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::set`]."]
                pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
                    ::subxt::tx::Payload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAllowDeath {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBalanceDeprecated {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                    #[codec(compact)]
                    pub old_reserved: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "set_balance_deprecated";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAll {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub keep_alive: ::core::primitive::bool,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnreserve {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpgradeAccounts {
                    pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Transfer {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSetBalance {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::transfer_allow_death`]."]
                pub fn transfer_allow_death(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_balance_deprecated`]."]
                pub fn set_balance_deprecated(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                    old_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "set_balance_deprecated",
                        types::SetBalanceDeprecated {
                            who,
                            new_free,
                            old_reserved,
                        },
                        [
                            125u8, 171u8, 21u8, 186u8, 108u8, 185u8, 241u8, 145u8, 125u8, 8u8,
                            12u8, 42u8, 96u8, 114u8, 80u8, 80u8, 227u8, 76u8, 20u8, 208u8, 93u8,
                            219u8, 36u8, 50u8, 209u8, 155u8, 70u8, 45u8, 6u8, 57u8, 156u8, 77u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_transfer`]."]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer_keep_alive`]."]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer_all`]."]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::TransferAll> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_unreserve`]."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceUnreserve> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::upgrade_accounts`]."]
                pub fn upgrade_accounts(
                    &self,
                    who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer`]."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer",
                        types::Transfer { dest, value },
                        [
                            154u8, 145u8, 140u8, 54u8, 50u8, 123u8, 225u8, 249u8, 200u8, 217u8,
                            172u8, 110u8, 233u8, 198u8, 77u8, 198u8, 211u8, 89u8, 8u8, 13u8, 240u8,
                            94u8, 28u8, 13u8, 242u8, 217u8, 168u8, 23u8, 106u8, 254u8, 249u8,
                            120u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_set_balance`]."]
                pub fn force_set_balance(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceSetBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::utils::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::utils::AccountId32,
                pub free: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        Vec::new(),
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        Vec::new(),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of holds that can exist on an account at any time."]
                pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxHolds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod validator_set {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::substrate_validator_set::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::substrate_validator_set::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddValidator {
                    pub validator_id: ::subxt::utils::AccountId32,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddValidator {
                    const PALLET: &'static str = "ValidatorSet";
                    const CALL: &'static str = "add_validator";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveValidator {
                    pub validator_id: ::subxt::utils::AccountId32,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveValidator {
                    const PALLET: &'static str = "ValidatorSet";
                    const CALL: &'static str = "remove_validator";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::add_validator`]."]
                pub fn add_validator(
                    &self,
                    validator_id: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::Payload<types::AddValidator> {
                    ::subxt::tx::Payload::new_static(
                        "ValidatorSet",
                        "add_validator",
                        types::AddValidator { validator_id },
                        [
                            126u8, 146u8, 187u8, 183u8, 159u8, 201u8, 223u8, 160u8, 19u8, 193u8,
                            112u8, 251u8, 195u8, 206u8, 104u8, 98u8, 246u8, 104u8, 40u8, 122u8,
                            86u8, 94u8, 97u8, 190u8, 47u8, 207u8, 221u8, 71u8, 119u8, 50u8, 79u8,
                            68u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::remove_validator`]."]
                pub fn remove_validator(
                    &self,
                    validator_id: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::Payload<types::RemoveValidator> {
                    ::subxt::tx::Payload::new_static(
                        "ValidatorSet",
                        "remove_validator",
                        types::RemoveValidator { validator_id },
                        [
                            83u8, 26u8, 215u8, 89u8, 19u8, 91u8, 208u8, 249u8, 201u8, 127u8, 230u8,
                            207u8, 191u8, 215u8, 193u8, 206u8, 55u8, 139u8, 121u8, 133u8, 42u8,
                            97u8, 68u8, 90u8, 210u8, 81u8, 74u8, 126u8, 60u8, 80u8, 242u8, 132u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::substrate_validator_set::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New validator addition initiated. Effective in ~2 sessions."]
            pub struct ValidatorAdditionInitiated(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for ValidatorAdditionInitiated {
                const PALLET: &'static str = "ValidatorSet";
                const EVENT: &'static str = "ValidatorAdditionInitiated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Validator removal initiated. Effective in ~2 sessions."]
            pub struct ValidatorRemovalInitiated(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for ValidatorRemovalInitiated {
                const PALLET: &'static str = "ValidatorSet";
                const EVENT: &'static str = "ValidatorRemovalInitiated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ValidatorSet",
                        "Validators",
                        vec![],
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }
                pub fn offline_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ValidatorSet",
                        "OfflineValidators",
                        vec![],
                        [
                            127u8, 204u8, 6u8, 22u8, 44u8, 16u8, 58u8, 220u8, 168u8, 214u8, 92u8,
                            60u8, 104u8, 206u8, 237u8, 231u8, 39u8, 59u8, 255u8, 82u8, 209u8,
                            236u8, 195u8, 135u8, 128u8, 185u8, 103u8, 140u8, 216u8, 213u8, 249u8,
                            2u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the session pallet."]
        pub type Error = runtime_types::pallet_session::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_session::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetKeys {
                    pub keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                    pub proof: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKeys {
                    const PALLET: &'static str = "Session";
                    const CALL: &'static str = "set_keys";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PurgeKeys;
                impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
                    const PALLET: &'static str = "Session";
                    const CALL: &'static str = "purge_keys";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::set_keys`]."]
                pub fn set_keys(
                    &self,
                    keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetKeys> {
                    ::subxt::tx::Payload::new_static(
                        "Session",
                        "set_keys",
                        types::SetKeys { keys, proof },
                        [
                            47u8, 127u8, 163u8, 217u8, 206u8, 187u8, 133u8, 242u8, 41u8, 220u8,
                            161u8, 23u8, 104u8, 81u8, 53u8, 96u8, 129u8, 183u8, 37u8, 129u8, 178u8,
                            48u8, 192u8, 123u8, 194u8, 58u8, 193u8, 238u8, 114u8, 250u8, 81u8,
                            252u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::purge_keys`]."]
                pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
                    ::subxt::tx::Payload::new_static(
                        "Session",
                        "purge_keys",
                        types::PurgeKeys {},
                        [
                            215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
                            151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
                            67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
                            209u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current set of validators."]
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "Validators",
                        vec![],
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }
                #[doc = " Current index of the session."]
                pub fn current_index(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "CurrentIndex",
                        vec![],
                        [
                            167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
                            135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
                            134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
                            221u8, 230u8,
                        ],
                    )
                }
                #[doc = " True if the underlying economic identities or weighting behind the validators"]
                #[doc = " has changed in the queued validator set."]
                pub fn queued_changed(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "QueuedChanged",
                        vec![],
                        [
                            184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
                            198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
                            36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
                            153u8,
                        ],
                    )
                }
                #[doc = " The queued keys for the next session. When the next session begins, these keys"]
                #[doc = " will be used to determine the validator's session keys."]
                pub fn queued_keys(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "QueuedKeys",
                        vec![],
                        [
                            193u8, 50u8, 14u8, 97u8, 30u8, 205u8, 136u8, 133u8, 46u8, 201u8, 62u8,
                            178u8, 135u8, 253u8, 69u8, 73u8, 216u8, 251u8, 72u8, 93u8, 167u8, 96u8,
                            92u8, 5u8, 183u8, 74u8, 191u8, 28u8, 39u8, 217u8, 60u8, 80u8,
                        ],
                    )
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "DisabledValidators",
                        vec![],
                        [
                            213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
                            36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
                            183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::node_template_runtime::opaque::SessionKeys,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "NextKeys",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            24u8, 204u8, 239u8, 120u8, 213u8, 252u8, 121u8, 93u8, 144u8, 97u8,
                            64u8, 52u8, 105u8, 146u8, 63u8, 76u8, 60u8, 161u8, 70u8, 198u8, 158u8,
                            139u8, 248u8, 134u8, 224u8, 82u8, 251u8, 68u8, 207u8, 88u8, 134u8,
                            238u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::node_template_runtime::opaque::SessionKeys,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "NextKeys",
                        Vec::new(),
                        [
                            24u8, 204u8, 239u8, 120u8, 213u8, 252u8, 121u8, 93u8, 144u8, 97u8,
                            64u8, 52u8, 105u8, 146u8, 63u8, 76u8, 60u8, 161u8, 70u8, 198u8, 158u8,
                            139u8, 248u8, 134u8, 224u8, 82u8, 251u8, 68u8, 207u8, 88u8, 134u8,
                            238u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        Vec::new(),
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "Authorities",
                        vec![],
                        [
                            232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
                            129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
                            94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "CurrentSlot",
                        vec![],
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportEquivocation {
                    pub equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    pub key_owner_proof: runtime_types::sp_core::Void,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportEquivocation {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    pub key_owner_proof: runtime_types::sp_core::Void,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation_unsigned";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct NoteStalled {
                    pub delay: ::core::primitive::u32,
                    pub best_finalized_block_number: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for NoteStalled {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "note_stalled";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::report_equivocation`]."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::Payload<types::ReportEquivocation> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            158u8, 70u8, 189u8, 51u8, 231u8, 191u8, 199u8, 33u8, 64u8, 156u8, 71u8,
                            243u8, 122u8, 199u8, 216u8, 10u8, 45u8, 73u8, 198u8, 141u8, 31u8,
                            209u8, 58u8, 164u8, 219u8, 124u8, 242u8, 26u8, 114u8, 52u8, 65u8,
                            106u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::report_equivocation_unsigned`]."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::Payload<types::ReportEquivocationUnsigned> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            53u8, 23u8, 255u8, 215u8, 105u8, 11u8, 67u8, 177u8, 234u8, 248u8,
                            183u8, 57u8, 230u8, 239u8, 54u8, 238u8, 115u8, 170u8, 153u8, 18u8,
                            55u8, 195u8, 85u8, 98u8, 109u8, 194u8, 57u8, 225u8, 139u8, 237u8,
                            171u8, 152u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::note_stalled`]."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::NoteStalled> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "note_stalled",
                        types::NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            158u8, 25u8, 64u8, 114u8, 131u8, 139u8, 227u8, 132u8, 42u8, 107u8,
                            40u8, 249u8, 18u8, 93u8, 254u8, 86u8, 37u8, 67u8, 250u8, 35u8, 241u8,
                            194u8, 209u8, 20u8, 39u8, 75u8, 186u8, 21u8, 48u8, 124u8, 151u8, 31u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "State",
                        vec![],
                        [
                            73u8, 71u8, 112u8, 83u8, 238u8, 75u8, 44u8, 9u8, 180u8, 33u8, 30u8,
                            121u8, 98u8, 96u8, 61u8, 133u8, 16u8, 70u8, 30u8, 249u8, 34u8, 148u8,
                            15u8, 239u8, 164u8, 157u8, 52u8, 27u8, 144u8, 52u8, 223u8, 109u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "PendingChange",
                        vec![],
                        [
                            150u8, 194u8, 185u8, 248u8, 239u8, 43u8, 141u8, 253u8, 61u8, 106u8,
                            74u8, 164u8, 209u8, 204u8, 206u8, 200u8, 32u8, 38u8, 11u8, 78u8, 84u8,
                            243u8, 181u8, 142u8, 179u8, 151u8, 81u8, 204u8, 244u8, 150u8, 137u8,
                            250u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "NextForced",
                        vec![],
                        [
                            3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
                            141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
                            231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "Stalled",
                        vec![],
                        [
                            6u8, 81u8, 205u8, 142u8, 195u8, 48u8, 0u8, 247u8, 108u8, 170u8, 10u8,
                            249u8, 72u8, 206u8, 32u8, 103u8, 109u8, 57u8, 51u8, 21u8, 144u8, 204u8,
                            79u8, 8u8, 191u8, 185u8, 38u8, 34u8, 118u8, 223u8, 75u8, 241u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        vec![],
                        [
                            234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
                            207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
                            246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
                            115u8, 73u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "SetIdSession",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "SetIdSession",
                        Vec::new(),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of entries to keep in the set id to session index mapping."]
                #[doc = ""]
                #[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
                #[doc = " value should relate to the bonding duration of whatever staking system is"]
                #[doc = " being used (if any). If equivocation handling is not enabled then this value"]
                #[doc = " can be zero."]
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::utils::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u8> {
                    ::subxt::constants::Address::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet"]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Sudo {
                    pub call: ::std::boxed::Box<runtime_types::node_template_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoUncheckedWeight {
                    pub call: ::std::boxed::Box<runtime_types::node_template_runtime::RuntimeCall>,
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetKey {
                    pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoAs {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub call: ::std::boxed::Box<runtime_types::node_template_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::sudo`]."]
                pub fn sudo(
                    &self,
                    call: runtime_types::node_template_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::Sudo> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            179u8, 22u8, 214u8, 155u8, 236u8, 245u8, 120u8, 98u8, 253u8, 224u8,
                            168u8, 6u8, 16u8, 18u8, 186u8, 153u8, 110u8, 222u8, 199u8, 20u8, 104u8,
                            245u8, 212u8, 133u8, 70u8, 85u8, 93u8, 72u8, 209u8, 4u8, 196u8, 112u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::sudo_unchecked_weight`]."]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_template_runtime::RuntimeCall,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            96u8, 55u8, 184u8, 57u8, 250u8, 21u8, 139u8, 66u8, 213u8, 225u8, 98u8,
                            132u8, 25u8, 240u8, 215u8, 56u8, 9u8, 186u8, 72u8, 43u8, 176u8, 41u8,
                            204u8, 22u8, 47u8, 22u8, 135u8, 179u8, 5u8, 77u8, 156u8, 48u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_key`]."]
                pub fn set_key(
                    &self,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::SetKey> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::sudo_as`]."]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call: runtime_types::node_template_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::SudoAs> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            155u8, 94u8, 225u8, 137u8, 250u8, 69u8, 218u8, 218u8, 96u8, 8u8, 190u8,
                            222u8, 109u8, 135u8, 220u8, 196u8, 205u8, 106u8, 99u8, 77u8, 13u8,
                            142u8, 113u8, 17u8, 224u8, 109u8, 151u8, 94u8, 52u8, 110u8, 202u8,
                            205u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod trading_account {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_trading_account::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_trading_account::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateMonetaryToTradingAccounts { pub monetary_accounts : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: MonetaryAccountDetails > , }
                impl ::subxt::blocks::StaticExtrinsic for UpdateMonetaryToTradingAccounts {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "update_monetary_to_trading_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Deposit { pub trading_account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal , pub collateral_id : :: core :: primitive :: u128 , pub amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , }
                impl ::subxt::blocks::StaticExtrinsic for Deposit {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "deposit";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddAccounts { pub accounts : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal > , }
                impl ::subxt::blocks::StaticExtrinsic for AddAccounts {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "add_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBalances {
                    pub account_id: runtime_types::primitive_types::U256,
                    pub balances: ::std::vec::Vec<
                        runtime_types::pallet_support::types::trading_account::BalanceUpdate,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalances {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "set_balances";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStandardWithdrawalFee {
                    pub withdrawal_fee: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStandardWithdrawalFee {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "set_standard_withdrawal_fee";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AdjustBalances {
                    pub start_index: ::core::primitive::u128,
                    pub end_index: ::core::primitive::u128,
                    pub precision: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for AdjustBalances {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "adjust_balances";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Withdraw {
                    pub withdrawal_request:
                        runtime_types::pallet_support::types::trading_account::WithdrawalRequest,
                }
                impl ::subxt::blocks::StaticExtrinsic for Withdraw {
                    const PALLET: &'static str = "TradingAccount";
                    const CALL: &'static str = "withdraw";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::update_monetary_to_trading_accounts`."]
                pub fn update_monetary_to_trading_accounts(
                    &self,
                    monetary_accounts : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: MonetaryAccountDetails >,
                ) -> ::subxt::tx::Payload<types::UpdateMonetaryToTradingAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "update_monetary_to_trading_accounts",
                        types::UpdateMonetaryToTradingAccounts { monetary_accounts },
                        [
                            143u8, 4u8, 250u8, 139u8, 200u8, 255u8, 237u8, 208u8, 174u8, 21u8,
                            66u8, 194u8, 22u8, 78u8, 10u8, 205u8, 169u8, 190u8, 71u8, 205u8, 132u8,
                            93u8, 209u8, 219u8, 155u8, 161u8, 148u8, 21u8, 177u8, 92u8, 126u8,
                            65u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::deposit`."]
                pub fn deposit(
                    &self,
                    trading_account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal,
                    collateral_id: ::core::primitive::u128,
                    amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                ) -> ::subxt::tx::Payload<types::Deposit> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "deposit",
                        types::Deposit {
                            trading_account,
                            collateral_id,
                            amount,
                        },
                        [
                            207u8, 213u8, 208u8, 102u8, 160u8, 176u8, 102u8, 76u8, 205u8, 8u8,
                            11u8, 231u8, 99u8, 246u8, 78u8, 250u8, 179u8, 165u8, 43u8, 97u8, 135u8,
                            95u8, 207u8, 198u8, 13u8, 239u8, 85u8, 119u8, 89u8, 217u8, 253u8,
                            133u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::add_accounts`."]
                pub fn add_accounts(
                    &self,
                    accounts : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal >,
                ) -> ::subxt::tx::Payload<types::AddAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "add_accounts",
                        types::AddAccounts { accounts },
                        [
                            4u8, 190u8, 34u8, 33u8, 23u8, 167u8, 33u8, 77u8, 169u8, 159u8, 200u8,
                            108u8, 194u8, 62u8, 243u8, 164u8, 55u8, 182u8, 34u8, 170u8, 188u8,
                            215u8, 217u8, 105u8, 106u8, 25u8, 126u8, 204u8, 150u8, 142u8, 5u8,
                            95u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_balances`."]
                pub fn set_balances(
                    &self,
                    account_id: runtime_types::primitive_types::U256,
                    balances: ::std::vec::Vec<
                        runtime_types::pallet_support::types::trading_account::BalanceUpdate,
                    >,
                ) -> ::subxt::tx::Payload<types::SetBalances> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "set_balances",
                        types::SetBalances {
                            account_id,
                            balances,
                        },
                        [
                            179u8, 188u8, 139u8, 53u8, 11u8, 172u8, 48u8, 228u8, 150u8, 219u8,
                            72u8, 255u8, 5u8, 193u8, 133u8, 174u8, 43u8, 141u8, 35u8, 125u8, 136u8,
                            179u8, 40u8, 170u8, 131u8, 181u8, 119u8, 46u8, 236u8, 153u8, 223u8,
                            158u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_standard_withdrawal_fee`."]
                pub fn set_standard_withdrawal_fee(
                    &self,
                    withdrawal_fee: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                ) -> ::subxt::tx::Payload<types::SetStandardWithdrawalFee> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "set_standard_withdrawal_fee",
                        types::SetStandardWithdrawalFee { withdrawal_fee },
                        [
                            207u8, 156u8, 158u8, 68u8, 28u8, 98u8, 227u8, 81u8, 63u8, 64u8, 185u8,
                            15u8, 192u8, 131u8, 141u8, 65u8, 60u8, 44u8, 1u8, 47u8, 206u8, 57u8,
                            204u8, 238u8, 5u8, 159u8, 39u8, 201u8, 103u8, 188u8, 67u8, 231u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::adjust_balances`."]
                pub fn adjust_balances(
                    &self,
                    start_index: ::core::primitive::u128,
                    end_index: ::core::primitive::u128,
                    precision: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::AdjustBalances> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "adjust_balances",
                        types::AdjustBalances {
                            start_index,
                            end_index,
                            precision,
                        },
                        [
                            193u8, 28u8, 82u8, 184u8, 138u8, 72u8, 181u8, 23u8, 188u8, 173u8, 91u8,
                            207u8, 128u8, 159u8, 225u8, 172u8, 240u8, 34u8, 167u8, 70u8, 208u8,
                            181u8, 231u8, 35u8, 133u8, 18u8, 251u8, 194u8, 8u8, 217u8, 208u8,
                            123u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::withdraw`."]
                pub fn withdraw(
                    &self,
                    withdrawal_request : runtime_types :: pallet_support :: types :: trading_account :: WithdrawalRequest,
                ) -> ::subxt::tx::Payload<types::Withdraw> {
                    ::subxt::tx::Payload::new_static(
                        "TradingAccount",
                        "withdraw",
                        types::Withdraw { withdrawal_request },
                        [
                            46u8, 126u8, 154u8, 188u8, 156u8, 210u8, 224u8, 30u8, 219u8, 144u8,
                            205u8, 222u8, 134u8, 28u8, 114u8, 172u8, 232u8, 114u8, 7u8, 7u8, 150u8,
                            122u8, 193u8, 69u8, 254u8, 189u8, 72u8, 76u8, 231u8, 170u8, 214u8,
                            34u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_trading_account::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Several accounts added"]
            pub struct AccountsAdded {
                pub length: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AccountsAdded {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "AccountsAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Balances for an account updated"]
            pub struct BalanceUpdated {
                pub account_id: runtime_types::primitive_types::U256,
                pub account:
                    runtime_types::pallet_support::types::trading_account::TradingAccountMinimal,
                pub collateral_id: ::core::primitive::u128,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub modify_type: ::core::primitive::u8,
                pub reason: ::core::primitive::u8,
                pub previous_balance: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub new_balance: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub block_number: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for BalanceUpdated {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "BalanceUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Event emitted for deferred deposits"]
            pub struct DeferredBalance {
                pub account_id: runtime_types::primitive_types::U256,
                pub collateral_id: ::core::primitive::u128,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for DeferredBalance {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "DeferredBalance";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Event to be synced by L2, for pnl changes"]
            pub struct UserBalanceChange {
                pub trading_account:
                    runtime_types::pallet_support::types::trading_account::TradingAccountMinimal,
                pub collateral_id: ::core::primitive::u128,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub modify_type: runtime_types::pallet_support::types::trading::FundModifyType,
                pub reason: ::core::primitive::u8,
                pub block_number: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for UserBalanceChange {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "UserBalanceChange";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Event to be synced by L2, for withdrawal requests"]
            pub struct UserWithdrawal {
                pub trading_account:
                    runtime_types::pallet_support::types::trading_account::TradingAccountMinimal,
                pub collateral_id: ::core::primitive::u128,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub block_number: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for UserWithdrawal {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "UserWithdrawal";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Account created"]
            pub struct AccountCreated {
                pub account_id: runtime_types::primitive_types::U256,
                pub account_address: runtime_types::primitive_types::U256,
                pub index: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for AccountCreated {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "AccountCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Amount passed to transfer/transfer_from functions is negative"]
            pub struct AmountIsNegative {
                pub account_id: runtime_types::primitive_types::U256,
                pub collateral_id: ::core::primitive::u128,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub reason: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for AmountIsNegative {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "AmountIsNegative";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Insurance fund updation event"]
            pub struct InsuranceFundChange {
                pub collateral_id: ::core::primitive::u128,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub modify_type: runtime_types::pallet_support::types::trading::FundModifyType,
                pub block_number: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for InsuranceFundChange {
                const PALLET: &'static str = "TradingAccount";
                const EVENT: &'static str = "InsuranceFundChange";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn accounts_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountsCount",
                        vec![],
                        [
                            67u8, 222u8, 131u8, 150u8, 189u8, 53u8, 123u8, 141u8, 107u8, 29u8,
                            231u8, 20u8, 77u8, 13u8, 180u8, 14u8, 13u8, 142u8, 199u8, 41u8, 167u8,
                            169u8, 60u8, 181u8, 106u8, 110u8, 252u8, 248u8, 110u8, 162u8, 46u8,
                            108u8,
                        ],
                    )
                }
                pub fn standard_withdrawal_fee(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "StandardWithdrawalFee",
                        vec![],
                        [
                            191u8, 143u8, 82u8, 19u8, 168u8, 214u8, 182u8, 149u8, 2u8, 42u8, 164u8,
                            15u8, 46u8, 18u8, 70u8, 91u8, 26u8, 6u8, 120u8, 112u8, 190u8, 236u8,
                            61u8, 75u8, 252u8, 246u8, 152u8, 196u8, 13u8, 244u8, 164u8, 88u8,
                        ],
                    )
                }
                pub fn account_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading_account::TradingAccount,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            47u8, 90u8, 93u8, 249u8, 213u8, 22u8, 15u8, 233u8, 164u8, 151u8, 189u8,
                            215u8, 1u8, 150u8, 65u8, 86u8, 203u8, 192u8, 87u8, 18u8, 144u8, 53u8,
                            70u8, 15u8, 9u8, 196u8, 188u8, 225u8, 103u8, 109u8, 247u8, 36u8,
                        ],
                    )
                }
                pub fn account_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading_account::TradingAccount,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountMap",
                        Vec::new(),
                        [
                            47u8, 90u8, 93u8, 249u8, 213u8, 22u8, 15u8, 233u8, 164u8, 151u8, 189u8,
                            215u8, 1u8, 150u8, 65u8, 86u8, 203u8, 192u8, 87u8, 18u8, 144u8, 53u8,
                            70u8, 15u8, 9u8, 196u8, 188u8, 225u8, 103u8, 109u8, 247u8, 36u8,
                        ],
                    )
                }
                pub fn monetary_account_volume_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_arithmetic::fixed_point::FixedI128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "MonetaryAccountVolumeMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            201u8, 88u8, 114u8, 226u8, 35u8, 203u8, 136u8, 69u8, 189u8, 216u8,
                            252u8, 184u8, 179u8, 56u8, 242u8, 134u8, 18u8, 197u8, 242u8, 135u8,
                            123u8, 7u8, 175u8, 35u8, 141u8, 14u8, 202u8, 116u8, 183u8, 176u8,
                            106u8, 104u8,
                        ],
                    )
                }
                pub fn monetary_account_volume_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_arithmetic::fixed_point::FixedI128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "MonetaryAccountVolumeMap",
                        Vec::new(),
                        [
                            201u8, 88u8, 114u8, 226u8, 35u8, 203u8, 136u8, 69u8, 189u8, 216u8,
                            252u8, 184u8, 179u8, 56u8, 242u8, 134u8, 18u8, 197u8, 242u8, 135u8,
                            123u8, 7u8, 175u8, 35u8, 141u8, 14u8, 202u8, 116u8, 183u8, 176u8,
                            106u8, 104u8,
                        ],
                    )
                }
                pub fn monetary_account_last_tx_timestamp(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "MonetaryAccountLastTxTimestamp",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            240u8, 237u8, 78u8, 56u8, 39u8, 216u8, 222u8, 147u8, 213u8, 2u8, 178u8,
                            78u8, 221u8, 108u8, 250u8, 75u8, 15u8, 173u8, 16u8, 46u8, 71u8, 50u8,
                            156u8, 93u8, 177u8, 50u8, 204u8, 27u8, 95u8, 224u8, 74u8, 250u8,
                        ],
                    )
                }
                pub fn monetary_account_last_tx_timestamp_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "MonetaryAccountLastTxTimestamp",
                        Vec::new(),
                        [
                            240u8, 237u8, 78u8, 56u8, 39u8, 216u8, 222u8, 147u8, 213u8, 2u8, 178u8,
                            78u8, 221u8, 108u8, 250u8, 75u8, 15u8, 173u8, 16u8, 46u8, 71u8, 50u8,
                            156u8, 93u8, 177u8, 50u8, 204u8, 27u8, 95u8, 224u8, 74u8, 250u8,
                        ],
                    )
                }
                pub fn is_withdrawal_processed(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "IsWithdrawalProcessed",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            46u8, 225u8, 112u8, 253u8, 40u8, 142u8, 207u8, 133u8, 221u8, 3u8,
                            226u8, 216u8, 165u8, 191u8, 24u8, 17u8, 70u8, 156u8, 20u8, 84u8, 221u8,
                            74u8, 156u8, 114u8, 81u8, 155u8, 39u8, 128u8, 15u8, 171u8, 190u8, 18u8,
                        ],
                    )
                }
                pub fn is_withdrawal_processed_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "IsWithdrawalProcessed",
                        Vec::new(),
                        [
                            46u8, 225u8, 112u8, 253u8, 40u8, 142u8, 207u8, 133u8, 221u8, 3u8,
                            226u8, 216u8, 165u8, 191u8, 24u8, 17u8, 70u8, 156u8, 20u8, 84u8, 221u8,
                            74u8, 156u8, 114u8, 81u8, 155u8, 39u8, 128u8, 15u8, 171u8, 190u8, 18u8,
                        ],
                    )
                }
                pub fn accounts_list_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountsListMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            170u8, 131u8, 60u8, 42u8, 182u8, 159u8, 250u8, 93u8, 102u8, 245u8,
                            183u8, 115u8, 86u8, 128u8, 138u8, 68u8, 8u8, 201u8, 104u8, 147u8, 87u8,
                            175u8, 214u8, 198u8, 10u8, 135u8, 212u8, 221u8, 131u8, 45u8, 9u8,
                            117u8,
                        ],
                    )
                }
                pub fn accounts_list_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountsListMap",
                        Vec::new(),
                        [
                            170u8, 131u8, 60u8, 42u8, 182u8, 159u8, 250u8, 93u8, 102u8, 245u8,
                            183u8, 115u8, 86u8, 128u8, 138u8, 68u8, 8u8, 201u8, 104u8, 147u8, 87u8,
                            175u8, 214u8, 198u8, 10u8, 135u8, 212u8, 221u8, 131u8, 45u8, 9u8,
                            117u8,
                        ],
                    )
                }
                pub fn monetary_to_trading_accounts_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "MonetaryToTradingAccountsMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            192u8, 219u8, 129u8, 234u8, 215u8, 150u8, 187u8, 208u8, 131u8, 75u8,
                            236u8, 131u8, 55u8, 49u8, 11u8, 191u8, 112u8, 180u8, 19u8, 186u8, 88u8,
                            191u8, 20u8, 52u8, 36u8, 202u8, 83u8, 212u8, 32u8, 194u8, 196u8, 74u8,
                        ],
                    )
                }
                pub fn monetary_to_trading_accounts_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "MonetaryToTradingAccountsMap",
                        Vec::new(),
                        [
                            192u8, 219u8, 129u8, 234u8, 215u8, 150u8, 187u8, 208u8, 131u8, 75u8,
                            236u8, 131u8, 55u8, 49u8, 11u8, 191u8, 112u8, 180u8, 19u8, 186u8, 88u8,
                            191u8, 20u8, 52u8, 36u8, 202u8, 83u8, 212u8, 32u8, 194u8, 196u8, 74u8,
                        ],
                    )
                }
                pub fn balances_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "BalancesMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            210u8, 138u8, 230u8, 211u8, 179u8, 26u8, 245u8, 4u8, 234u8, 239u8,
                            178u8, 107u8, 247u8, 208u8, 200u8, 163u8, 230u8, 183u8, 115u8, 186u8,
                            100u8, 189u8, 241u8, 127u8, 121u8, 78u8, 161u8, 54u8, 157u8, 148u8,
                            69u8, 12u8,
                        ],
                    )
                }
                pub fn balances_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "BalancesMap",
                        Vec::new(),
                        [
                            210u8, 138u8, 230u8, 211u8, 179u8, 26u8, 245u8, 4u8, 234u8, 239u8,
                            178u8, 107u8, 247u8, 208u8, 200u8, 163u8, 230u8, 183u8, 115u8, 186u8,
                            100u8, 189u8, 241u8, 127u8, 121u8, 78u8, 161u8, 54u8, 157u8, 148u8,
                            69u8, 12u8,
                        ],
                    )
                }
                pub fn deferred_balances_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "DeferredBalancesMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            198u8, 251u8, 217u8, 57u8, 130u8, 65u8, 217u8, 105u8, 44u8, 126u8,
                            47u8, 26u8, 53u8, 46u8, 231u8, 68u8, 2u8, 182u8, 230u8, 42u8, 103u8,
                            121u8, 209u8, 33u8, 66u8, 181u8, 1u8, 172u8, 189u8, 55u8, 30u8, 166u8,
                        ],
                    )
                }
                pub fn deferred_balances_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "DeferredBalancesMap",
                        Vec::new(),
                        [
                            198u8, 251u8, 217u8, 57u8, 130u8, 65u8, 217u8, 105u8, 44u8, 126u8,
                            47u8, 26u8, 53u8, 46u8, 231u8, 68u8, 2u8, 182u8, 230u8, 42u8, 103u8,
                            121u8, 209u8, 33u8, 66u8, 181u8, 1u8, 172u8, 189u8, 55u8, 30u8, 166u8,
                        ],
                    )
                }
                pub fn locked_margin_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "LockedMarginMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            219u8, 157u8, 208u8, 85u8, 5u8, 134u8, 176u8, 48u8, 20u8, 34u8, 32u8,
                            47u8, 48u8, 137u8, 15u8, 248u8, 178u8, 145u8, 202u8, 132u8, 235u8,
                            135u8, 10u8, 155u8, 98u8, 113u8, 185u8, 215u8, 253u8, 110u8, 92u8, 2u8,
                        ],
                    )
                }
                pub fn locked_margin_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "LockedMarginMap",
                        Vec::new(),
                        [
                            219u8, 157u8, 208u8, 85u8, 5u8, 134u8, 176u8, 48u8, 20u8, 34u8, 32u8,
                            47u8, 48u8, 137u8, 15u8, 248u8, 178u8, 145u8, 202u8, 132u8, 235u8,
                            135u8, 10u8, 155u8, 98u8, 113u8, 185u8, 215u8, 253u8, 110u8, 92u8, 2u8,
                        ],
                    )
                }
                pub fn account_collaterals_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountCollateralsMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            172u8, 241u8, 100u8, 100u8, 5u8, 116u8, 162u8, 188u8, 50u8, 69u8,
                            116u8, 13u8, 229u8, 14u8, 40u8, 179u8, 181u8, 117u8, 136u8, 115u8, 3u8,
                            224u8, 212u8, 216u8, 105u8, 40u8, 125u8, 245u8, 254u8, 203u8, 66u8,
                            74u8,
                        ],
                    )
                }
                pub fn account_collaterals_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingAccount",
                        "AccountCollateralsMap",
                        Vec::new(),
                        [
                            172u8, 241u8, 100u8, 100u8, 5u8, 116u8, 162u8, 188u8, 50u8, 69u8,
                            116u8, 13u8, 229u8, 14u8, 40u8, 179u8, 181u8, 117u8, 136u8, 115u8, 3u8,
                            224u8, 212u8, 216u8, 105u8, 40u8, 125u8, 245u8, 254u8, 203u8, 66u8,
                            74u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod assets {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_asset::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_asset::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ConsumeNonce;
                impl ::subxt::blocks::StaticExtrinsic for ConsumeNonce {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "consume_nonce";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReplaceAllAssets {
                    pub assets:
                        ::std::vec::Vec<runtime_types::pallet_support::types::asset::ExtendedAsset>,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReplaceAllAssets {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "replace_all_assets";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveAsset {
                    pub id: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "remove_asset";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateAsset {
                    pub extended_asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "update_asset";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddAsset {
                    pub extended_asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "add_asset";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::consume_nonce`."]
                pub fn consume_nonce(&self) -> ::subxt::tx::Payload<types::ConsumeNonce> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "consume_nonce",
                        types::ConsumeNonce {},
                        [
                            155u8, 200u8, 35u8, 133u8, 125u8, 113u8, 58u8, 156u8, 98u8, 98u8,
                            216u8, 158u8, 105u8, 247u8, 24u8, 50u8, 212u8, 2u8, 65u8, 206u8, 198u8,
                            118u8, 176u8, 252u8, 72u8, 12u8, 169u8, 165u8, 114u8, 60u8, 122u8,
                            40u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::replace_all_assets`."]
                pub fn replace_all_assets(
                    &self,
                    assets: ::std::vec::Vec<
                        runtime_types::pallet_support::types::asset::ExtendedAsset,
                    >,
                ) -> ::subxt::tx::Payload<types::ReplaceAllAssets> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "replace_all_assets",
                        types::ReplaceAllAssets { assets },
                        [
                            54u8, 186u8, 225u8, 207u8, 143u8, 206u8, 60u8, 152u8, 255u8, 142u8,
                            138u8, 116u8, 108u8, 59u8, 146u8, 99u8, 4u8, 238u8, 119u8, 11u8, 212u8,
                            228u8, 13u8, 136u8, 74u8, 165u8, 76u8, 237u8, 32u8, 130u8, 47u8, 169u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::remove_asset`."]
                pub fn remove_asset(
                    &self,
                    id: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::RemoveAsset> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "remove_asset",
                        types::RemoveAsset { id },
                        [
                            102u8, 18u8, 234u8, 125u8, 107u8, 139u8, 110u8, 150u8, 185u8, 60u8,
                            5u8, 146u8, 219u8, 28u8, 235u8, 38u8, 134u8, 212u8, 8u8, 241u8, 9u8,
                            241u8, 163u8, 153u8, 30u8, 53u8, 27u8, 83u8, 120u8, 135u8, 64u8, 131u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::update_asset`."]
                pub fn update_asset(
                    &self,
                    extended_asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                ) -> ::subxt::tx::Payload<types::UpdateAsset> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "update_asset",
                        types::UpdateAsset { extended_asset },
                        [
                            201u8, 165u8, 205u8, 135u8, 194u8, 248u8, 230u8, 195u8, 98u8, 164u8,
                            229u8, 59u8, 135u8, 63u8, 145u8, 160u8, 43u8, 186u8, 173u8, 185u8,
                            152u8, 136u8, 74u8, 38u8, 55u8, 122u8, 45u8, 241u8, 175u8, 51u8, 125u8,
                            213u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::add_asset`."]
                pub fn add_asset(
                    &self,
                    extended_asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                ) -> ::subxt::tx::Payload<types::AddAsset> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "add_asset",
                        types::AddAsset { extended_asset },
                        [
                            221u8, 232u8, 93u8, 35u8, 100u8, 198u8, 143u8, 70u8, 152u8, 110u8,
                            144u8, 236u8, 249u8, 180u8, 5u8, 150u8, 196u8, 8u8, 122u8, 60u8, 234u8,
                            229u8, 137u8, 97u8, 147u8, 177u8, 128u8, 47u8, 146u8, 251u8, 124u8,
                            24u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_asset::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AssetCreated {
                pub asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
            }
            impl ::subxt::events::StaticEvent for AssetCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AssetUpdated {
                pub asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
            }
            impl ::subxt::events::StaticEvent for AssetUpdated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AssetRemoved {
                pub asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
            }
            impl ::subxt::events::StaticEvent for AssetRemoved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Stores the number of valid assets in the system"]
                pub fn assets_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "AssetsCount",
                        vec![],
                        [
                            124u8, 245u8, 208u8, 80u8, 121u8, 62u8, 92u8, 74u8, 166u8, 51u8, 113u8,
                            122u8, 234u8, 220u8, 204u8, 176u8, 119u8, 233u8, 66u8, 77u8, 100u8,
                            113u8, 166u8, 91u8, 140u8, 231u8, 12u8, 41u8, 111u8, 185u8, 212u8,
                            213u8,
                        ],
                    )
                }
                #[doc = " Maps the Assets struct to the unique_id."]
                pub fn asset_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::asset::ExtendedAsset,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "AssetMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            0u8, 175u8, 60u8, 133u8, 249u8, 61u8, 117u8, 67u8, 105u8, 163u8, 44u8,
                            36u8, 63u8, 193u8, 25u8, 51u8, 154u8, 64u8, 252u8, 100u8, 41u8, 145u8,
                            28u8, 156u8, 78u8, 66u8, 40u8, 53u8, 152u8, 254u8, 241u8, 34u8,
                        ],
                    )
                }
                #[doc = " Maps the Assets struct to the unique_id."]
                pub fn asset_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::asset::ExtendedAsset,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "AssetMap",
                        Vec::new(),
                        [
                            0u8, 175u8, 60u8, 133u8, 249u8, 61u8, 117u8, 67u8, 105u8, 163u8, 44u8,
                            36u8, 63u8, 193u8, 25u8, 51u8, 154u8, 64u8, 252u8, 100u8, 41u8, 145u8,
                            28u8, 156u8, 78u8, 66u8, 40u8, 53u8, 152u8, 254u8, 241u8, 34u8,
                        ],
                    )
                }
                #[doc = " Stores the default collateral in the system"]
                pub fn default_collateral_asset(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u8,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "DefaultCollateralAsset",
                        vec![],
                        [
                            205u8, 46u8, 86u8, 253u8, 98u8, 213u8, 123u8, 18u8, 50u8, 100u8, 226u8,
                            89u8, 165u8, 218u8, 157u8, 139u8, 80u8, 167u8, 128u8, 83u8, 182u8,
                            199u8, 64u8, 18u8, 60u8, 44u8, 227u8, 213u8, 76u8, 26u8, 18u8, 246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod markets {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_market::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_market::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReplaceAllMarkets {
                    pub markets: ::std::vec::Vec<
                        runtime_types::pallet_support::types::market::ExtendedMarket,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReplaceAllMarkets {
                    const PALLET: &'static str = "Markets";
                    const CALL: &'static str = "replace_all_markets";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddMarket {
                    pub extended_market:
                        runtime_types::pallet_support::types::market::ExtendedMarket,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddMarket {
                    const PALLET: &'static str = "Markets";
                    const CALL: &'static str = "add_market";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateMarket {
                    pub extended_market:
                        runtime_types::pallet_support::types::market::ExtendedMarket,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateMarket {
                    const PALLET: &'static str = "Markets";
                    const CALL: &'static str = "update_market";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveMarket {
                    pub id: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveMarket {
                    const PALLET: &'static str = "Markets";
                    const CALL: &'static str = "remove_market";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::replace_all_markets`."]
                pub fn replace_all_markets(
                    &self,
                    markets: ::std::vec::Vec<
                        runtime_types::pallet_support::types::market::ExtendedMarket,
                    >,
                ) -> ::subxt::tx::Payload<types::ReplaceAllMarkets> {
                    ::subxt::tx::Payload::new_static(
                        "Markets",
                        "replace_all_markets",
                        types::ReplaceAllMarkets { markets },
                        [
                            206u8, 155u8, 20u8, 66u8, 33u8, 234u8, 208u8, 187u8, 227u8, 146u8,
                            126u8, 128u8, 64u8, 82u8, 35u8, 14u8, 223u8, 254u8, 189u8, 254u8,
                            124u8, 30u8, 149u8, 94u8, 167u8, 154u8, 24u8, 79u8, 12u8, 115u8, 157u8,
                            125u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::add_market`."]
                pub fn add_market(
                    &self,
                    extended_market: runtime_types::pallet_support::types::market::ExtendedMarket,
                ) -> ::subxt::tx::Payload<types::AddMarket> {
                    ::subxt::tx::Payload::new_static(
                        "Markets",
                        "add_market",
                        types::AddMarket { extended_market },
                        [
                            156u8, 10u8, 54u8, 110u8, 83u8, 160u8, 207u8, 191u8, 3u8, 146u8, 164u8,
                            105u8, 185u8, 98u8, 138u8, 100u8, 45u8, 134u8, 168u8, 98u8, 254u8,
                            81u8, 146u8, 18u8, 30u8, 168u8, 70u8, 175u8, 190u8, 102u8, 207u8,
                            170u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::update_market`."]
                pub fn update_market(
                    &self,
                    extended_market: runtime_types::pallet_support::types::market::ExtendedMarket,
                ) -> ::subxt::tx::Payload<types::UpdateMarket> {
                    ::subxt::tx::Payload::new_static(
                        "Markets",
                        "update_market",
                        types::UpdateMarket { extended_market },
                        [
                            108u8, 13u8, 3u8, 249u8, 164u8, 51u8, 242u8, 86u8, 35u8, 96u8, 207u8,
                            169u8, 32u8, 185u8, 156u8, 145u8, 129u8, 39u8, 193u8, 201u8, 5u8,
                            173u8, 135u8, 138u8, 210u8, 247u8, 81u8, 99u8, 24u8, 185u8, 171u8,
                            58u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::remove_market`."]
                pub fn remove_market(
                    &self,
                    id: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::RemoveMarket> {
                    ::subxt::tx::Payload::new_static(
                        "Markets",
                        "remove_market",
                        types::RemoveMarket { id },
                        [
                            12u8, 178u8, 229u8, 214u8, 88u8, 155u8, 112u8, 25u8, 165u8, 62u8,
                            170u8, 18u8, 255u8, 15u8, 196u8, 106u8, 159u8, 139u8, 47u8, 160u8,
                            187u8, 0u8, 18u8, 95u8, 26u8, 223u8, 227u8, 152u8, 234u8, 199u8, 37u8,
                            54u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_market::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Market successfully created"]
            pub struct MarketCreated {
                pub market: runtime_types::pallet_support::types::market::ExtendedMarket,
            }
            impl ::subxt::events::StaticEvent for MarketCreated {
                const PALLET: &'static str = "Markets";
                const EVENT: &'static str = "MarketCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Market successfully updated"]
            pub struct MarketUpdated {
                pub market: runtime_types::pallet_support::types::market::ExtendedMarket,
            }
            impl ::subxt::events::StaticEvent for MarketUpdated {
                const PALLET: &'static str = "Markets";
                const EVENT: &'static str = "MarketUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Market successfully removed"]
            pub struct MarketRemoved {
                pub market: runtime_types::pallet_support::types::market::ExtendedMarket,
            }
            impl ::subxt::events::StaticEvent for MarketRemoved {
                const PALLET: &'static str = "Markets";
                const EVENT: &'static str = "MarketRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn markets_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Markets",
                        "MarketsCount",
                        vec![],
                        [
                            223u8, 194u8, 191u8, 187u8, 152u8, 104u8, 167u8, 122u8, 119u8, 145u8,
                            126u8, 79u8, 159u8, 190u8, 152u8, 173u8, 165u8, 74u8, 52u8, 240u8,
                            126u8, 148u8, 46u8, 157u8, 13u8, 142u8, 84u8, 136u8, 113u8, 218u8,
                            223u8, 240u8,
                        ],
                    )
                }
                #[doc = " Maps the Market struct to the unique_id."]
                pub fn market_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::market::ExtendedMarket,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Markets",
                        "MarketMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            40u8, 93u8, 20u8, 159u8, 16u8, 23u8, 34u8, 207u8, 64u8, 77u8, 159u8,
                            241u8, 70u8, 96u8, 235u8, 140u8, 218u8, 82u8, 176u8, 97u8, 27u8, 10u8,
                            191u8, 17u8, 185u8, 112u8, 205u8, 115u8, 27u8, 84u8, 0u8, 55u8,
                        ],
                    )
                }
                #[doc = " Maps the Market struct to the unique_id."]
                pub fn market_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::market::ExtendedMarket,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Markets",
                        "MarketMap",
                        Vec::new(),
                        [
                            40u8, 93u8, 20u8, 159u8, 16u8, 23u8, 34u8, 207u8, 64u8, 77u8, 159u8,
                            241u8, 70u8, 96u8, 235u8, 140u8, 218u8, 82u8, 176u8, 97u8, 27u8, 10u8,
                            191u8, 17u8, 185u8, 112u8, 205u8, 115u8, 27u8, 84u8, 0u8, 55u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sync_facade {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_sync_facade::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_sync_facade::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddSigner {
                    pub pub_key: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddSigner {
                    const PALLET: &'static str = "SyncFacade";
                    const CALL: &'static str = "add_signer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetSignersQuorum {
                    pub new_quorum: ::core::primitive::u8,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetSignersQuorum {
                    const PALLET: &'static str = "SyncFacade";
                    const CALL: &'static str = "set_signers_quorum";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveSigner {
                    pub pub_key: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveSigner {
                    const PALLET: &'static str = "SyncFacade";
                    const CALL: &'static str = "remove_signer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SynchronizeEvents {
                    pub events_batch: ::std::vec::Vec<
                        runtime_types::pallet_support::types::sync_facade::UniversalEvent,
                    >,
                    pub signatures: ::std::vec::Vec<
                        runtime_types::pallet_support::types::sync_facade::SyncSignature,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for SynchronizeEvents {
                    const PALLET: &'static str = "SyncFacade";
                    const CALL: &'static str = "synchronize_events";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::add_signer`."]
                pub fn add_signer(
                    &self,
                    pub_key: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::AddSigner> {
                    ::subxt::tx::Payload::new_static(
                        "SyncFacade",
                        "add_signer",
                        types::AddSigner { pub_key },
                        [
                            180u8, 198u8, 130u8, 243u8, 183u8, 45u8, 61u8, 79u8, 213u8, 93u8,
                            104u8, 145u8, 176u8, 168u8, 116u8, 36u8, 190u8, 185u8, 141u8, 46u8,
                            45u8, 167u8, 107u8, 143u8, 152u8, 33u8, 46u8, 88u8, 204u8, 79u8, 225u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_signers_quorum`."]
                pub fn set_signers_quorum(
                    &self,
                    new_quorum: ::core::primitive::u8,
                ) -> ::subxt::tx::Payload<types::SetSignersQuorum> {
                    ::subxt::tx::Payload::new_static(
                        "SyncFacade",
                        "set_signers_quorum",
                        types::SetSignersQuorum { new_quorum },
                        [
                            249u8, 149u8, 170u8, 4u8, 147u8, 229u8, 198u8, 134u8, 250u8, 91u8,
                            199u8, 79u8, 54u8, 231u8, 103u8, 239u8, 87u8, 115u8, 150u8, 52u8,
                            201u8, 108u8, 22u8, 4u8, 96u8, 178u8, 97u8, 138u8, 72u8, 253u8, 32u8,
                            160u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::remove_signer`."]
                pub fn remove_signer(
                    &self,
                    pub_key: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::RemoveSigner> {
                    ::subxt::tx::Payload::new_static(
                        "SyncFacade",
                        "remove_signer",
                        types::RemoveSigner { pub_key },
                        [
                            24u8, 204u8, 222u8, 11u8, 169u8, 219u8, 68u8, 35u8, 157u8, 81u8, 73u8,
                            48u8, 24u8, 160u8, 190u8, 139u8, 63u8, 253u8, 218u8, 238u8, 142u8,
                            148u8, 65u8, 32u8, 228u8, 50u8, 20u8, 54u8, 119u8, 118u8, 187u8, 154u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::synchronize_events`."]
                pub fn synchronize_events(
                    &self,
                    events_batch: ::std::vec::Vec<
                        runtime_types::pallet_support::types::sync_facade::UniversalEvent,
                    >,
                    signatures: ::std::vec::Vec<
                        runtime_types::pallet_support::types::sync_facade::SyncSignature,
                    >,
                ) -> ::subxt::tx::Payload<types::SynchronizeEvents> {
                    ::subxt::tx::Payload::new_static(
                        "SyncFacade",
                        "synchronize_events",
                        types::SynchronizeEvents {
                            events_batch,
                            signatures,
                        },
                        [
                            21u8, 111u8, 202u8, 58u8, 240u8, 181u8, 1u8, 180u8, 201u8, 146u8,
                            249u8, 28u8, 241u8, 155u8, 96u8, 160u8, 56u8, 11u8, 44u8, 185u8, 67u8,
                            65u8, 15u8, 228u8, 134u8, 91u8, 238u8, 38u8, 105u8, 77u8, 80u8, 88u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sync_facade::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Signer added by the admin successfully"]
            pub struct SignerAdded {
                pub signer: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for SignerAdded {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "SignerAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Signer removed by the admin succesfully"]
            pub struct SignerRemoved {
                pub signer: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for SignerRemoved {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "SignerRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New Quorum requirement set by the admin"]
            pub struct QuorumSet {
                pub quorum: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for QuorumSet {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "QuorumSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A invalid request to remove non-existent market"]
            pub struct MarketRemovedError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for MarketRemovedError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "MarketRemovedError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to remove non-existent asset"]
            pub struct AssetRemovedError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AssetRemovedError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "AssetRemovedError";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to add a duplicate signer"]
            pub struct SignerAddedError {
                pub pub_key: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for SignerAddedError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "SignerAddedError";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to remove non-existent signer"]
            pub struct SignerRemovedError {
                pub pub_key: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for SignerRemovedError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "SignerRemovedError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to remove signer; leads to insufficient signers"]
            pub struct SignerRemovedQuorumError {
                pub quorum: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for SignerRemovedQuorumError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "SignerRemovedQuorumError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to set a signer"]
            pub struct QuorumSetError {
                pub quorum: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for QuorumSetError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "QuorumSetError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request for user deposit"]
            pub struct UserDepositError {
                pub collateral_id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for UserDepositError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "UserDepositError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid key in settings"]
            pub struct SettingsKeyError {
                pub key: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SettingsKeyError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "SettingsKeyError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Insufficient data for setting fees"]
            pub struct InsufficientFeeData {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for InsufficientFeeData {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "InsufficientFeeData";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Fee data length mismatch"]
            pub struct FeeDataLengthMismatch {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for FeeDataLengthMismatch {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "FeeDataLengthMismatch";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Token parsing error"]
            pub struct TokenParsingError {
                pub key: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for TokenParsingError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "TokenParsingError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to add an asset"]
            pub struct AddAssetError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AddAssetError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "AddAssetError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to update an asset"]
            pub struct UpdateAssetError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for UpdateAssetError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "UpdateAssetError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid requet to add a market"]
            pub struct AddMarketError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AddMarketError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "AddMarketError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to update a market"]
            pub struct UpdateMarketError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for UpdateMarketError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "UpdateMarketError";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An unknown asset/market id passed"]
            pub struct UnknownIdForFees {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for UnknownIdForFees {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "UnknownIdForFees";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid request to set max abr"]
            pub struct InvalidMarket {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for InvalidMarket {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "InvalidMarket";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A max abr request with empty array"]
            pub struct EmptyValuesError {
                pub id: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for EmptyValuesError {
                const PALLET: &'static str = "SyncFacade";
                const EVENT: &'static str = "EmptyValuesError";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn signers(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "Signers",
                        vec![],
                        [
                            125u8, 177u8, 211u8, 107u8, 175u8, 77u8, 19u8, 228u8, 5u8, 73u8, 236u8,
                            73u8, 128u8, 81u8, 88u8, 140u8, 8u8, 76u8, 235u8, 83u8, 83u8, 126u8,
                            76u8, 71u8, 251u8, 34u8, 163u8, 51u8, 139u8, 233u8, 7u8, 208u8,
                        ],
                    )
                }
                pub fn is_signer_whitelisted(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "IsSignerWhitelisted",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            30u8, 29u8, 234u8, 62u8, 10u8, 88u8, 12u8, 165u8, 115u8, 145u8, 242u8,
                            204u8, 240u8, 43u8, 209u8, 111u8, 104u8, 20u8, 149u8, 31u8, 172u8,
                            31u8, 27u8, 80u8, 106u8, 250u8, 69u8, 203u8, 41u8, 255u8, 245u8, 157u8,
                        ],
                    )
                }
                pub fn is_signer_whitelisted_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "IsSignerWhitelisted",
                        Vec::new(),
                        [
                            30u8, 29u8, 234u8, 62u8, 10u8, 88u8, 12u8, 165u8, 115u8, 145u8, 242u8,
                            204u8, 240u8, 43u8, 209u8, 111u8, 104u8, 20u8, 149u8, 31u8, 172u8,
                            31u8, 27u8, 80u8, 106u8, 250u8, 69u8, 203u8, 41u8, 255u8, 245u8, 157u8,
                        ],
                    )
                }
                pub fn is_batch_processed(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "IsBatchProcessed",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            158u8, 186u8, 125u8, 183u8, 192u8, 211u8, 213u8, 4u8, 100u8, 215u8,
                            159u8, 63u8, 62u8, 92u8, 62u8, 220u8, 245u8, 129u8, 255u8, 93u8, 32u8,
                            66u8, 246u8, 165u8, 158u8, 209u8, 33u8, 145u8, 5u8, 55u8, 172u8, 55u8,
                        ],
                    )
                }
                pub fn is_batch_processed_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "IsBatchProcessed",
                        Vec::new(),
                        [
                            158u8, 186u8, 125u8, 183u8, 192u8, 211u8, 213u8, 4u8, 100u8, 215u8,
                            159u8, 63u8, 62u8, 92u8, 62u8, 220u8, 245u8, 129u8, 255u8, 93u8, 32u8,
                            66u8, 246u8, 165u8, 158u8, 209u8, 33u8, 145u8, 5u8, 55u8, 172u8, 55u8,
                        ],
                    )
                }
                pub fn last_processed(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                        runtime_types::primitive_types::U256,
                    ),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "LastProcessed",
                        vec![],
                        [
                            26u8, 95u8, 198u8, 112u8, 125u8, 83u8, 193u8, 223u8, 199u8, 31u8,
                            198u8, 200u8, 78u8, 6u8, 155u8, 30u8, 99u8, 191u8, 246u8, 221u8, 80u8,
                            179u8, 113u8, 90u8, 109u8, 162u8, 204u8, 86u8, 125u8, 189u8, 89u8,
                            85u8,
                        ],
                    )
                }
                pub fn temp_fees_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<
                        runtime_types::pallet_support::types::sync_facade::FeeSettingsType,
                    >,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_arithmetic::fixed_point::FixedI128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "TempFeesMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            175u8, 135u8, 127u8, 249u8, 44u8, 63u8, 70u8, 57u8, 175u8, 149u8,
                            249u8, 176u8, 113u8, 10u8, 163u8, 64u8, 206u8, 121u8, 152u8, 10u8,
                            114u8, 181u8, 126u8, 73u8, 166u8, 132u8, 94u8, 73u8, 21u8, 202u8,
                            211u8, 100u8,
                        ],
                    )
                }
                pub fn temp_fees_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_arithmetic::fixed_point::FixedI128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "TempFeesMap",
                        Vec::new(),
                        [
                            175u8, 135u8, 127u8, 249u8, 44u8, 63u8, 70u8, 57u8, 175u8, 149u8,
                            249u8, 176u8, 113u8, 10u8, 163u8, 64u8, 206u8, 121u8, 152u8, 10u8,
                            114u8, 181u8, 126u8, 73u8, 166u8, 132u8, 94u8, 73u8, 21u8, 202u8,
                            211u8, 100u8,
                        ],
                    )
                }
                pub fn temp_assets_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "TempAssetsMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            32u8, 241u8, 162u8, 206u8, 101u8, 194u8, 135u8, 128u8, 11u8, 176u8,
                            154u8, 171u8, 51u8, 232u8, 46u8, 198u8, 73u8, 254u8, 221u8, 182u8,
                            184u8, 152u8, 228u8, 158u8, 47u8, 230u8, 75u8, 44u8, 141u8, 216u8,
                            192u8, 184u8,
                        ],
                    )
                }
                pub fn temp_assets_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "TempAssetsMap",
                        Vec::new(),
                        [
                            32u8, 241u8, 162u8, 206u8, 101u8, 194u8, 135u8, 128u8, 11u8, 176u8,
                            154u8, 171u8, 51u8, 232u8, 46u8, 198u8, 73u8, 254u8, 221u8, 182u8,
                            184u8, 152u8, 228u8, 158u8, 47u8, 230u8, 75u8, 44u8, 141u8, 216u8,
                            192u8, 184u8,
                        ],
                    )
                }
                pub fn signers_quorum(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u8,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "SyncFacade",
                        "SignersQuorum",
                        vec![],
                        [
                            96u8, 236u8, 214u8, 28u8, 53u8, 29u8, 187u8, 138u8, 104u8, 4u8, 223u8,
                            223u8, 74u8, 130u8, 178u8, 229u8, 78u8, 95u8, 169u8, 126u8, 242u8,
                            222u8, 71u8, 217u8, 126u8, 115u8, 203u8, 212u8, 249u8, 207u8, 147u8,
                            130u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod trading {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_trading::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_trading::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExecuteTrade {
                    pub batch_id: runtime_types::primitive_types::U256,
                    pub quantity_locked: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    pub market_id: ::core::primitive::u128,
                    pub oracle_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    pub orders:
                        ::std::vec::Vec<runtime_types::pallet_support::types::trading::Order>,
                    pub batch_timestamp: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for ExecuteTrade {
                    const PALLET: &'static str = "Trading";
                    const CALL: &'static str = "execute_trade";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddLiquidatorSigner {
                    pub pub_key: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddLiquidatorSigner {
                    const PALLET: &'static str = "Trading";
                    const CALL: &'static str = "add_liquidator_signer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveLiquidatorSigner {
                    pub pub_key: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveLiquidatorSigner {
                    const PALLET: &'static str = "Trading";
                    const CALL: &'static str = "remove_liquidator_signer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CancelOrder {
                    pub order_id: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelOrder {
                    const PALLET: &'static str = "Trading";
                    const CALL: &'static str = "cancel_order";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerformCleanup;
                impl ::subxt::blocks::StaticExtrinsic for PerformCleanup {
                    const PALLET: &'static str = "Trading";
                    const CALL: &'static str = "perform_cleanup";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetMatchingTimeLimit {
                    pub time_limit: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetMatchingTimeLimit {
                    const PALLET: &'static str = "Trading";
                    const CALL: &'static str = "set_matching_time_limit";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::execute_trade`."]
                pub fn execute_trade(
                    &self,
                    batch_id: runtime_types::primitive_types::U256,
                    quantity_locked: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    market_id: ::core::primitive::u128,
                    oracle_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    orders: ::std::vec::Vec<runtime_types::pallet_support::types::trading::Order>,
                    batch_timestamp: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::ExecuteTrade> {
                    ::subxt::tx::Payload::new_static(
                        "Trading",
                        "execute_trade",
                        types::ExecuteTrade {
                            batch_id,
                            quantity_locked,
                            market_id,
                            oracle_price,
                            orders,
                            batch_timestamp,
                        },
                        [
                            11u8, 185u8, 190u8, 103u8, 121u8, 123u8, 125u8, 135u8, 15u8, 208u8,
                            152u8, 5u8, 174u8, 81u8, 50u8, 190u8, 122u8, 118u8, 101u8, 31u8, 56u8,
                            22u8, 36u8, 207u8, 137u8, 217u8, 50u8, 22u8, 211u8, 213u8, 136u8, 60u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::add_liquidator_signer`."]
                pub fn add_liquidator_signer(
                    &self,
                    pub_key: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::AddLiquidatorSigner> {
                    ::subxt::tx::Payload::new_static(
                        "Trading",
                        "add_liquidator_signer",
                        types::AddLiquidatorSigner { pub_key },
                        [
                            212u8, 142u8, 169u8, 238u8, 173u8, 80u8, 167u8, 66u8, 50u8, 52u8,
                            218u8, 182u8, 38u8, 98u8, 148u8, 33u8, 161u8, 168u8, 198u8, 21u8,
                            219u8, 2u8, 50u8, 199u8, 94u8, 48u8, 37u8, 68u8, 66u8, 216u8, 84u8,
                            174u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::remove_liquidator_signer`."]
                pub fn remove_liquidator_signer(
                    &self,
                    pub_key: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::RemoveLiquidatorSigner> {
                    ::subxt::tx::Payload::new_static(
                        "Trading",
                        "remove_liquidator_signer",
                        types::RemoveLiquidatorSigner { pub_key },
                        [
                            178u8, 14u8, 147u8, 236u8, 15u8, 225u8, 173u8, 158u8, 34u8, 189u8,
                            204u8, 234u8, 33u8, 78u8, 0u8, 243u8, 213u8, 127u8, 11u8, 243u8, 110u8,
                            230u8, 169u8, 33u8, 40u8, 63u8, 3u8, 2u8, 124u8, 12u8, 252u8, 155u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::cancel_order`."]
                pub fn cancel_order(
                    &self,
                    order_id: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::CancelOrder> {
                    ::subxt::tx::Payload::new_static(
                        "Trading",
                        "cancel_order",
                        types::CancelOrder { order_id },
                        [
                            193u8, 19u8, 128u8, 199u8, 124u8, 115u8, 17u8, 122u8, 111u8, 84u8,
                            101u8, 44u8, 61u8, 80u8, 27u8, 29u8, 130u8, 24u8, 24u8, 5u8, 247u8,
                            155u8, 51u8, 118u8, 59u8, 139u8, 61u8, 30u8, 188u8, 138u8, 105u8,
                            120u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::perform_cleanup`."]
                pub fn perform_cleanup(&self) -> ::subxt::tx::Payload<types::PerformCleanup> {
                    ::subxt::tx::Payload::new_static(
                        "Trading",
                        "perform_cleanup",
                        types::PerformCleanup {},
                        [
                            12u8, 140u8, 35u8, 141u8, 246u8, 218u8, 198u8, 101u8, 218u8, 199u8,
                            163u8, 14u8, 83u8, 46u8, 225u8, 79u8, 166u8, 81u8, 2u8, 62u8, 70u8,
                            238u8, 87u8, 254u8, 143u8, 219u8, 165u8, 5u8, 223u8, 165u8, 49u8,
                            132u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_matching_time_limit`."]
                pub fn set_matching_time_limit(
                    &self,
                    time_limit: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetMatchingTimeLimit> {
                    ::subxt::tx::Payload::new_static(
                        "Trading",
                        "set_matching_time_limit",
                        types::SetMatchingTimeLimit { time_limit },
                        [
                            163u8, 29u8, 142u8, 200u8, 74u8, 216u8, 216u8, 187u8, 114u8, 207u8,
                            101u8, 241u8, 252u8, 9u8, 121u8, 208u8, 211u8, 43u8, 30u8, 213u8, 79u8,
                            126u8, 32u8, 59u8, 36u8, 82u8, 105u8, 214u8, 88u8, 205u8, 222u8, 37u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_trading::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Trade batch executed successfully"]
            pub struct TradeExecuted {
                pub batch_id: runtime_types::primitive_types::U256,
                pub market_id: ::core::primitive::u128,
                pub size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub execution_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub direction: ::core::primitive::u8,
                pub side: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for TradeExecuted {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "TradeExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Trade batch failed since no makers got executed"]
            pub struct TradeExecutionFailed {
                pub batch_id: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for TradeExecutionFailed {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "TradeExecutionFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Order error"]
            pub struct OrderError {
                pub order_id: runtime_types::primitive_types::U256,
                pub error_code: ::core::primitive::u16,
                pub account_id: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for OrderError {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "OrderError";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Order of a user executed successfully"]
            pub struct OrderExecuted {
                pub account_id: runtime_types::primitive_types::U256,
                pub order_id: runtime_types::primitive_types::U256,
                pub market_id: ::core::primitive::u128,
                pub size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub direction: ::core::primitive::u8,
                pub side: ::core::primitive::u8,
                pub order_type: ::core::primitive::u8,
                pub execution_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub pnl: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub fee: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub is_final: ::core::primitive::bool,
                pub is_maker: ::core::primitive::bool,
            }
            impl ::subxt::events::StaticEvent for OrderExecuted {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "OrderExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Force closure flag updation event"]
            pub struct ForceClosureFlagsChanged {
                pub account_id: runtime_types::primitive_types::U256,
                pub collateral_id: ::core::primitive::u128,
                pub force_closure_flag: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for ForceClosureFlagsChanged {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "ForceClosureFlagsChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Liquidator signer added"]
            pub struct LiquidatorSignerAdded {
                pub signer: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for LiquidatorSignerAdded {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "LiquidatorSignerAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Liquidator signer removed"]
            pub struct LiquidatorSignerRemoved {
                pub signer: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for LiquidatorSignerRemoved {
                const PALLET: &'static str = "Trading";
                const EVENT: &'static str = "LiquidatorSignerRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn batch_status_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "BatchStatusMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            229u8, 255u8, 222u8, 67u8, 87u8, 151u8, 33u8, 226u8, 206u8, 241u8,
                            240u8, 176u8, 189u8, 240u8, 248u8, 216u8, 124u8, 245u8, 65u8, 127u8,
                            80u8, 26u8, 176u8, 242u8, 89u8, 3u8, 169u8, 79u8, 71u8, 159u8, 170u8,
                            67u8,
                        ],
                    )
                }
                pub fn batch_status_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "BatchStatusMap",
                        Vec::new(),
                        [
                            229u8, 255u8, 222u8, 67u8, 87u8, 151u8, 33u8, 226u8, 206u8, 241u8,
                            240u8, 176u8, 189u8, 240u8, 248u8, 216u8, 124u8, 245u8, 65u8, 127u8,
                            80u8, 26u8, 176u8, 242u8, 89u8, 3u8, 169u8, 79u8, 71u8, 159u8, 170u8,
                            67u8,
                        ],
                    )
                }
                pub fn order_state_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        ::core::primitive::bool,
                    ),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OrderStateMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            225u8, 37u8, 249u8, 253u8, 125u8, 29u8, 69u8, 68u8, 8u8, 86u8, 192u8,
                            16u8, 185u8, 24u8, 5u8, 57u8, 165u8, 104u8, 173u8, 21u8, 128u8, 122u8,
                            132u8, 103u8, 245u8, 247u8, 89u8, 185u8, 62u8, 16u8, 158u8, 156u8,
                        ],
                    )
                }
                pub fn order_state_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        ::core::primitive::bool,
                    ),
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OrderStateMap",
                        Vec::new(),
                        [
                            225u8, 37u8, 249u8, 253u8, 125u8, 29u8, 69u8, 68u8, 8u8, 86u8, 192u8,
                            16u8, 185u8, 24u8, 5u8, 57u8, 165u8, 104u8, 173u8, 21u8, 128u8, 122u8,
                            132u8, 103u8, 245u8, 247u8, 89u8, 185u8, 62u8, 16u8, 158u8, 156u8,
                        ],
                    )
                }
                pub fn positions_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<(
                        ::core::primitive::u128,
                        runtime_types::pallet_support::types::trading::Direction,
                    )>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading::Position,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "PositionsMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            255u8, 115u8, 57u8, 37u8, 234u8, 233u8, 149u8, 12u8, 54u8, 179u8, 30u8,
                            150u8, 221u8, 239u8, 100u8, 225u8, 194u8, 83u8, 103u8, 17u8, 155u8,
                            47u8, 205u8, 180u8, 134u8, 170u8, 245u8, 71u8, 52u8, 164u8, 197u8,
                            104u8,
                        ],
                    )
                }
                pub fn positions_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading::Position,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "PositionsMap",
                        Vec::new(),
                        [
                            255u8, 115u8, 57u8, 37u8, 234u8, 233u8, 149u8, 12u8, 54u8, 179u8, 30u8,
                            150u8, 221u8, 239u8, 100u8, 225u8, 194u8, 83u8, 103u8, 17u8, 155u8,
                            47u8, 205u8, 180u8, 134u8, 170u8, 245u8, 71u8, 52u8, 164u8, 197u8,
                            104u8,
                        ],
                    )
                }
                pub fn collateral_to_market_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "CollateralToMarketMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            179u8, 145u8, 107u8, 128u8, 30u8, 158u8, 8u8, 110u8, 233u8, 186u8,
                            24u8, 2u8, 1u8, 109u8, 89u8, 40u8, 81u8, 97u8, 170u8, 209u8, 41u8,
                            12u8, 238u8, 121u8, 144u8, 180u8, 107u8, 150u8, 97u8, 248u8, 182u8,
                            176u8,
                        ],
                    )
                }
                pub fn collateral_to_market_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "CollateralToMarketMap",
                        Vec::new(),
                        [
                            179u8, 145u8, 107u8, 128u8, 30u8, 158u8, 8u8, 110u8, 233u8, 186u8,
                            24u8, 2u8, 1u8, 109u8, 89u8, 40u8, 81u8, 97u8, 170u8, 209u8, 41u8,
                            12u8, 238u8, 121u8, 144u8, 180u8, 107u8, 150u8, 97u8, 248u8, 182u8,
                            176u8,
                        ],
                    )
                }
                pub fn open_interest_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OpenInterestMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            53u8, 100u8, 139u8, 40u8, 214u8, 80u8, 50u8, 53u8, 91u8, 164u8, 182u8,
                            230u8, 243u8, 236u8, 95u8, 166u8, 181u8, 81u8, 81u8, 205u8, 167u8, 3u8,
                            28u8, 97u8, 127u8, 255u8, 76u8, 143u8, 224u8, 25u8, 145u8, 244u8,
                        ],
                    )
                }
                pub fn open_interest_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OpenInterestMap",
                        Vec::new(),
                        [
                            53u8, 100u8, 139u8, 40u8, 214u8, 80u8, 50u8, 53u8, 91u8, 164u8, 182u8,
                            230u8, 243u8, 236u8, 95u8, 166u8, 181u8, 81u8, 81u8, 205u8, 167u8, 3u8,
                            28u8, 97u8, 127u8, 255u8, 76u8, 143u8, 224u8, 25u8, 145u8, 244u8,
                        ],
                    )
                }
                pub fn initial_margin_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<
                        runtime_types::pallet_support::types::trading::Direction,
                    >,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "InitialMarginMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            249u8, 251u8, 205u8, 85u8, 184u8, 218u8, 90u8, 123u8, 198u8, 157u8,
                            78u8, 79u8, 162u8, 48u8, 226u8, 154u8, 4u8, 76u8, 229u8, 243u8, 234u8,
                            149u8, 47u8, 109u8, 147u8, 38u8, 87u8, 140u8, 160u8, 9u8, 66u8, 159u8,
                        ],
                    )
                }
                pub fn initial_margin_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "InitialMarginMap",
                        Vec::new(),
                        [
                            249u8, 251u8, 205u8, 85u8, 184u8, 218u8, 90u8, 123u8, 198u8, 157u8,
                            78u8, 79u8, 162u8, 48u8, 226u8, 154u8, 4u8, 76u8, 229u8, 243u8, 234u8,
                            149u8, 47u8, 109u8, 147u8, 38u8, 87u8, 140u8, 160u8, 9u8, 66u8, 159u8,
                        ],
                    )
                }
                pub fn deleveragable_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "DeleveragableMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            213u8, 10u8, 252u8, 75u8, 65u8, 44u8, 196u8, 207u8, 42u8, 46u8, 59u8,
                            168u8, 175u8, 223u8, 34u8, 211u8, 228u8, 131u8, 66u8, 149u8, 80u8,
                            89u8, 250u8, 100u8, 253u8, 174u8, 222u8, 163u8, 26u8, 57u8, 12u8,
                            225u8,
                        ],
                    )
                }
                pub fn deleveragable_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "DeleveragableMap",
                        Vec::new(),
                        [
                            213u8, 10u8, 252u8, 75u8, 65u8, 44u8, 196u8, 207u8, 42u8, 46u8, 59u8,
                            168u8, 175u8, 223u8, 34u8, 211u8, 228u8, 131u8, 66u8, 149u8, 80u8,
                            89u8, 250u8, 100u8, 253u8, 174u8, 222u8, 163u8, 26u8, 57u8, 12u8,
                            225u8,
                        ],
                    )
                }
                pub fn force_closure_flag_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading::ForceClosureFlag,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "ForceClosureFlagMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            227u8, 158u8, 183u8, 78u8, 36u8, 171u8, 120u8, 48u8, 61u8, 62u8, 128u8,
                            222u8, 140u8, 215u8, 147u8, 1u8, 63u8, 71u8, 110u8, 80u8, 87u8, 135u8,
                            29u8, 222u8, 64u8, 178u8, 92u8, 26u8, 123u8, 204u8, 97u8, 148u8,
                        ],
                    )
                }
                pub fn force_closure_flag_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading::ForceClosureFlag,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "ForceClosureFlagMap",
                        Vec::new(),
                        [
                            227u8, 158u8, 183u8, 78u8, 36u8, 171u8, 120u8, 48u8, 61u8, 62u8, 128u8,
                            222u8, 140u8, 215u8, 147u8, 1u8, 63u8, 71u8, 110u8, 80u8, 87u8, 135u8,
                            29u8, 222u8, 64u8, 178u8, 92u8, 26u8, 123u8, 204u8, 97u8, 148u8,
                        ],
                    )
                }
                pub fn order_hash_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OrderHashMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            230u8, 53u8, 124u8, 227u8, 70u8, 113u8, 145u8, 112u8, 31u8, 170u8,
                            193u8, 152u8, 21u8, 35u8, 9u8, 50u8, 17u8, 41u8, 136u8, 79u8, 18u8,
                            145u8, 101u8, 111u8, 187u8, 45u8, 157u8, 101u8, 218u8, 144u8, 33u8,
                            64u8,
                        ],
                    )
                }
                pub fn order_hash_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OrderHashMap",
                        Vec::new(),
                        [
                            230u8, 53u8, 124u8, 227u8, 70u8, 113u8, 145u8, 112u8, 31u8, 170u8,
                            193u8, 152u8, 21u8, 35u8, 9u8, 50u8, 17u8, 41u8, 136u8, 79u8, 18u8,
                            145u8, 101u8, 111u8, 187u8, 45u8, 157u8, 101u8, 218u8, 144u8, 33u8,
                            64u8,
                        ],
                    )
                }
                pub fn liquidator_signers(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "LiquidatorSigners",
                        vec![],
                        [
                            103u8, 64u8, 30u8, 7u8, 151u8, 46u8, 214u8, 230u8, 97u8, 253u8, 118u8,
                            36u8, 4u8, 185u8, 99u8, 254u8, 128u8, 218u8, 142u8, 89u8, 143u8, 155u8,
                            163u8, 115u8, 241u8, 250u8, 167u8, 227u8, 249u8, 53u8, 60u8, 211u8,
                        ],
                    )
                }
                pub fn is_liquidator_signer_whitelisted(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::primitive_types::U256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "IsLiquidatorSignerWhitelisted",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            97u8, 199u8, 239u8, 242u8, 198u8, 74u8, 108u8, 245u8, 196u8, 223u8,
                            181u8, 154u8, 161u8, 185u8, 30u8, 151u8, 9u8, 250u8, 35u8, 37u8, 232u8,
                            86u8, 135u8, 57u8, 229u8, 216u8, 181u8, 78u8, 25u8, 246u8, 91u8, 117u8,
                        ],
                    )
                }
                pub fn is_liquidator_signer_whitelisted_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "IsLiquidatorSignerWhitelisted",
                        Vec::new(),
                        [
                            97u8, 199u8, 239u8, 242u8, 198u8, 74u8, 108u8, 245u8, 196u8, 223u8,
                            181u8, 154u8, 161u8, 185u8, 30u8, 151u8, 9u8, 250u8, 35u8, 37u8, 232u8,
                            86u8, 135u8, 57u8, 229u8, 216u8, 181u8, 78u8, 25u8, 246u8, 91u8, 117u8,
                        ],
                    )
                }
                pub fn orders_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OrdersMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            139u8, 47u8, 7u8, 150u8, 167u8, 151u8, 87u8, 4u8, 102u8, 79u8, 251u8,
                            182u8, 69u8, 191u8, 77u8, 197u8, 157u8, 52u8, 137u8, 129u8, 179u8,
                            197u8, 232u8, 112u8, 91u8, 139u8, 86u8, 218u8, 110u8, 12u8, 15u8,
                            216u8,
                        ],
                    )
                }
                pub fn orders_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "OrdersMap",
                        Vec::new(),
                        [
                            139u8, 47u8, 7u8, 150u8, 167u8, 151u8, 87u8, 4u8, 102u8, 79u8, 251u8,
                            182u8, 69u8, 191u8, 77u8, 197u8, 157u8, 52u8, 137u8, 129u8, 179u8,
                            197u8, 232u8, 112u8, 91u8, 139u8, 86u8, 218u8, 110u8, 12u8, 15u8,
                            216u8,
                        ],
                    )
                }
                pub fn batches_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "BatchesMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            147u8, 184u8, 144u8, 101u8, 9u8, 219u8, 144u8, 143u8, 19u8, 42u8,
                            211u8, 189u8, 187u8, 77u8, 190u8, 225u8, 188u8, 31u8, 183u8, 224u8,
                            105u8, 154u8, 135u8, 96u8, 90u8, 55u8, 149u8, 171u8, 33u8, 163u8, 29u8,
                            224u8,
                        ],
                    )
                }
                pub fn batches_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "BatchesMap",
                        Vec::new(),
                        [
                            147u8, 184u8, 144u8, 101u8, 9u8, 219u8, 144u8, 143u8, 19u8, 42u8,
                            211u8, 189u8, 187u8, 77u8, 190u8, 225u8, 188u8, 31u8, 183u8, 224u8,
                            105u8, 154u8, 135u8, 96u8, 90u8, 55u8, 149u8, 171u8, 33u8, 163u8, 29u8,
                            224u8,
                        ],
                    )
                }
                pub fn start_timestamp(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "StartTimestamp",
                        vec![],
                        [
                            5u8, 179u8, 63u8, 175u8, 228u8, 123u8, 176u8, 65u8, 12u8, 22u8, 234u8,
                            77u8, 134u8, 91u8, 167u8, 52u8, 25u8, 230u8, 59u8, 60u8, 74u8, 239u8,
                            230u8, 22u8, 73u8, 150u8, 77u8, 132u8, 13u8, 163u8, 121u8, 224u8,
                        ],
                    )
                }
                pub fn trading_fee_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "TradingFeeMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            72u8, 172u8, 188u8, 65u8, 104u8, 96u8, 148u8, 183u8, 70u8, 23u8, 14u8,
                            73u8, 103u8, 161u8, 113u8, 239u8, 185u8, 63u8, 223u8, 235u8, 52u8,
                            85u8, 72u8, 176u8, 64u8, 72u8, 23u8, 226u8, 222u8, 220u8, 112u8, 43u8,
                        ],
                    )
                }
                pub fn trading_fee_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "TradingFeeMap",
                        Vec::new(),
                        [
                            72u8, 172u8, 188u8, 65u8, 104u8, 96u8, 148u8, 183u8, 70u8, 23u8, 14u8,
                            73u8, 103u8, 161u8, 113u8, 239u8, 185u8, 63u8, 223u8, 235u8, 52u8,
                            85u8, 72u8, 176u8, 64u8, 72u8, 23u8, 226u8, 222u8, 220u8, 112u8, 43u8,
                        ],
                    )
                }
                pub fn liquidation_fee_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "LiquidationFeeMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            234u8, 99u8, 79u8, 66u8, 86u8, 6u8, 115u8, 44u8, 98u8, 99u8, 236u8,
                            102u8, 188u8, 64u8, 11u8, 118u8, 88u8, 43u8, 231u8, 9u8, 188u8, 197u8,
                            78u8, 200u8, 26u8, 119u8, 6u8, 132u8, 126u8, 73u8, 111u8, 79u8,
                        ],
                    )
                }
                pub fn liquidation_fee_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "LiquidationFeeMap",
                        Vec::new(),
                        [
                            234u8, 99u8, 79u8, 66u8, 86u8, 6u8, 115u8, 44u8, 98u8, 99u8, 236u8,
                            102u8, 188u8, 64u8, 11u8, 118u8, 88u8, 43u8, 231u8, 9u8, 188u8, 197u8,
                            78u8, 200u8, 26u8, 119u8, 6u8, 132u8, 126u8, 73u8, 111u8, 79u8,
                        ],
                    )
                }
                pub fn matching_time_limit(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Trading",
                        "MatchingTimeLimit",
                        vec![],
                        [
                            4u8, 39u8, 230u8, 221u8, 76u8, 242u8, 88u8, 136u8, 220u8, 54u8, 211u8,
                            17u8, 204u8, 52u8, 83u8, 227u8, 1u8, 198u8, 149u8, 79u8, 73u8, 92u8,
                            160u8, 63u8, 181u8, 3u8, 82u8, 36u8, 120u8, 122u8, 67u8, 88u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod trading_fees {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_trading_fees::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_trading_fees::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateBaseFees {
                    pub id: ::core::primitive::u128,
                    pub side: runtime_types::pallet_support::types::trading::Side,
                    pub order_side: runtime_types::pallet_support::types::trading::OrderSide,
                    pub fee_details: ::std::vec::Vec<
                        runtime_types::pallet_support::types::trading_fees::BaseFee,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateBaseFees {
                    const PALLET: &'static str = "TradingFees";
                    const CALL: &'static str = "update_base_fees";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::update_base_fees`."]
                pub fn update_base_fees(
                    &self,
                    id: ::core::primitive::u128,
                    side: runtime_types::pallet_support::types::trading::Side,
                    order_side: runtime_types::pallet_support::types::trading::OrderSide,
                    fee_details: ::std::vec::Vec<
                        runtime_types::pallet_support::types::trading_fees::BaseFee,
                    >,
                ) -> ::subxt::tx::Payload<types::UpdateBaseFees> {
                    ::subxt::tx::Payload::new_static(
                        "TradingFees",
                        "update_base_fees",
                        types::UpdateBaseFees {
                            id,
                            side,
                            order_side,
                            fee_details,
                        },
                        [
                            25u8, 151u8, 14u8, 181u8, 206u8, 200u8, 40u8, 118u8, 32u8, 219u8,
                            203u8, 178u8, 226u8, 72u8, 117u8, 67u8, 78u8, 118u8, 20u8, 203u8, 32u8,
                            238u8, 135u8, 73u8, 150u8, 115u8, 25u8, 225u8, 11u8, 50u8, 107u8,
                            133u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_trading_fees::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Base fees details updated"]
            pub struct BaseFeesUpdated {
                pub fee_tiers: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for BaseFeesUpdated {
                const PALLET: &'static str = "TradingFees";
                const EVENT: &'static str = "BaseFeesUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn max_base_fee_tier(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<
                        runtime_types::pallet_support::types::trading::OrderSide,
                    >,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u8,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingFees",
                        "MaxBaseFeeTier",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            21u8, 121u8, 217u8, 39u8, 203u8, 75u8, 179u8, 32u8, 92u8, 223u8, 138u8,
                            191u8, 79u8, 149u8, 134u8, 176u8, 10u8, 16u8, 177u8, 31u8, 124u8,
                            143u8, 40u8, 236u8, 25u8, 174u8, 79u8, 95u8, 110u8, 142u8, 71u8, 176u8,
                        ],
                    )
                }
                pub fn max_base_fee_tier_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u8,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingFees",
                        "MaxBaseFeeTier",
                        Vec::new(),
                        [
                            21u8, 121u8, 217u8, 39u8, 203u8, 75u8, 179u8, 32u8, 92u8, 223u8, 138u8,
                            191u8, 79u8, 149u8, 134u8, 176u8, 10u8, 16u8, 177u8, 31u8, 124u8,
                            143u8, 40u8, 236u8, 25u8, 174u8, 79u8, 95u8, 110u8, 142u8, 71u8, 176u8,
                        ],
                    )
                }
                pub fn base_fee_tier_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<(
                        ::core::primitive::u8,
                        runtime_types::pallet_support::types::trading::Side,
                        runtime_types::pallet_support::types::trading::OrderSide,
                    )>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading_fees::BaseFee,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingFees",
                        "BaseFeeTierMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            8u8, 189u8, 188u8, 94u8, 189u8, 181u8, 241u8, 186u8, 254u8, 236u8,
                            95u8, 209u8, 64u8, 112u8, 49u8, 237u8, 243u8, 232u8, 192u8, 70u8,
                            141u8, 68u8, 99u8, 198u8, 44u8, 177u8, 140u8, 216u8, 68u8, 198u8,
                            177u8, 80u8,
                        ],
                    )
                }
                pub fn base_fee_tier_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::trading_fees::BaseFee,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TradingFees",
                        "BaseFeeTierMap",
                        Vec::new(),
                        [
                            8u8, 189u8, 188u8, 94u8, 189u8, 181u8, 241u8, 186u8, 254u8, 236u8,
                            95u8, 209u8, 64u8, 112u8, 49u8, 237u8, 243u8, 232u8, 192u8, 70u8,
                            141u8, 68u8, 99u8, 198u8, 44u8, 177u8, 140u8, 216u8, 68u8, 198u8,
                            177u8, 80u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod prices {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_prices::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_prices::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetInitialisationTimestamp {
                    pub timestamp: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetInitialisationTimestamp {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_initialisation_timestamp";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetDefaultMaxAbr {
                    pub max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetDefaultMaxAbr {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_default_max_abr";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetMaxAbr {
                    pub market_id: ::core::primitive::u128,
                    pub max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetMaxAbr {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_max_abr";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetAbrInterval {
                    pub new_abr_interval: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetAbrInterval {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_abr_interval";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBaseAbr {
                    pub new_base_abr: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBaseAbr {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_base_abr";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBollingerWidth {
                    pub new_bollinger_width: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBollingerWidth {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_bollinger_width";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetNoOfUsersPerBatch {
                    pub new_no_of_users_per_batch: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetNoOfUsersPerBatch {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_no_of_users_per_batch";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetAbrValue {
                    pub market_id: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetAbrValue {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "set_abr_value";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MakeAbrPayments;
                impl ::subxt::blocks::StaticExtrinsic for MakeAbrPayments {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "make_abr_payments";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdatePrices {
                    pub prices: ::std::vec::Vec<
                        runtime_types::pallet_support::types::prices::MultiplePrices,
                    >,
                    pub timestamp: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdatePrices {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "update_prices";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerformPricesCleanup;
                impl ::subxt::blocks::StaticExtrinsic for PerformPricesCleanup {
                    const PALLET: &'static str = "Prices";
                    const CALL: &'static str = "perform_prices_cleanup";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See `Pallet::set_initialisation_timestamp`."]
                pub fn set_initialisation_timestamp(
                    &self,
                    timestamp: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetInitialisationTimestamp> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_initialisation_timestamp",
                        types::SetInitialisationTimestamp { timestamp },
                        [
                            185u8, 64u8, 178u8, 230u8, 45u8, 73u8, 175u8, 93u8, 184u8, 151u8,
                            222u8, 75u8, 80u8, 125u8, 21u8, 60u8, 50u8, 67u8, 52u8, 108u8, 49u8,
                            86u8, 224u8, 129u8, 21u8, 8u8, 242u8, 19u8, 91u8, 125u8, 177u8, 55u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_default_max_abr`."]
                pub fn set_default_max_abr(
                    &self,
                    max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                ) -> ::subxt::tx::Payload<types::SetDefaultMaxAbr> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_default_max_abr",
                        types::SetDefaultMaxAbr { max_abr_value },
                        [
                            39u8, 85u8, 131u8, 156u8, 167u8, 178u8, 165u8, 173u8, 104u8, 192u8,
                            205u8, 238u8, 148u8, 53u8, 141u8, 8u8, 168u8, 3u8, 63u8, 32u8, 41u8,
                            88u8, 174u8, 205u8, 134u8, 209u8, 253u8, 63u8, 204u8, 113u8, 35u8,
                            125u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_max_abr`."]
                pub fn set_max_abr(
                    &self,
                    market_id: ::core::primitive::u128,
                    max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                ) -> ::subxt::tx::Payload<types::SetMaxAbr> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_max_abr",
                        types::SetMaxAbr {
                            market_id,
                            max_abr_value,
                        },
                        [
                            135u8, 244u8, 128u8, 45u8, 202u8, 45u8, 70u8, 147u8, 202u8, 187u8,
                            56u8, 233u8, 250u8, 67u8, 196u8, 207u8, 220u8, 247u8, 81u8, 9u8, 122u8,
                            74u8, 197u8, 130u8, 110u8, 224u8, 131u8, 118u8, 5u8, 184u8, 194u8,
                            122u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_abr_interval`."]
                pub fn set_abr_interval(
                    &self,
                    new_abr_interval: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetAbrInterval> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_abr_interval",
                        types::SetAbrInterval { new_abr_interval },
                        [
                            190u8, 110u8, 160u8, 32u8, 108u8, 60u8, 102u8, 251u8, 226u8, 25u8,
                            208u8, 180u8, 43u8, 14u8, 65u8, 60u8, 171u8, 69u8, 194u8, 195u8, 150u8,
                            109u8, 102u8, 123u8, 148u8, 66u8, 81u8, 151u8, 213u8, 224u8, 232u8,
                            14u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_base_abr`."]
                pub fn set_base_abr(
                    &self,
                    new_base_abr: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                ) -> ::subxt::tx::Payload<types::SetBaseAbr> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_base_abr",
                        types::SetBaseAbr { new_base_abr },
                        [
                            158u8, 4u8, 160u8, 214u8, 100u8, 165u8, 25u8, 157u8, 253u8, 184u8,
                            34u8, 229u8, 214u8, 122u8, 240u8, 11u8, 166u8, 236u8, 47u8, 166u8,
                            64u8, 228u8, 227u8, 124u8, 184u8, 79u8, 54u8, 134u8, 184u8, 15u8,
                            196u8, 126u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_bollinger_width`."]
                pub fn set_bollinger_width(
                    &self,
                    new_bollinger_width: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                ) -> ::subxt::tx::Payload<types::SetBollingerWidth> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_bollinger_width",
                        types::SetBollingerWidth {
                            new_bollinger_width,
                        },
                        [
                            1u8, 244u8, 104u8, 18u8, 157u8, 10u8, 27u8, 128u8, 241u8, 6u8, 172u8,
                            53u8, 74u8, 255u8, 27u8, 21u8, 56u8, 200u8, 42u8, 194u8, 62u8, 87u8,
                            77u8, 178u8, 189u8, 85u8, 207u8, 101u8, 71u8, 32u8, 237u8, 204u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_no_of_users_per_batch`."]
                pub fn set_no_of_users_per_batch(
                    &self,
                    new_no_of_users_per_batch: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetNoOfUsersPerBatch> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_no_of_users_per_batch",
                        types::SetNoOfUsersPerBatch {
                            new_no_of_users_per_batch,
                        },
                        [
                            244u8, 24u8, 208u8, 73u8, 188u8, 235u8, 156u8, 149u8, 254u8, 196u8,
                            8u8, 248u8, 202u8, 80u8, 55u8, 246u8, 224u8, 72u8, 230u8, 27u8, 173u8,
                            64u8, 202u8, 246u8, 252u8, 169u8, 200u8, 176u8, 206u8, 26u8, 113u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::set_abr_value`."]
                pub fn set_abr_value(
                    &self,
                    market_id: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetAbrValue> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "set_abr_value",
                        types::SetAbrValue { market_id },
                        [
                            191u8, 84u8, 1u8, 158u8, 19u8, 45u8, 142u8, 180u8, 29u8, 74u8, 112u8,
                            222u8, 79u8, 154u8, 122u8, 75u8, 46u8, 117u8, 46u8, 34u8, 248u8, 92u8,
                            227u8, 12u8, 185u8, 217u8, 201u8, 142u8, 88u8, 100u8, 180u8, 74u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::make_abr_payments`."]
                pub fn make_abr_payments(&self) -> ::subxt::tx::Payload<types::MakeAbrPayments> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "make_abr_payments",
                        types::MakeAbrPayments {},
                        [
                            142u8, 230u8, 62u8, 207u8, 156u8, 62u8, 248u8, 217u8, 49u8, 70u8, 17u8,
                            165u8, 29u8, 205u8, 150u8, 83u8, 246u8, 39u8, 42u8, 88u8, 145u8, 52u8,
                            123u8, 169u8, 9u8, 208u8, 136u8, 208u8, 115u8, 138u8, 188u8, 21u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::update_prices`."]
                pub fn update_prices(
                    &self,
                    prices: ::std::vec::Vec<
                        runtime_types::pallet_support::types::prices::MultiplePrices,
                    >,
                    timestamp: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::UpdatePrices> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "update_prices",
                        types::UpdatePrices { prices, timestamp },
                        [
                            168u8, 240u8, 20u8, 103u8, 206u8, 15u8, 241u8, 211u8, 28u8, 34u8,
                            167u8, 122u8, 150u8, 130u8, 121u8, 48u8, 32u8, 45u8, 117u8, 19u8,
                            144u8, 54u8, 33u8, 78u8, 106u8, 107u8, 2u8, 209u8, 75u8, 255u8, 22u8,
                            30u8,
                        ],
                    )
                }
                #[doc = "See `Pallet::perform_prices_cleanup`."]
                pub fn perform_prices_cleanup(
                    &self,
                ) -> ::subxt::tx::Payload<types::PerformPricesCleanup> {
                    ::subxt::tx::Payload::new_static(
                        "Prices",
                        "perform_prices_cleanup",
                        types::PerformPricesCleanup {},
                        [
                            90u8, 51u8, 41u8, 117u8, 182u8, 31u8, 4u8, 137u8, 15u8, 210u8, 223u8,
                            208u8, 226u8, 62u8, 206u8, 60u8, 197u8, 90u8, 204u8, 138u8, 159u8,
                            115u8, 106u8, 239u8, 173u8, 27u8, 39u8, 23u8, 97u8, 202u8, 173u8,
                            192u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_prices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Last traded price was successfully updated"]
            pub struct LastOraclePriceUpdated {
                pub market_id: ::core::primitive::u128,
                pub price: runtime_types::pallet_support::types::prices::LastOraclePrice,
            }
            impl ::subxt::events::StaticEvent for LastOraclePriceUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "LastOraclePriceUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "ABR timestamp set successfully"]
            pub struct AbrTimestampSet {
                pub epoch: ::core::primitive::u64,
                pub timestamp: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for AbrTimestampSet {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "AbrTimestampSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "ABR state changed successfully"]
            pub struct AbrStateChanged {
                pub epoch: ::core::primitive::u64,
                pub state: runtime_types::pallet_support::types::abr::ABRState,
            }
            impl ::subxt::events::StaticEvent for AbrStateChanged {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "AbrStateChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "ABR value set successfully"]
            pub struct AbrValueSet {
                pub epoch: ::core::primitive::u64,
                pub market_id: ::core::primitive::u128,
                pub abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub abr_last_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for AbrValueSet {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "AbrValueSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "ABR payment made successfully"]
            pub struct AbrPaymentMade {
                pub epoch: ::core::primitive::u64,
                pub batch_id: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for AbrPaymentMade {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "AbrPaymentMade";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "ABR payment for a user made successfully"]
            pub struct UserAbrPayment {
                pub account_id: runtime_types::primitive_types::U256,
                pub market_id: ::core::primitive::u128,
                pub collateral_id: ::core::primitive::u128,
                pub abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub abr_timestamp: ::core::primitive::u64,
                pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                pub modify_type: runtime_types::pallet_support::types::trading::FundModifyType,
                pub position_size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for UserAbrPayment {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "UserAbrPayment";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "ABR interval updated successfully"]
            pub struct AbrIntervalUpdated {
                pub abr_interval: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for AbrIntervalUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "AbrIntervalUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Default Max ABR value updated successfully"]
            pub struct DefaultMaxAbrUpdated {
                pub max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for DefaultMaxAbrUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "DefaultMaxAbrUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Max ABR value of a market updated successfully"]
            pub struct MaxAbrForMarketUpdated {
                pub market_id: ::core::primitive::u128,
                pub max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for MaxAbrForMarketUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "MaxAbrForMarketUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Initialisation timestamp updated successfully"]
            pub struct InitialisationTimestampUpdated {
                pub timestamp: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for InitialisationTimestampUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "InitialisationTimestampUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "No of users per batch updated successfully"]
            pub struct NoOfUsersPerBatchUpdated {
                pub no_of_users_per_batch: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for NoOfUsersPerBatchUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "NoOfUsersPerBatchUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Base ABR updated successfully"]
            pub struct BaseAbrUpdated {
                pub base_abr: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for BaseAbrUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "BaseAbrUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Bollinger width updated successfully"]
            pub struct BollingerWidthUpdated {
                pub bollinger_width: runtime_types::sp_arithmetic::fixed_point::FixedI128,
            }
            impl ::subxt::events::StaticEvent for BollingerWidthUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "BollingerWidthUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Index/mark prices updated successfully"]
            pub struct PricesUpdated {
                pub timestamp: ::core::primitive::u64,
                pub prices:
                    ::std::vec::Vec<runtime_types::pallet_support::types::prices::MultiplePrices>,
            }
            impl ::subxt::events::StaticEvent for PricesUpdated {
                const PALLET: &'static str = "Prices";
                const EVENT: &'static str = "PricesUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn last_oracle_prices_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::prices::LastOraclePrice,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "LastOraclePricesMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            181u8, 85u8, 10u8, 201u8, 193u8, 180u8, 95u8, 230u8, 63u8, 189u8,
                            205u8, 219u8, 110u8, 11u8, 160u8, 78u8, 42u8, 232u8, 150u8, 26u8, 90u8,
                            27u8, 124u8, 162u8, 25u8, 78u8, 200u8, 151u8, 90u8, 218u8, 100u8,
                            210u8,
                        ],
                    )
                }
                pub fn last_oracle_prices_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::prices::LastOraclePrice,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "LastOraclePricesMap",
                        Vec::new(),
                        [
                            181u8, 85u8, 10u8, 201u8, 193u8, 180u8, 95u8, 230u8, 63u8, 189u8,
                            205u8, 219u8, 110u8, 11u8, 160u8, 78u8, 42u8, 232u8, 150u8, 26u8, 90u8,
                            27u8, 124u8, 162u8, 25u8, 78u8, 200u8, 151u8, 90u8, 218u8, 100u8,
                            210u8,
                        ],
                    )
                }
                pub fn current_prices_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::prices::CurrentPrice,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "CurrentPricesMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            183u8, 213u8, 246u8, 121u8, 231u8, 199u8, 10u8, 146u8, 133u8, 65u8,
                            16u8, 244u8, 1u8, 186u8, 67u8, 22u8, 0u8, 76u8, 5u8, 30u8, 75u8, 65u8,
                            40u8, 17u8, 236u8, 19u8, 153u8, 31u8, 156u8, 43u8, 208u8, 95u8,
                        ],
                    )
                }
                pub fn current_prices_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::prices::CurrentPrice,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "CurrentPricesMap",
                        Vec::new(),
                        [
                            183u8, 213u8, 246u8, 121u8, 231u8, 199u8, 10u8, 146u8, 133u8, 65u8,
                            16u8, 244u8, 1u8, 186u8, 67u8, 22u8, 0u8, 76u8, 5u8, 30u8, 75u8, 65u8,
                            40u8, 17u8, 236u8, 19u8, 153u8, 31u8, 156u8, 43u8, 208u8, 95u8,
                        ],
                    )
                }
                pub fn historical_prices_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::prices::HistoricalPrice,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "HistoricalPricesMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            197u8, 162u8, 114u8, 229u8, 78u8, 162u8, 55u8, 109u8, 166u8, 25u8,
                            120u8, 255u8, 196u8, 92u8, 79u8, 213u8, 156u8, 234u8, 148u8, 134u8,
                            63u8, 136u8, 95u8, 50u8, 133u8, 166u8, 118u8, 129u8, 239u8, 67u8, 73u8,
                            217u8,
                        ],
                    )
                }
                pub fn historical_prices_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::prices::HistoricalPrice,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "HistoricalPricesMap",
                        Vec::new(),
                        [
                            197u8, 162u8, 114u8, 229u8, 78u8, 162u8, 55u8, 109u8, 166u8, 25u8,
                            120u8, 255u8, 196u8, 92u8, 79u8, 213u8, 156u8, 234u8, 148u8, 134u8,
                            63u8, 136u8, 95u8, 50u8, 133u8, 166u8, 118u8, 129u8, 239u8, 67u8, 73u8,
                            217u8,
                        ],
                    )
                }
                pub fn prices_start_timestamp(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "PricesStartTimestamp",
                        vec![],
                        [
                            43u8, 168u8, 198u8, 183u8, 200u8, 165u8, 169u8, 139u8, 146u8, 250u8,
                            33u8, 81u8, 88u8, 23u8, 12u8, 69u8, 58u8, 254u8, 96u8, 93u8, 89u8,
                            26u8, 158u8, 3u8, 69u8, 250u8, 29u8, 237u8, 102u8, 164u8, 203u8, 199u8,
                        ],
                    )
                }
                #[doc = " Stores the timestamp at which substrate was initialised"]
                pub fn initialisation_timestamp(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "InitialisationTimestamp",
                        vec![],
                        [
                            12u8, 105u8, 181u8, 161u8, 44u8, 145u8, 117u8, 162u8, 0u8, 29u8, 27u8,
                            99u8, 204u8, 238u8, 253u8, 234u8, 129u8, 46u8, 75u8, 2u8, 174u8, 128u8,
                            7u8, 0u8, 161u8, 83u8, 221u8, 6u8, 93u8, 149u8, 238u8, 192u8,
                        ],
                    )
                }
                #[doc = " Stores the state of ABR"]
                pub fn abr_state(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_support::types::abr::ABRState,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "AbrState",
                        vec![],
                        [
                            15u8, 151u8, 135u8, 97u8, 165u8, 223u8, 10u8, 77u8, 179u8, 255u8, 4u8,
                            248u8, 18u8, 65u8, 137u8, 146u8, 177u8, 78u8, 63u8, 107u8, 3u8, 15u8,
                            236u8, 177u8, 189u8, 214u8, 47u8, 221u8, 184u8, 183u8, 187u8, 34u8,
                        ],
                    )
                }
                #[doc = " Stores the epoch value"]
                pub fn abr_epoch(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "AbrEpoch",
                        vec![],
                        [
                            154u8, 52u8, 58u8, 51u8, 70u8, 170u8, 113u8, 18u8, 124u8, 197u8, 35u8,
                            243u8, 14u8, 137u8, 196u8, 211u8, 134u8, 50u8, 126u8, 225u8, 217u8,
                            21u8, 96u8, 12u8, 114u8, 73u8, 85u8, 221u8, 249u8, 106u8, 59u8, 94u8,
                        ],
                    )
                }
                #[doc = " Stores the ABR Interval"]
                pub fn abr_interval(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "AbrInterval",
                        vec![],
                        [
                            67u8, 161u8, 18u8, 136u8, 25u8, 203u8, 238u8, 120u8, 193u8, 51u8,
                            219u8, 130u8, 243u8, 49u8, 213u8, 243u8, 14u8, 191u8, 189u8, 8u8, 20u8,
                            207u8, 228u8, 40u8, 205u8, 77u8, 117u8, 132u8, 149u8, 170u8, 161u8,
                            97u8,
                        ],
                    )
                }
                #[doc = " Stores the no of users per batch"]
                pub fn users_per_batch(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "UsersPerBatch",
                        vec![],
                        [
                            62u8, 59u8, 45u8, 245u8, 207u8, 205u8, 189u8, 49u8, 12u8, 134u8, 10u8,
                            67u8, 251u8, 122u8, 110u8, 225u8, 165u8, 189u8, 211u8, 144u8, 121u8,
                            240u8, 226u8, 217u8, 15u8, 93u8, 20u8, 248u8, 200u8, 14u8, 150u8,
                            190u8,
                        ],
                    )
                }
                #[doc = " key - Epoch, value - timestamp"]
                pub fn epoch_to_timestamp_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "EpochToTimestampMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            42u8, 173u8, 82u8, 168u8, 56u8, 68u8, 88u8, 37u8, 241u8, 70u8, 253u8,
                            215u8, 63u8, 245u8, 220u8, 237u8, 5u8, 213u8, 171u8, 162u8, 41u8,
                            130u8, 60u8, 120u8, 127u8, 135u8, 26u8, 103u8, 244u8, 142u8, 86u8,
                            209u8,
                        ],
                    )
                }
                #[doc = " key - Epoch, value - timestamp"]
                pub fn epoch_to_timestamp_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "EpochToTimestampMap",
                        Vec::new(),
                        [
                            42u8, 173u8, 82u8, 168u8, 56u8, 68u8, 88u8, 37u8, 241u8, 70u8, 253u8,
                            215u8, 63u8, 245u8, 220u8, 237u8, 5u8, 213u8, 171u8, 162u8, 41u8,
                            130u8, 60u8, 120u8, 127u8, 135u8, 26u8, 103u8, 244u8, 142u8, 86u8,
                            209u8,
                        ],
                    )
                }
                #[doc = " key - Epoch, value - No.of batches"]
                pub fn no_of_batches_for_epoch_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "NoOfBatchesForEpochMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            34u8, 230u8, 88u8, 198u8, 20u8, 58u8, 188u8, 223u8, 192u8, 41u8, 70u8,
                            123u8, 168u8, 21u8, 251u8, 46u8, 168u8, 82u8, 70u8, 72u8, 4u8, 42u8,
                            243u8, 85u8, 66u8, 191u8, 140u8, 148u8, 100u8, 232u8, 138u8, 241u8,
                        ],
                    )
                }
                #[doc = " key - Epoch, value - No.of batches"]
                pub fn no_of_batches_for_epoch_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "NoOfBatchesForEpochMap",
                        Vec::new(),
                        [
                            34u8, 230u8, 88u8, 198u8, 20u8, 58u8, 188u8, 223u8, 192u8, 41u8, 70u8,
                            123u8, 168u8, 21u8, 251u8, 46u8, 168u8, 82u8, 70u8, 72u8, 4u8, 42u8,
                            243u8, 85u8, 66u8, 191u8, 140u8, 148u8, 100u8, 232u8, 138u8, 241u8,
                        ],
                    )
                }
                #[doc = " key - Epoch, value - No.of batches fetched"]
                pub fn batches_fetched_for_epoch_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "BatchesFetchedForEpochMap",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            212u8, 231u8, 189u8, 3u8, 218u8, 129u8, 56u8, 180u8, 177u8, 79u8,
                            148u8, 236u8, 228u8, 85u8, 189u8, 74u8, 173u8, 57u8, 88u8, 205u8,
                            215u8, 10u8, 0u8, 252u8, 142u8, 118u8, 118u8, 203u8, 83u8, 118u8, 27u8,
                            89u8,
                        ],
                    )
                }
                #[doc = " key - Epoch, value - No.of batches fetched"]
                pub fn batches_fetched_for_epoch_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "BatchesFetchedForEpochMap",
                        Vec::new(),
                        [
                            212u8, 231u8, 189u8, 3u8, 218u8, 129u8, 56u8, 180u8, 177u8, 79u8,
                            148u8, 236u8, 228u8, 85u8, 189u8, 74u8, 173u8, 57u8, 88u8, 205u8,
                            215u8, 10u8, 0u8, 252u8, 142u8, 118u8, 118u8, 203u8, 83u8, 118u8, 27u8,
                            89u8,
                        ],
                    )
                }
                #[doc = " key1 - Epoch, Key2 - Market_id, value - ABR value"]
                pub fn epoch_market_to_abr_value_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "EpochMarketToAbrValueMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            189u8, 37u8, 130u8, 188u8, 81u8, 84u8, 235u8, 98u8, 58u8, 201u8, 28u8,
                            248u8, 44u8, 50u8, 6u8, 125u8, 50u8, 189u8, 91u8, 67u8, 120u8, 71u8,
                            216u8, 176u8, 245u8, 55u8, 119u8, 162u8, 8u8, 125u8, 51u8, 190u8,
                        ],
                    )
                }
                #[doc = " key1 - Epoch, Key2 - Market_id, value - ABR value"]
                pub fn epoch_market_to_abr_value_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "EpochMarketToAbrValueMap",
                        Vec::new(),
                        [
                            189u8, 37u8, 130u8, 188u8, 81u8, 84u8, 235u8, 98u8, 58u8, 201u8, 28u8,
                            248u8, 44u8, 50u8, 6u8, 125u8, 50u8, 189u8, 91u8, 67u8, 120u8, 71u8,
                            216u8, 176u8, 245u8, 55u8, 119u8, 162u8, 8u8, 125u8, 51u8, 190u8,
                        ],
                    )
                }
                #[doc = " key1 - Epoch, Key2 - Market_id, value - Last market price"]
                pub fn epoch_market_to_last_price_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "EpochMarketToLastPriceMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            13u8, 244u8, 92u8, 11u8, 127u8, 247u8, 95u8, 235u8, 199u8, 182u8,
                            246u8, 204u8, 191u8, 212u8, 56u8, 4u8, 11u8, 20u8, 216u8, 48u8, 244u8,
                            157u8, 92u8, 199u8, 17u8, 16u8, 166u8, 144u8, 122u8, 220u8, 90u8,
                            187u8,
                        ],
                    )
                }
                #[doc = " key1 - Epoch, Key2 - Market_id, value - Last market price"]
                pub fn epoch_market_to_last_price_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "EpochMarketToLastPriceMap",
                        Vec::new(),
                        [
                            13u8, 244u8, 92u8, 11u8, 127u8, 247u8, 95u8, 235u8, 199u8, 182u8,
                            246u8, 204u8, 191u8, 212u8, 56u8, 4u8, 11u8, 20u8, 216u8, 48u8, 244u8,
                            157u8, 92u8, 199u8, 17u8, 16u8, 166u8, 144u8, 122u8, 220u8, 90u8,
                            187u8,
                        ],
                    )
                }
                #[doc = " key1 - Epoch, Key2 - Market_id, value - Status of the market"]
                pub fn abr_market_status_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "AbrMarketStatusMap",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            233u8, 202u8, 40u8, 14u8, 10u8, 97u8, 29u8, 88u8, 87u8, 22u8, 21u8,
                            42u8, 64u8, 36u8, 156u8, 173u8, 196u8, 128u8, 134u8, 159u8, 101u8,
                            199u8, 145u8, 134u8, 239u8, 55u8, 105u8, 4u8, 209u8, 122u8, 207u8,
                            20u8,
                        ],
                    )
                }
                #[doc = " key1 - Epoch, Key2 - Market_id, value - Status of the market"]
                pub fn abr_market_status_map_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "AbrMarketStatusMap",
                        Vec::new(),
                        [
                            233u8, 202u8, 40u8, 14u8, 10u8, 97u8, 29u8, 88u8, 87u8, 22u8, 21u8,
                            42u8, 64u8, 36u8, 156u8, 173u8, 196u8, 128u8, 134u8, 159u8, 101u8,
                            199u8, 145u8, 134u8, 239u8, 55u8, 105u8, 4u8, 209u8, 122u8, 207u8,
                            20u8,
                        ],
                    )
                }
                #[doc = " Stores the base ABR"]
                pub fn base_abr(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "BaseAbr",
                        vec![],
                        [
                            166u8, 242u8, 117u8, 18u8, 175u8, 13u8, 74u8, 100u8, 198u8, 64u8,
                            189u8, 38u8, 128u8, 92u8, 97u8, 230u8, 25u8, 121u8, 225u8, 176u8, 92u8,
                            252u8, 58u8, 216u8, 234u8, 146u8, 183u8, 109u8, 168u8, 100u8, 20u8,
                            62u8,
                        ],
                    )
                }
                #[doc = " Stores the bollinger width"]
                pub fn bollinger_width(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "BollingerWidth",
                        vec![],
                        [
                            11u8, 54u8, 179u8, 115u8, 119u8, 8u8, 102u8, 95u8, 212u8, 106u8, 6u8,
                            6u8, 176u8, 77u8, 218u8, 171u8, 15u8, 133u8, 166u8, 200u8, 174u8, 72u8,
                            117u8, 97u8, 160u8, 182u8, 236u8, 151u8, 117u8, 224u8, 181u8, 102u8,
                        ],
                    )
                }
                pub fn max_abr_per_market(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "MaxABRPerMarket",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            255u8, 27u8, 46u8, 42u8, 97u8, 94u8, 212u8, 206u8, 52u8, 35u8, 202u8,
                            234u8, 134u8, 105u8, 12u8, 238u8, 204u8, 157u8, 194u8, 152u8, 48u8,
                            24u8, 231u8, 146u8, 88u8, 26u8, 110u8, 226u8, 151u8, 2u8, 223u8, 103u8,
                        ],
                    )
                }
                pub fn max_abr_per_market_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "MaxABRPerMarket",
                        Vec::new(),
                        [
                            255u8, 27u8, 46u8, 42u8, 97u8, 94u8, 212u8, 206u8, 52u8, 35u8, 202u8,
                            234u8, 134u8, 105u8, 12u8, 238u8, 204u8, 157u8, 194u8, 152u8, 48u8,
                            24u8, 231u8, 146u8, 88u8, 26u8, 110u8, 226u8, 151u8, 2u8, 223u8, 103u8,
                        ],
                    )
                }
                pub fn max_abr_default(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Prices",
                        "MaxABRDefault",
                        vec![],
                        [
                            189u8, 247u8, 228u8, 22u8, 108u8, 166u8, 33u8, 38u8, 93u8, 215u8, 56u8,
                            65u8, 111u8, 227u8, 11u8, 81u8, 123u8, 132u8, 240u8, 240u8, 234u8,
                            218u8, 89u8, 18u8, 63u8, 172u8, 142u8, 110u8, 162u8, 169u8, 31u8, 56u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod risk_management {
        use super::root_mod;
        use super::runtime_types;
    }
    pub mod node_authorization {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_node_authorization::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_node_authorization::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddWellKnownNode {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                    pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddWellKnownNode {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "add_well_known_node";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveWellKnownNode {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveWellKnownNode {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "remove_well_known_node";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SwapWellKnownNode {
                    pub remove: runtime_types::sp_core::OpaquePeerId,
                    pub add: runtime_types::sp_core::OpaquePeerId,
                }
                impl ::subxt::blocks::StaticExtrinsic for SwapWellKnownNode {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "swap_well_known_node";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ResetWellKnownNodes {
                    pub nodes: ::std::vec::Vec<(
                        runtime_types::sp_core::OpaquePeerId,
                        ::subxt::utils::AccountId32,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for ResetWellKnownNodes {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "reset_well_known_nodes";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ClaimNode {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                }
                impl ::subxt::blocks::StaticExtrinsic for ClaimNode {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "claim_node";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveClaim {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveClaim {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "remove_claim";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferNode {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                    pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferNode {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "transfer_node";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddConnections {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                    pub connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddConnections {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "add_connections";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveConnections {
                    pub node: runtime_types::sp_core::OpaquePeerId,
                    pub connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveConnections {
                    const PALLET: &'static str = "NodeAuthorization";
                    const CALL: &'static str = "remove_connections";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::add_well_known_node`]."]
                pub fn add_well_known_node(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                    owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::AddWellKnownNode> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "add_well_known_node",
                        types::AddWellKnownNode { node, owner },
                        [
                            222u8, 218u8, 8u8, 102u8, 17u8, 146u8, 163u8, 216u8, 160u8, 37u8,
                            239u8, 108u8, 255u8, 255u8, 61u8, 35u8, 82u8, 139u8, 128u8, 138u8,
                            58u8, 151u8, 73u8, 97u8, 63u8, 58u8, 65u8, 40u8, 197u8, 35u8, 3u8,
                            58u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::remove_well_known_node`]."]
                pub fn remove_well_known_node(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                ) -> ::subxt::tx::Payload<types::RemoveWellKnownNode> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "remove_well_known_node",
                        types::RemoveWellKnownNode { node },
                        [
                            237u8, 162u8, 84u8, 156u8, 139u8, 61u8, 114u8, 59u8, 187u8, 46u8, 9u8,
                            217u8, 90u8, 43u8, 9u8, 66u8, 122u8, 65u8, 249u8, 177u8, 143u8, 114u8,
                            1u8, 203u8, 160u8, 248u8, 127u8, 33u8, 125u8, 44u8, 214u8, 109u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::swap_well_known_node`]."]
                pub fn swap_well_known_node(
                    &self,
                    remove: runtime_types::sp_core::OpaquePeerId,
                    add: runtime_types::sp_core::OpaquePeerId,
                ) -> ::subxt::tx::Payload<types::SwapWellKnownNode> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "swap_well_known_node",
                        types::SwapWellKnownNode { remove, add },
                        [
                            254u8, 216u8, 205u8, 83u8, 75u8, 165u8, 59u8, 22u8, 221u8, 140u8,
                            227u8, 18u8, 203u8, 150u8, 27u8, 161u8, 57u8, 194u8, 60u8, 53u8, 75u8,
                            8u8, 66u8, 79u8, 102u8, 15u8, 74u8, 181u8, 52u8, 167u8, 77u8, 124u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::reset_well_known_nodes`]."]
                pub fn reset_well_known_nodes(
                    &self,
                    nodes: ::std::vec::Vec<(
                        runtime_types::sp_core::OpaquePeerId,
                        ::subxt::utils::AccountId32,
                    )>,
                ) -> ::subxt::tx::Payload<types::ResetWellKnownNodes> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "reset_well_known_nodes",
                        types::ResetWellKnownNodes { nodes },
                        [
                            13u8, 48u8, 24u8, 13u8, 25u8, 206u8, 124u8, 84u8, 101u8, 76u8, 217u8,
                            190u8, 141u8, 92u8, 11u8, 218u8, 48u8, 129u8, 255u8, 71u8, 81u8, 2u8,
                            16u8, 218u8, 123u8, 120u8, 9u8, 210u8, 149u8, 140u8, 96u8, 87u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::claim_node`]."]
                pub fn claim_node(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                ) -> ::subxt::tx::Payload<types::ClaimNode> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "claim_node",
                        types::ClaimNode { node },
                        [
                            211u8, 12u8, 121u8, 181u8, 113u8, 98u8, 37u8, 201u8, 246u8, 95u8, 9u8,
                            6u8, 123u8, 23u8, 244u8, 84u8, 247u8, 96u8, 122u8, 213u8, 73u8, 185u8,
                            214u8, 199u8, 253u8, 10u8, 25u8, 178u8, 240u8, 11u8, 30u8, 182u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::remove_claim`]."]
                pub fn remove_claim(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                ) -> ::subxt::tx::Payload<types::RemoveClaim> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "remove_claim",
                        types::RemoveClaim { node },
                        [
                            171u8, 88u8, 189u8, 27u8, 199u8, 205u8, 26u8, 132u8, 162u8, 79u8, 55u8,
                            242u8, 6u8, 32u8, 108u8, 47u8, 56u8, 201u8, 31u8, 130u8, 70u8, 43u8,
                            134u8, 118u8, 22u8, 31u8, 235u8, 178u8, 63u8, 180u8, 54u8, 49u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer_node`]."]
                pub fn transfer_node(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                    owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::TransferNode> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "transfer_node",
                        types::TransferNode { node, owner },
                        [
                            188u8, 194u8, 87u8, 98u8, 186u8, 120u8, 30u8, 63u8, 167u8, 155u8,
                            227u8, 10u8, 196u8, 80u8, 119u8, 230u8, 211u8, 242u8, 240u8, 197u8,
                            211u8, 151u8, 119u8, 15u8, 14u8, 77u8, 84u8, 9u8, 23u8, 80u8, 129u8,
                            19u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::add_connections`]."]
                pub fn add_connections(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                    connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                ) -> ::subxt::tx::Payload<types::AddConnections> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "add_connections",
                        types::AddConnections { node, connections },
                        [
                            54u8, 201u8, 157u8, 160u8, 161u8, 99u8, 195u8, 229u8, 18u8, 207u8,
                            237u8, 63u8, 56u8, 242u8, 86u8, 132u8, 125u8, 89u8, 40u8, 211u8, 156u8,
                            137u8, 232u8, 34u8, 222u8, 11u8, 53u8, 190u8, 158u8, 12u8, 17u8, 103u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::remove_connections`]."]
                pub fn remove_connections(
                    &self,
                    node: runtime_types::sp_core::OpaquePeerId,
                    connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                ) -> ::subxt::tx::Payload<types::RemoveConnections> {
                    ::subxt::tx::Payload::new_static(
                        "NodeAuthorization",
                        "remove_connections",
                        types::RemoveConnections { node, connections },
                        [
                            146u8, 134u8, 47u8, 221u8, 191u8, 181u8, 22u8, 68u8, 124u8, 143u8,
                            131u8, 230u8, 249u8, 63u8, 60u8, 240u8, 58u8, 85u8, 63u8, 173u8, 21u8,
                            193u8, 134u8, 74u8, 124u8, 107u8, 33u8, 42u8, 28u8, 126u8, 234u8,
                            189u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_node_authorization::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The given well known node was added."]
            pub struct NodeAdded {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NodeAdded {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "NodeAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The given well known node was removed."]
            pub struct NodeRemoved {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
            }
            impl ::subxt::events::StaticEvent for NodeRemoved {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "NodeRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The given well known node was swapped; first item was removed,"]
            #[doc = "the latter was added."]
            pub struct NodeSwapped {
                pub removed: runtime_types::sp_core::OpaquePeerId,
                pub added: runtime_types::sp_core::OpaquePeerId,
            }
            impl ::subxt::events::StaticEvent for NodeSwapped {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "NodeSwapped";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The given well known nodes were reset."]
            pub struct NodesReset {
                pub nodes: ::std::vec::Vec<(
                    runtime_types::sp_core::OpaquePeerId,
                    ::subxt::utils::AccountId32,
                )>,
            }
            impl ::subxt::events::StaticEvent for NodesReset {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "NodesReset";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The given node was claimed by a user."]
            pub struct NodeClaimed {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NodeClaimed {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "NodeClaimed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The given claim was removed by its owner."]
            pub struct ClaimRemoved {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ClaimRemoved {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "ClaimRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The node was transferred to another account."]
            pub struct NodeTransferred {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
                pub target: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NodeTransferred {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "NodeTransferred";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The allowed connections were added to a node."]
            pub struct ConnectionsAdded {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
                pub allowed_connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
            }
            impl ::subxt::events::StaticEvent for ConnectionsAdded {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "ConnectionsAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The allowed connections were removed from a node."]
            pub struct ConnectionsRemoved {
                pub peer_id: runtime_types::sp_core::OpaquePeerId,
                pub allowed_connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
            }
            impl ::subxt::events::StaticEvent for ConnectionsRemoved {
                const PALLET: &'static str = "NodeAuthorization";
                const EVENT: &'static str = "ConnectionsRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The set of well known nodes. This is stored sorted (just by value)."]
                pub fn well_known_nodes(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "NodeAuthorization",
                        "WellKnownNodes",
                        vec![],
                        [
                            33u8, 218u8, 32u8, 18u8, 78u8, 255u8, 32u8, 34u8, 130u8, 170u8, 47u8,
                            227u8, 232u8, 232u8, 106u8, 35u8, 30u8, 219u8, 74u8, 187u8, 194u8,
                            86u8, 24u8, 174u8, 2u8, 13u8, 167u8, 74u8, 237u8, 206u8, 126u8, 44u8,
                        ],
                    )
                }
                #[doc = " A map that maintains the ownership of each node."]
                pub fn owners(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::sp_core::OpaquePeerId>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "NodeAuthorization",
                        "Owners",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            92u8, 140u8, 179u8, 76u8, 3u8, 125u8, 246u8, 188u8, 159u8, 7u8, 1u8,
                            120u8, 153u8, 47u8, 1u8, 194u8, 146u8, 249u8, 122u8, 40u8, 115u8,
                            129u8, 92u8, 252u8, 144u8, 141u8, 75u8, 242u8, 51u8, 14u8, 46u8, 226u8,
                        ],
                    )
                }
                #[doc = " A map that maintains the ownership of each node."]
                pub fn owners_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "NodeAuthorization",
                        "Owners",
                        Vec::new(),
                        [
                            92u8, 140u8, 179u8, 76u8, 3u8, 125u8, 246u8, 188u8, 159u8, 7u8, 1u8,
                            120u8, 153u8, 47u8, 1u8, 194u8, 146u8, 249u8, 122u8, 40u8, 115u8,
                            129u8, 92u8, 252u8, 144u8, 141u8, 75u8, 242u8, 51u8, 14u8, 46u8, 226u8,
                        ],
                    )
                }
                #[doc = " The additional adapative connections of each node."]
                pub fn additional_connections(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::sp_core::OpaquePeerId>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "NodeAuthorization",
                        "AdditionalConnections",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            109u8, 141u8, 249u8, 81u8, 209u8, 63u8, 86u8, 178u8, 81u8, 246u8,
                            242u8, 204u8, 87u8, 11u8, 151u8, 198u8, 107u8, 231u8, 92u8, 191u8,
                            168u8, 150u8, 126u8, 185u8, 93u8, 88u8, 134u8, 141u8, 74u8, 215u8,
                            132u8, 192u8,
                        ],
                    )
                }
                #[doc = " The additional adapative connections of each node."]
                pub fn additional_connections_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "NodeAuthorization",
                        "AdditionalConnections",
                        Vec::new(),
                        [
                            109u8, 141u8, 249u8, 81u8, 209u8, 63u8, 86u8, 178u8, 81u8, 246u8,
                            242u8, 204u8, 87u8, 11u8, 151u8, 198u8, 107u8, 231u8, 92u8, 191u8,
                            168u8, 150u8, 126u8, 185u8, 93u8, 88u8, 134u8, 141u8, 74u8, 215u8,
                            132u8, 192u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The maximum number of well known nodes that are allowed to set"]
                pub fn max_well_known_nodes(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "NodeAuthorization",
                        "MaxWellKnownNodes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum length in bytes of PeerId"]
                pub fn max_peer_id_length(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "NodeAuthorization",
                        "MaxPeerIdLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::remark`]."]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::set_heap_pages`]."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::set_code`]."]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::set_code_without_checks`]."]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::set_storage`]."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::kill_storage`]."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::kill_prefix`]."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::remark_with_event`]."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod node_template_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SessionKeys {
                    pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 3)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Call),
                #[codec(index = 4)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 6)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 8)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 9)]
                TradingAccount(runtime_types::pallet_trading_account::pallet::Call),
                #[codec(index = 10)]
                Assets(runtime_types::pallet_asset::pallet::Call),
                #[codec(index = 11)]
                Markets(runtime_types::pallet_market::pallet::Call),
                #[codec(index = 12)]
                SyncFacade(runtime_types::pallet_sync_facade::pallet::Call),
                #[codec(index = 13)]
                Trading(runtime_types::pallet_trading::pallet::Call),
                #[codec(index = 14)]
                TradingFees(runtime_types::pallet_trading_fees::pallet::Call),
                #[codec(index = 15)]
                Prices(runtime_types::pallet_prices::pallet::Call),
                #[codec(index = 17)]
                NodeAuthorization(runtime_types::pallet_node_authorization::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 3)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Error),
                #[codec(index = 4)]
                Session(runtime_types::pallet_session::pallet::Error),
                #[codec(index = 6)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 8)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 9)]
                TradingAccount(runtime_types::pallet_trading_account::pallet::Error),
                #[codec(index = 10)]
                Assets(runtime_types::pallet_asset::pallet::Error),
                #[codec(index = 11)]
                Markets(runtime_types::pallet_market::pallet::Error),
                #[codec(index = 12)]
                SyncFacade(runtime_types::pallet_sync_facade::pallet::Error),
                #[codec(index = 13)]
                Trading(runtime_types::pallet_trading::pallet::Error),
                #[codec(index = 14)]
                TradingFees(runtime_types::pallet_trading_fees::pallet::Error),
                #[codec(index = 15)]
                Prices(runtime_types::pallet_prices::pallet::Error),
                #[codec(index = 17)]
                NodeAuthorization(runtime_types::pallet_node_authorization::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 3)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Event),
                #[codec(index = 4)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 6)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 7)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 8)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 9)]
                TradingAccount(runtime_types::pallet_trading_account::pallet::Event),
                #[codec(index = 10)]
                Assets(runtime_types::pallet_asset::pallet::Event),
                #[codec(index = 11)]
                Markets(runtime_types::pallet_market::pallet::Event),
                #[codec(index = 12)]
                SyncFacade(runtime_types::pallet_sync_facade::pallet::Event),
                #[codec(index = 13)]
                Trading(runtime_types::pallet_trading::pallet::Event),
                #[codec(index = 14)]
                TradingFees(runtime_types::pallet_trading_fees::pallet::Event),
                #[codec(index = 15)]
                Prices(runtime_types::pallet_prices::pallet::Event),
                #[codec(index = 17)]
                NodeAuthorization(runtime_types::pallet_node_authorization::pallet::Event),
            }
        }
        pub mod pallet_asset {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See `Pallet::consume_nonce`."]
                    consume_nonce,
                    #[codec(index = 1)]
                    #[doc = "See `Pallet::replace_all_assets`."]
                    replace_all_assets {
                        assets: ::std::vec::Vec<
                            runtime_types::pallet_support::types::asset::ExtendedAsset,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "See `Pallet::remove_asset`."]
                    remove_asset { id: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "See `Pallet::update_asset`."]
                    update_asset {
                        extended_asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                    },
                    #[codec(index = 4)]
                    #[doc = "See `Pallet::add_asset`."]
                    add_asset {
                        extended_asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Each asset must have a unique identifier"]
                    DuplicateAsset,
                    #[codec(index = 1)]
                    #[doc = "The total supply of assets can't exceed the u64 limit"]
                    BoundsOverflow,
                    #[codec(index = 2)]
                    #[doc = "Invalid value for id or token decimal"]
                    InvalidAsset,
                    #[codec(index = 3)]
                    #[doc = "Invalid Call to dev mode only function"]
                    DevOnlyCall,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    AssetCreated {
                        asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                    },
                    #[codec(index = 1)]
                    AssetUpdated {
                        asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                    },
                    #[codec(index = 2)]
                    AssetRemoved {
                        asset: runtime_types::pallet_support::types::asset::ExtendedAsset,
                    },
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::transfer_allow_death`]."]
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::set_balance_deprecated`]."]
                    set_balance_deprecated {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        old_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::force_transfer`]."]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::transfer_keep_alive`]."]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::transfer_all`]."]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::force_unreserve`]."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::upgrade_accounts`]."]
                    upgrade_accounts {
                        who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::transfer`]."]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::force_set_balance`]."]
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `MaxHolds`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::report_equivocation`]."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::report_equivocation_unsigned`]."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::note_stalled`]."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_market {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See `Pallet::replace_all_markets`."]
                    replace_all_markets {
                        markets: ::std::vec::Vec<
                            runtime_types::pallet_support::types::market::ExtendedMarket,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "See `Pallet::add_market`."]
                    add_market {
                        extended_market:
                            runtime_types::pallet_support::types::market::ExtendedMarket,
                    },
                    #[codec(index = 2)]
                    #[doc = "See `Pallet::update_market`."]
                    update_market {
                        extended_market:
                            runtime_types::pallet_support::types::market::ExtendedMarket,
                    },
                    #[codec(index = 3)]
                    #[doc = "See `Pallet::remove_market`."]
                    remove_market { id: ::core::primitive::u128 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Each market must have a unique identifier"]
                    DuplicateMarket,
                    #[codec(index = 1)]
                    #[doc = "The total supply of markets can't exceed the u64 limit"]
                    BoundsOverflow,
                    #[codec(index = 2)]
                    #[doc = "Invalid value for Market Id"]
                    InvalidMarket,
                    #[codec(index = 3)]
                    #[doc = "Invalid value for is_tradable field"]
                    InvalidTradableFlag,
                    #[codec(index = 4)]
                    #[doc = "Asset not created"]
                    AssetNotFound,
                    #[codec(index = 5)]
                    #[doc = "Asset provided as collateral is not marked as collateral in the system"]
                    AssetNotCollateral,
                    #[codec(index = 6)]
                    #[doc = "Invalid value for max leverage or currently allowed leverage"]
                    InvalidLeverage,
                    #[codec(index = 7)]
                    #[doc = "Unauthorized attempt to update markets"]
                    NotAdmin,
                    #[codec(index = 8)]
                    #[doc = "Invalid Call to dev mode only function"]
                    DevOnlyCall,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Market successfully created"]
                    MarketCreated {
                        market: runtime_types::pallet_support::types::market::ExtendedMarket,
                    },
                    #[codec(index = 1)]
                    #[doc = "Market successfully updated"]
                    MarketUpdated {
                        market: runtime_types::pallet_support::types::market::ExtendedMarket,
                    },
                    #[codec(index = 2)]
                    #[doc = "Market successfully removed"]
                    MarketRemoved {
                        market: runtime_types::pallet_support::types::market::ExtendedMarket,
                    },
                }
            }
        }
        pub mod pallet_node_authorization {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::add_well_known_node`]."]
                    add_well_known_node {
                        node: runtime_types::sp_core::OpaquePeerId,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::remove_well_known_node`]."]
                    remove_well_known_node {
                        node: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::swap_well_known_node`]."]
                    swap_well_known_node {
                        remove: runtime_types::sp_core::OpaquePeerId,
                        add: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::reset_well_known_nodes`]."]
                    reset_well_known_nodes {
                        nodes: ::std::vec::Vec<(
                            runtime_types::sp_core::OpaquePeerId,
                            ::subxt::utils::AccountId32,
                        )>,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::claim_node`]."]
                    claim_node {
                        node: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::remove_claim`]."]
                    remove_claim {
                        node: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::transfer_node`]."]
                    transfer_node {
                        node: runtime_types::sp_core::OpaquePeerId,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::add_connections`]."]
                    add_connections {
                        node: runtime_types::sp_core::OpaquePeerId,
                        connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::remove_connections`]."]
                    remove_connections {
                        node: runtime_types::sp_core::OpaquePeerId,
                        connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The PeerId is too long."]
                    PeerIdTooLong,
                    #[codec(index = 1)]
                    #[doc = "Too many well known nodes."]
                    TooManyNodes,
                    #[codec(index = 2)]
                    #[doc = "The node is already joined in the list."]
                    AlreadyJoined,
                    #[codec(index = 3)]
                    #[doc = "The node doesn't exist in the list."]
                    NotExist,
                    #[codec(index = 4)]
                    #[doc = "The node is already claimed by a user."]
                    AlreadyClaimed,
                    #[codec(index = 5)]
                    #[doc = "The node hasn't been claimed yet."]
                    NotClaimed,
                    #[codec(index = 6)]
                    #[doc = "You are not the owner of the node."]
                    NotOwner,
                    #[codec(index = 7)]
                    #[doc = "No permisson to perform specific operation."]
                    PermissionDenied,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The given well known node was added."]
                    NodeAdded {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "The given well known node was removed."]
                    NodeRemoved {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 2)]
                    #[doc = "The given well known node was swapped; first item was removed,"]
                    #[doc = "the latter was added."]
                    NodeSwapped {
                        removed: runtime_types::sp_core::OpaquePeerId,
                        added: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 3)]
                    #[doc = "The given well known nodes were reset."]
                    NodesReset {
                        nodes: ::std::vec::Vec<(
                            runtime_types::sp_core::OpaquePeerId,
                            ::subxt::utils::AccountId32,
                        )>,
                    },
                    #[codec(index = 4)]
                    #[doc = "The given node was claimed by a user."]
                    NodeClaimed {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "The given claim was removed by its owner."]
                    ClaimRemoved {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "The node was transferred to another account."]
                    NodeTransferred {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                        target: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "The allowed connections were added to a node."]
                    ConnectionsAdded {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                        allowed_connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    },
                    #[codec(index = 8)]
                    #[doc = "The allowed connections were removed from a node."]
                    ConnectionsRemoved {
                        peer_id: runtime_types::sp_core::OpaquePeerId,
                        allowed_connections: ::std::vec::Vec<runtime_types::sp_core::OpaquePeerId>,
                    },
                }
            }
        }
        pub mod pallet_prices {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See `Pallet::set_initialisation_timestamp`."]
                    set_initialisation_timestamp { timestamp: ::core::primitive::u64 },
                    #[codec(index = 1)]
                    #[doc = "See `Pallet::set_default_max_abr`."]
                    set_default_max_abr {
                        max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 2)]
                    #[doc = "See `Pallet::set_max_abr`."]
                    set_max_abr {
                        market_id: ::core::primitive::u128,
                        max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 3)]
                    #[doc = "See `Pallet::set_abr_interval`."]
                    set_abr_interval {
                        new_abr_interval: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    #[doc = "See `Pallet::set_base_abr`."]
                    set_base_abr {
                        new_base_abr: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 5)]
                    #[doc = "See `Pallet::set_bollinger_width`."]
                    set_bollinger_width {
                        new_bollinger_width: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 6)]
                    #[doc = "See `Pallet::set_no_of_users_per_batch`."]
                    set_no_of_users_per_batch {
                        new_no_of_users_per_batch: ::core::primitive::u64,
                    },
                    #[codec(index = 7)]
                    #[doc = "See `Pallet::set_abr_value`."]
                    set_abr_value { market_id: ::core::primitive::u128 },
                    #[codec(index = 8)]
                    #[doc = "See `Pallet::make_abr_payments`."]
                    make_abr_payments,
                    #[codec(index = 9)]
                    #[doc = "See `Pallet::update_prices`."]
                    update_prices {
                        prices: ::std::vec::Vec<
                            runtime_types::pallet_support::types::prices::MultiplePrices,
                        >,
                        timestamp: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    #[doc = "See `Pallet::perform_prices_cleanup`."]
                    perform_prices_cleanup,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid value for price"]
                    InvalidPrice,
                    #[codec(index = 1)]
                    #[doc = "Invalid value for Market Id"]
                    MarketNotFound,
                    #[codec(index = 2)]
                    #[doc = "Price interval should be >= 1 second"]
                    InvalidPriceInterval,
                    #[codec(index = 3)]
                    #[doc = "When no.of users per batch provided is invalid"]
                    InvalidUsersPerBatch,
                    #[codec(index = 4)]
                    #[doc = "When ABR interval provided is invalid"]
                    InvalidAbrInterval,
                    #[codec(index = 5)]
                    #[doc = "When timestamp provided is invalid"]
                    InvalidTimestamp,
                    #[codec(index = 6)]
                    #[doc = "When ABR state is invalid"]
                    InvalidState,
                    #[codec(index = 7)]
                    #[doc = "When ABR value is already set for the market"]
                    AbrValueAlreadySet,
                    #[codec(index = 8)]
                    #[doc = "When market provided is not tradable"]
                    MarketNotTradable,
                    #[codec(index = 9)]
                    #[doc = "When interval provided for setting ABR price is invalid"]
                    InvalidAbrPriceInterval,
                    #[codec(index = 10)]
                    #[doc = "When Base ABR provided is not within the range"]
                    InvalidBaseAbr,
                    #[codec(index = 11)]
                    #[doc = "When bollinger width provided is invalid"]
                    InvalidBollingerWidth,
                    #[codec(index = 12)]
                    #[doc = "Invalid value for initialisation timestamp"]
                    InvalidInitialisationTimestamp,
                    #[codec(index = 13)]
                    #[doc = "Set ABR value called before the abr interval is met"]
                    EarlyAbrCall,
                    #[codec(index = 14)]
                    #[doc = "When initialisation timestamp is already set"]
                    InitialisationTimestampAlreadySet,
                    #[codec(index = 15)]
                    #[doc = "When negative max value is passed to set_max_abr"]
                    NegativeMaxValue,
                    #[codec(index = 16)]
                    #[doc = "When timestamp provided is not yet met"]
                    FutureTimestampPriceUpdate,
                    #[codec(index = 17)]
                    #[doc = "Prices Start timestamp is not set"]
                    PricesStartTimestampEmpty,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Last traded price was successfully updated"]
                    LastOraclePriceUpdated {
                        market_id: ::core::primitive::u128,
                        price: runtime_types::pallet_support::types::prices::LastOraclePrice,
                    },
                    #[codec(index = 1)]
                    #[doc = "ABR timestamp set successfully"]
                    AbrTimestampSet {
                        epoch: ::core::primitive::u64,
                        timestamp: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "ABR state changed successfully"]
                    AbrStateChanged {
                        epoch: ::core::primitive::u64,
                        state: runtime_types::pallet_support::types::abr::ABRState,
                    },
                    #[codec(index = 3)]
                    #[doc = "ABR value set successfully"]
                    AbrValueSet {
                        epoch: ::core::primitive::u64,
                        market_id: ::core::primitive::u128,
                        abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        abr_last_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 4)]
                    #[doc = "ABR payment made successfully"]
                    AbrPaymentMade {
                        epoch: ::core::primitive::u64,
                        batch_id: ::core::primitive::u64,
                    },
                    #[codec(index = 5)]
                    #[doc = "ABR payment for a user made successfully"]
                    UserAbrPayment {
                        account_id: runtime_types::primitive_types::U256,
                        market_id: ::core::primitive::u128,
                        collateral_id: ::core::primitive::u128,
                        abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        abr_timestamp: ::core::primitive::u64,
                        amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        modify_type: runtime_types::pallet_support::types::trading::FundModifyType,
                        position_size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 6)]
                    #[doc = "ABR interval updated successfully"]
                    AbrIntervalUpdated {
                        abr_interval: ::core::primitive::u64,
                    },
                    #[codec(index = 7)]
                    #[doc = "Default Max ABR value updated successfully"]
                    DefaultMaxAbrUpdated {
                        max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Max ABR value of a market updated successfully"]
                    MaxAbrForMarketUpdated {
                        market_id: ::core::primitive::u128,
                        max_abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Initialisation timestamp updated successfully"]
                    InitialisationTimestampUpdated { timestamp: ::core::primitive::u64 },
                    #[codec(index = 10)]
                    #[doc = "No of users per batch updated successfully"]
                    NoOfUsersPerBatchUpdated {
                        no_of_users_per_batch: ::core::primitive::u64,
                    },
                    #[codec(index = 11)]
                    #[doc = "Base ABR updated successfully"]
                    BaseAbrUpdated {
                        base_abr: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Bollinger width updated successfully"]
                    BollingerWidthUpdated {
                        bollinger_width: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Index/mark prices updated successfully"]
                    PricesUpdated {
                        timestamp: ::core::primitive::u64,
                        prices: ::std::vec::Vec<
                            runtime_types::pallet_support::types::prices::MultiplePrices,
                        >,
                    },
                }
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::set_keys`]."]
                    set_keys {
                        keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::purge_keys`]."]
                    purge_keys,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the session pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid ownership proof."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "No associated validator ID for account."]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    #[doc = "Registered duplicate key."]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    #[doc = "No keys are associated with this account."]
                    NoKeys,
                    #[codec(index = 4)]
                    #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::sudo`]."]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::sudo_unchecked_weight`]."]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::RuntimeCall>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::set_key`]."]
                    set_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::sudo_as`]."]
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::RuntimeCall>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_support {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod abr {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct ABRDetails {
                        pub abr_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub abr_timestamp: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum ABRState {
                        #[codec(index = 0)]
                        State0,
                        #[codec(index = 1)]
                        State1,
                        #[codec(index = 2)]
                        State2,
                    }
                }
                pub mod asset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Asset {
                        pub id: ::core::primitive::u128,
                        pub version: ::core::primitive::u16,
                        pub short_name: runtime_types::primitive_types::U256,
                        pub is_tradable: ::core::primitive::bool,
                        pub is_collateral: ::core::primitive::bool,
                        pub decimals: ::core::primitive::u8,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct AssetAddress {
                        pub chain: ::core::primitive::u128,
                        pub address: runtime_types::primitive_types::U256,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct ExtendedAsset {
                        pub asset: runtime_types::pallet_support::types::asset::Asset,
                        pub asset_addresses:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                runtime_types::pallet_support::types::asset::AssetAddress,
                            >,
                        pub metadata_url:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                    }
                }
                pub mod common {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum HashType {
                        #[codec(index = 0)]
                        Pedersen,
                        #[codec(index = 1)]
                        Poseidon,
                    }
                }
                pub mod market {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct ExtendedMarket {
                        pub market: runtime_types::pallet_support::types::market::Market,
                        pub metadata_url:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Market {
                        pub id: ::core::primitive::u128,
                        pub version: ::core::primitive::u16,
                        pub asset: ::core::primitive::u128,
                        pub asset_collateral: ::core::primitive::u128,
                        pub is_tradable: ::core::primitive::bool,
                        pub is_archived: ::core::primitive::bool,
                        pub ttl: ::core::primitive::u32,
                        pub tick_size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub tick_precision: ::core::primitive::u8,
                        pub step_size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub step_precision: ::core::primitive::u8,
                        pub minimum_order_size:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub minimum_leverage: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub maximum_leverage: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub currently_allowed_leverage:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub maintenance_margin_fraction:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub initial_margin_fraction:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub incremental_initial_margin_fraction:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub incremental_position_size:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub baseline_position_size:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub maximum_position_size:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                }
                pub mod prices {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CurrentPrice {
                        pub timestamp: ::core::primitive::u64,
                        pub index_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub mark_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct HistoricalPrice {
                        pub index_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub mark_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct LastOraclePrice {
                        pub timestamp: ::core::primitive::u64,
                        pub price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiplePrices {
                        pub market_id: ::core::primitive::u128,
                        pub index_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub mark_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                }
                pub mod sync_facade {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct AssetRemoved {
                        pub event_index: ::core::primitive::u32,
                        pub id: ::core::primitive::u128,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct AssetUpdated {
                        pub event_index: ::core::primitive::u32,
                        pub id: ::core::primitive::u128,
                        pub asset: runtime_types::pallet_support::types::asset::Asset,
                        pub asset_addresses:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                runtime_types::pallet_support::types::asset::AssetAddress,
                            >,
                        pub metadata_url:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum FeeSettingsType {
                        #[codec(index = 0)]
                        MakerVols,
                        #[codec(index = 1)]
                        TakerVols,
                        #[codec(index = 2)]
                        MakerOpen,
                        #[codec(index = 3)]
                        MakerClose,
                        #[codec(index = 4)]
                        TakerOpen,
                        #[codec(index = 5)]
                        TakerClose,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MarketRemoved {
                        pub event_index: ::core::primitive::u32,
                        pub id: ::core::primitive::u128,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MarketUpdated {
                        pub event_index: ::core::primitive::u32,
                        pub id: ::core::primitive::u128,
                        pub market: runtime_types::pallet_support::types::market::Market,
                        pub metadata_url:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct QuorumSet {
                        pub event_index: ::core::primitive::u32,
                        pub quorum: ::core::primitive::u8,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Setting {
                        pub key: runtime_types::primitive_types::U256,
                        pub values: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct SettingsAdded {
                        pub event_index: ::core::primitive::u32,
                        pub settings: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_support::types::sync_facade::Setting,
                        >,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct SignerAdded {
                        pub event_index: ::core::primitive::u32,
                        pub signer: runtime_types::primitive_types::U256,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct SignerRemoved {
                        pub event_index: ::core::primitive::u32,
                        pub signer: runtime_types::primitive_types::U256,
                        pub block_number: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct SyncSignature {
                        pub signer_pub_key: runtime_types::primitive_types::U256,
                        pub r: runtime_types::primitive_types::U256,
                        pub s: runtime_types::primitive_types::U256,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum UniversalEvent {
                        #[codec(index = 0)]
                        MarketUpdated(
                            runtime_types::pallet_support::types::sync_facade::MarketUpdated,
                        ),
                        #[codec(index = 1)]
                        AssetUpdated(
                            runtime_types::pallet_support::types::sync_facade::AssetUpdated,
                        ),
                        #[codec(index = 2)]
                        MarketRemoved(
                            runtime_types::pallet_support::types::sync_facade::MarketRemoved,
                        ),
                        #[codec(index = 3)]
                        AssetRemoved(
                            runtime_types::pallet_support::types::sync_facade::AssetRemoved,
                        ),
                        #[codec(index = 4)]
                        UserDeposit(runtime_types::pallet_support::types::sync_facade::UserDeposit),
                        #[codec(index = 5)]
                        SignerAdded(runtime_types::pallet_support::types::sync_facade::SignerAdded),
                        #[codec(index = 6)]
                        SignerRemoved(
                            runtime_types::pallet_support::types::sync_facade::SignerRemoved,
                        ),
                        #[codec(index = 7)]
                        QuorumSet(runtime_types::pallet_support::types::sync_facade::QuorumSet),
                        #[codec(index = 8)]
                        SettingsAdded(
                            runtime_types::pallet_support::types::sync_facade::SettingsAdded,
                        ),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct UserDeposit { pub event_index : :: core :: primitive :: u32 , pub trading_account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal , pub collateral_id : :: core :: primitive :: u128 , pub nonce : runtime_types :: primitive_types :: U256 , pub amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , pub block_number : :: core :: primitive :: u64 , }
                }
                pub mod trading {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct AccountInfo {
                        pub positions: ::std::vec::Vec<
                            runtime_types::pallet_support::types::trading::PositionExtended,
                        >,
                        pub available_margin: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub total_margin: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub collateral_balance:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub force_closure_flag: ::core::option::Option<
                            runtime_types::pallet_support::types::trading::ForceClosureFlag,
                        >,
                        pub unused_balance: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Direction {
                        #[codec(index = 0)]
                        Long,
                        #[codec(index = 1)]
                        Short,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum ForceClosureFlag {
                        #[codec(index = 0)]
                        Deleverage,
                        #[codec(index = 1)]
                        Liquidate,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum FundModifyType {
                        #[codec(index = 0)]
                        Increase,
                        #[codec(index = 1)]
                        Decrease,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MarginInfo {
                        pub is_liquidation: ::core::primitive::bool,
                        pub total_margin: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub available_margin: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub unrealized_pnl_sum:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub maintenance_margin_requirement:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Order {
                        pub account_id: runtime_types::primitive_types::U256,
                        pub order_id: runtime_types::primitive_types::U256,
                        pub market_id: ::core::primitive::u128,
                        pub order_type: runtime_types::pallet_support::types::trading::OrderType,
                        pub direction: runtime_types::pallet_support::types::trading::Direction,
                        pub side: runtime_types::pallet_support::types::trading::Side,
                        pub price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub leverage: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub slippage: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub post_only: ::core::primitive::bool,
                        pub time_in_force:
                            runtime_types::pallet_support::types::trading::TimeInForce,
                        pub signature_info:
                            runtime_types::pallet_support::types::trading::SignatureInfo,
                        pub timestamp: ::core::primitive::u64,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum OrderSide {
                        #[codec(index = 0)]
                        Maker,
                        #[codec(index = 1)]
                        Taker,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum OrderType {
                        #[codec(index = 0)]
                        Limit,
                        #[codec(index = 1)]
                        Market,
                        #[codec(index = 2)]
                        Forced,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Position {
                        pub market_id: ::core::primitive::u128,
                        pub direction: runtime_types::pallet_support::types::trading::Direction,
                        pub avg_execution_price:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub margin_amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub borrowed_amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub leverage: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub created_timestamp: ::core::primitive::u64,
                        pub modified_timestamp: ::core::primitive::u64,
                        pub realized_pnl: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct PositionExtended {
                        pub market_id: runtime_types::primitive_types::U256,
                        pub direction: runtime_types::pallet_support::types::trading::Direction,
                        pub avg_execution_price:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub margin_amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub borrowed_amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub leverage: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub created_timestamp: ::core::primitive::u64,
                        pub modified_timestamp: ::core::primitive::u64,
                        pub realized_pnl: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub maintenance_margin:
                            runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub mark_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Side {
                        #[codec(index = 0)]
                        Buy,
                        #[codec(index = 1)]
                        Sell,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct SignatureInfo {
                        pub liquidator_pub_key: runtime_types::primitive_types::U256,
                        pub hash_type: runtime_types::pallet_support::types::common::HashType,
                        pub sig_r: runtime_types::primitive_types::U256,
                        pub sig_s: runtime_types::primitive_types::U256,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum TimeInForce {
                        #[codec(index = 0)]
                        GTC,
                        #[codec(index = 1)]
                        IOC,
                        #[codec(index = 2)]
                        FOK,
                    }
                }
                pub mod trading_account {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct BalanceUpdate {
                        pub asset_id: ::core::primitive::u128,
                        pub balance_value: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MonetaryAccountDetails {
                        pub monetary_account: runtime_types::primitive_types::U256,
                        pub trading_accounts: ::std::vec::Vec<runtime_types::primitive_types::U256>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct TradingAccount {
                        pub account_id: runtime_types::primitive_types::U256,
                        pub account_address: runtime_types::primitive_types::U256,
                        pub index: ::core::primitive::u8,
                        pub pub_key: runtime_types::primitive_types::U256,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct TradingAccountMinimal {
                        pub account_address: runtime_types::primitive_types::U256,
                        pub pub_key: runtime_types::primitive_types::U256,
                        pub index: ::core::primitive::u8,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct WithdrawalRequest {
                        pub account_id: runtime_types::primitive_types::U256,
                        pub collateral_id: ::core::primitive::u128,
                        pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub timestamp: ::core::primitive::u64,
                        pub sig_r: runtime_types::primitive_types::U256,
                        pub sig_s: runtime_types::primitive_types::U256,
                        pub hash_type: runtime_types::pallet_support::types::common::HashType,
                    }
                }
                pub mod trading_fees {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct BaseFee {
                        pub volume: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub fee: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct FeeRates {
                        pub maker_buy: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub maker_sell: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub taker_buy: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pub taker_sell: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                    }
                }
            }
        }
        pub mod pallet_sync_facade {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See `Pallet::add_signer`."]
                    add_signer {
                        pub_key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 1)]
                    #[doc = "See `Pallet::set_signers_quorum`."]
                    set_signers_quorum { new_quorum: ::core::primitive::u8 },
                    #[codec(index = 2)]
                    #[doc = "See `Pallet::remove_signer`."]
                    remove_signer {
                        pub_key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 3)]
                    #[doc = "See `Pallet::synchronize_events`."]
                    synchronize_events {
                        events_batch: ::std::vec::Vec<
                            runtime_types::pallet_support::types::sync_facade::UniversalEvent,
                        >,
                        signatures: ::std::vec::Vec<
                            runtime_types::pallet_support::types::sync_facade::SyncSignature,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Signer passed is 0"]
                    ZeroSigner,
                    #[codec(index = 1)]
                    #[doc = "Duplicate signer"]
                    DuplicateSigner,
                    #[codec(index = 2)]
                    #[doc = "No of signers less than required quorum"]
                    InsufficientSigners,
                    #[codec(index = 3)]
                    #[doc = "Signer not whitelisted"]
                    SignerNotWhitelisted,
                    #[codec(index = 4)]
                    #[doc = "No events provided"]
                    EmptyBatch,
                    #[codec(index = 5)]
                    #[doc = "Batch sent again"]
                    DuplicateBatch,
                    #[codec(index = 6)]
                    #[doc = "Old Batch sent"]
                    OldBatch,
                    #[codec(index = 7)]
                    #[doc = "Not enough signatures for a sync tx"]
                    InsufficientSignatures,
                    #[codec(index = 8)]
                    #[doc = "Invalid FieldElement value"]
                    ConversionError,
                    #[codec(index = 9)]
                    #[doc = "Invalid Call to dev mode only function"]
                    DevOnlyCall,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Signer added by the admin successfully"]
                    SignerAdded {
                        signer: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 1)]
                    #[doc = "Signer removed by the admin succesfully"]
                    SignerRemoved {
                        signer: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 2)]
                    #[doc = "New Quorum requirement set by the admin"]
                    QuorumSet { quorum: ::core::primitive::u8 },
                    #[codec(index = 3)]
                    #[doc = "A invalid request to remove non-existent market"]
                    MarketRemovedError { id: ::core::primitive::u128 },
                    #[codec(index = 4)]
                    #[doc = "An invalid request to remove non-existent asset"]
                    AssetRemovedError { id: ::core::primitive::u128 },
                    #[codec(index = 5)]
                    #[doc = "An invalid request to add a duplicate signer"]
                    SignerAddedError {
                        pub_key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 6)]
                    #[doc = "An invalid request to remove non-existent signer"]
                    SignerRemovedError {
                        pub_key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 7)]
                    #[doc = "An invalid request to remove signer; leads to insufficient signers"]
                    SignerRemovedQuorumError { quorum: ::core::primitive::u8 },
                    #[codec(index = 8)]
                    #[doc = "An invalid request to set a signer"]
                    QuorumSetError { quorum: ::core::primitive::u8 },
                    #[codec(index = 9)]
                    #[doc = "An invalid request for user deposit"]
                    UserDepositError {
                        collateral_id: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "An invalid key in settings"]
                    SettingsKeyError { key: ::core::primitive::u128 },
                    #[codec(index = 11)]
                    #[doc = "Insufficient data for setting fees"]
                    InsufficientFeeData { id: ::core::primitive::u128 },
                    #[codec(index = 12)]
                    #[doc = "Fee data length mismatch"]
                    FeeDataLengthMismatch { id: ::core::primitive::u128 },
                    #[codec(index = 13)]
                    #[doc = "Token parsing error"]
                    TokenParsingError {
                        key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 14)]
                    #[doc = "An invalid request to add an asset"]
                    AddAssetError { id: ::core::primitive::u128 },
                    #[codec(index = 15)]
                    #[doc = "An invalid request to update an asset"]
                    UpdateAssetError { id: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "An invalid requet to add a market"]
                    AddMarketError { id: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "An invalid request to update a market"]
                    UpdateMarketError { id: ::core::primitive::u128 },
                    #[codec(index = 18)]
                    #[doc = "An unknown asset/market id passed"]
                    UnknownIdForFees { id: ::core::primitive::u128 },
                    #[codec(index = 19)]
                    #[doc = "An invalid request to set max abr"]
                    InvalidMarket { id: ::core::primitive::u128 },
                    #[codec(index = 20)]
                    #[doc = "A max abr request with empty array"]
                    EmptyValuesError { id: ::core::primitive::u128 },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::set`]."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_trading {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See `Pallet::execute_trade`."]
                    execute_trade {
                        batch_id: runtime_types::primitive_types::U256,
                        quantity_locked: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        market_id: ::core::primitive::u128,
                        oracle_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        orders:
                            ::std::vec::Vec<runtime_types::pallet_support::types::trading::Order>,
                        batch_timestamp: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    #[doc = "See `Pallet::add_liquidator_signer`."]
                    add_liquidator_signer {
                        pub_key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 2)]
                    #[doc = "See `Pallet::remove_liquidator_signer`."]
                    remove_liquidator_signer {
                        pub_key: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 3)]
                    #[doc = "See `Pallet::cancel_order`."]
                    cancel_order {
                        order_id: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 4)]
                    #[doc = "See `Pallet::perform_cleanup`."]
                    perform_cleanup,
                    #[codec(index = 5)]
                    #[doc = "See `Pallet::set_matching_time_limit`."]
                    set_matching_time_limit { time_limit: ::core::primitive::u64 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Balance not enough to open the position"]
                    TradeBatchError501,
                    #[codec(index = 1)]
                    #[doc = "Invalid value for leverage (less than min or greater than currently allowed leverage)"]
                    TradeBatchError502,
                    #[codec(index = 2)]
                    #[doc = "Invalid quantity locked w.r.t step size"]
                    TradeBatchError503,
                    #[codec(index = 3)]
                    #[doc = "Market matched and order market are different"]
                    TradeBatchError504,
                    #[codec(index = 4)]
                    #[doc = "Order size less than min quantity"]
                    TradeBatchError505,
                    #[codec(index = 5)]
                    #[doc = "Price is not within slippage limit"]
                    TradeBatchError506,
                    #[codec(index = 6)]
                    #[doc = "Execution price is not valid wrt limit price for long sell or short buy"]
                    TradeBatchError507,
                    #[codec(index = 7)]
                    #[doc = "Execution price is not valid wrt limit price for long buy or short sell"]
                    TradeBatchError508,
                    #[codec(index = 8)]
                    #[doc = "Invalid market"]
                    TradeBatchError509,
                    #[codec(index = 9)]
                    #[doc = "User's account is not registered"]
                    TradeBatchError510,
                    #[codec(index = 10)]
                    #[doc = "Taker side or direction is invalid wrt to makers"]
                    TradeBatchError511,
                    #[codec(index = 11)]
                    #[doc = "Maker side or direction does not match with other makers"]
                    TradeBatchError512,
                    #[codec(index = 12)]
                    #[doc = "Invalid oracle price,"]
                    TradeBatchError513,
                    #[codec(index = 13)]
                    #[doc = "Taker order could not be executed since all makers do not satisfy slippage limit"]
                    TradeBatchError514,
                    #[codec(index = 14)]
                    #[doc = "Taker order is post only"]
                    TradeBatchError515,
                    #[codec(index = 15)]
                    #[doc = "FoK Orders should be filled completely"]
                    TradeBatchError516,
                    #[codec(index = 16)]
                    #[doc = "Order size is invalid w.r.t to step size"]
                    TradeBatchError517,
                    #[codec(index = 17)]
                    #[doc = "Maker order can only be limit order"]
                    TradeBatchError518,
                    #[codec(index = 18)]
                    #[doc = "Slippage must be between 0 and 15"]
                    TradeBatchError521,
                    #[codec(index = 19)]
                    #[doc = "Quantity locked must be > 0"]
                    TradeBatchError522,
                    #[codec(index = 20)]
                    #[doc = "Maker order skipped since quantity_executed = quantity_locked for the batch"]
                    TradeBatchError523,
                    #[codec(index = 21)]
                    #[doc = "Order is trying to close an empty position"]
                    TradeBatchError524,
                    #[codec(index = 22)]
                    #[doc = "Batch id already used"]
                    TradeBatchError525,
                    #[codec(index = 23)]
                    #[doc = "Position cannot be opened becuase of passive risk management"]
                    TradeBatchError531,
                    #[codec(index = 24)]
                    #[doc = "Not enough margin to cover losses - short limit sell or long limit sell"]
                    TradeBatchError532,
                    #[codec(index = 25)]
                    #[doc = "Order is fully executed"]
                    TradeBatchError533,
                    #[codec(index = 26)]
                    #[doc = "Invalid order hash - order could not be hashed into a Field Element"]
                    TradeBatchError534,
                    #[codec(index = 27)]
                    #[doc = "Invalid Signature Field Elements - sig_r and/or sig_s could not be converted into a"]
                    #[doc = "Signature"]
                    TradeBatchError535,
                    #[codec(index = 28)]
                    #[doc = "ECDSA Signature could not be verified"]
                    TradeBatchError536,
                    #[codec(index = 29)]
                    #[doc = "Invalid public key - publickey u256 could not be converted to Field Element"]
                    TradeBatchError538,
                    #[codec(index = 30)]
                    #[doc = "When force closure flag is Liquidate or Deleverage, order type can only be Forced"]
                    TradeBatchError539,
                    #[codec(index = 31)]
                    #[doc = "If taker is forced, force closure flag must be present"]
                    TradeBatchError540,
                    #[codec(index = 32)]
                    #[doc = "Order hash mismatch for a particular order id"]
                    TradeBatchError541,
                    #[codec(index = 33)]
                    #[doc = "When a non-whitelisted pub key is used in liquidation"]
                    TradeBatchError542,
                    #[codec(index = 34)]
                    #[doc = "When a cancelled order is sent for execution"]
                    TradeBatchError543,
                    #[codec(index = 35)]
                    #[doc = "Order is older than 4 weeks"]
                    TradeBatchError544,
                    #[codec(index = 36)]
                    #[doc = "Batch is older than expected"]
                    TradeBatchError545,
                    #[codec(index = 37)]
                    #[doc = "Trade Volume Calculation Error"]
                    TradeBatchError546,
                    #[codec(index = 38)]
                    #[doc = "Insufficient num of orders in the batch"]
                    TradeBatchError547,
                    #[codec(index = 39)]
                    TradeBatchError548,
                    #[codec(index = 40)]
                    #[doc = "When a zero signer is being added"]
                    ZeroSigner,
                    #[codec(index = 41)]
                    #[doc = "When a duplicate signer is being added"]
                    DuplicateSigner,
                    #[codec(index = 42)]
                    #[doc = "When the order is signed with a pub key that is not whitelisted"]
                    SignerNotWhitelisted,
                    #[codec(index = 43)]
                    #[doc = "When order id passed is zero"]
                    ZeroOrderId,
                    #[codec(index = 44)]
                    #[doc = "Start timestamp is not set"]
                    StartTimestampEmpty,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Trade batch executed successfully"]
                    TradeExecuted {
                        batch_id: runtime_types::primitive_types::U256,
                        market_id: ::core::primitive::u128,
                        size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        execution_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        direction: ::core::primitive::u8,
                        side: ::core::primitive::u8,
                    },
                    #[codec(index = 1)]
                    #[doc = "Trade batch failed since no makers got executed"]
                    TradeExecutionFailed {
                        batch_id: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 2)]
                    #[doc = "Order error"]
                    OrderError {
                        order_id: runtime_types::primitive_types::U256,
                        error_code: ::core::primitive::u16,
                        account_id: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 3)]
                    #[doc = "Order of a user executed successfully"]
                    OrderExecuted {
                        account_id: runtime_types::primitive_types::U256,
                        order_id: runtime_types::primitive_types::U256,
                        market_id: ::core::primitive::u128,
                        size: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        direction: ::core::primitive::u8,
                        side: ::core::primitive::u8,
                        order_type: ::core::primitive::u8,
                        execution_price: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        pnl: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        fee: runtime_types::sp_arithmetic::fixed_point::FixedI128,
                        is_final: ::core::primitive::bool,
                        is_maker: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    #[doc = "Force closure flag updation event"]
                    ForceClosureFlagsChanged {
                        account_id: runtime_types::primitive_types::U256,
                        collateral_id: ::core::primitive::u128,
                        force_closure_flag: ::core::primitive::u8,
                    },
                    #[codec(index = 5)]
                    #[doc = "Liquidator signer added"]
                    LiquidatorSignerAdded {
                        signer: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 6)]
                    #[doc = "Liquidator signer removed"]
                    LiquidatorSignerRemoved {
                        signer: runtime_types::primitive_types::U256,
                    },
                }
            }
        }
        pub mod pallet_trading_account {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "See `Pallet::update_monetary_to_trading_accounts`."] update_monetary_to_trading_accounts { monetary_accounts : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: MonetaryAccountDetails > , } , # [codec (index = 1)] # [doc = "See `Pallet::deposit`."] deposit { trading_account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal , collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , } , # [codec (index = 2)] # [doc = "See `Pallet::add_accounts`."] add_accounts { accounts : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal > , } , # [codec (index = 3)] # [doc = "See `Pallet::set_balances`."] set_balances { account_id : runtime_types :: primitive_types :: U256 , balances : :: std :: vec :: Vec < runtime_types :: pallet_support :: types :: trading_account :: BalanceUpdate > , } , # [codec (index = 4)] # [doc = "See `Pallet::set_standard_withdrawal_fee`."] set_standard_withdrawal_fee { withdrawal_fee : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , } , # [codec (index = 5)] # [doc = "See `Pallet::adjust_balances`."] adjust_balances { start_index : :: core :: primitive :: u128 , end_index : :: core :: primitive :: u128 , precision : :: core :: primitive :: u32 , } , # [codec (index = 6)] # [doc = "See `Pallet::withdraw`."] withdraw { withdrawal_request : runtime_types :: pallet_support :: types :: trading_account :: WithdrawalRequest , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account already exists"]
                    DuplicateAccount,
                    #[codec(index = 1)]
                    #[doc = "Duplicate Withdrawal Request"]
                    DuplicateWithdrawal,
                    #[codec(index = 2)]
                    #[doc = "Account does not exist"]
                    AccountDoesNotExist,
                    #[codec(index = 3)]
                    #[doc = "Asset not created"]
                    AssetNotFound,
                    #[codec(index = 4)]
                    #[doc = "Asset provided as collateral is not marked as collateral in the system"]
                    AssetNotCollateral,
                    #[codec(index = 5)]
                    #[doc = "Withdrawal amount is less than available balance"]
                    InsufficientBalance,
                    #[codec(index = 6)]
                    #[doc = "Invalid withdrawal request hash - withdrawal request could not be hashed into a Field"]
                    #[doc = "Element"]
                    InvalidWithdrawalRequestHash,
                    #[codec(index = 7)]
                    #[doc = "Invalid Signature Field Elements - sig_r and/or sig_s could not be converted into a"]
                    #[doc = "Signature"]
                    InvalidSignatureFelt,
                    #[codec(index = 8)]
                    #[doc = "ECDSA Signature could not be verified"]
                    InvalidSignature,
                    #[codec(index = 9)]
                    #[doc = "Public Key not found for account id"]
                    NoPublicKeyFound,
                    #[codec(index = 10)]
                    #[doc = "Invalid public key - publickey u256 could not be converted to Field Element"]
                    InvalidPublicKey,
                    #[codec(index = 11)]
                    #[doc = "Invalid standard withdrawal fee"]
                    InvalidWithdrawalFee,
                    #[codec(index = 12)]
                    #[doc = "Invalid arguments in the withdrawal request"]
                    InvalidWithdrawalRequest,
                    #[codec(index = 13)]
                    #[doc = "Deposit and Withdrawal are not allowed if deleveraging or liquidation is in progress"]
                    ForceClosureFlagSet,
                    #[codec(index = 14)]
                    #[doc = "Market does not exist"]
                    MarketDoesNotExist,
                    #[codec(index = 15)]
                    #[doc = "Invalid Call to dev mode only function"]
                    DevOnlyCall,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "Several accounts added"] AccountsAdded { length : :: core :: primitive :: u128 , } , # [codec (index = 1)] # [doc = "Balances for an account updated"] BalanceUpdated { account_id : runtime_types :: primitive_types :: U256 , account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal , collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , modify_type : :: core :: primitive :: u8 , reason : :: core :: primitive :: u8 , previous_balance : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , new_balance : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , block_number : :: core :: primitive :: u32 , } , # [codec (index = 2)] # [doc = "Event emitted for deferred deposits"] DeferredBalance { account_id : runtime_types :: primitive_types :: U256 , collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , } , # [codec (index = 3)] # [doc = "Event to be synced by L2, for pnl changes"] UserBalanceChange { trading_account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal , collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , modify_type : runtime_types :: pallet_support :: types :: trading :: FundModifyType , reason : :: core :: primitive :: u8 , block_number : :: core :: primitive :: u32 , } , # [codec (index = 4)] # [doc = "Event to be synced by L2, for withdrawal requests"] UserWithdrawal { trading_account : runtime_types :: pallet_support :: types :: trading_account :: TradingAccountMinimal , collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , block_number : :: core :: primitive :: u32 , } , # [codec (index = 5)] # [doc = "Account created"] AccountCreated { account_id : runtime_types :: primitive_types :: U256 , account_address : runtime_types :: primitive_types :: U256 , index : :: core :: primitive :: u8 , } , # [codec (index = 6)] # [doc = "Amount passed to transfer/transfer_from functions is negative"] AmountIsNegative { account_id : runtime_types :: primitive_types :: U256 , collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , reason : :: core :: primitive :: u8 , } , # [codec (index = 7)] # [doc = "Insurance fund updation event"] InsuranceFundChange { collateral_id : :: core :: primitive :: u128 , amount : runtime_types :: sp_arithmetic :: fixed_point :: FixedI128 , modify_type : runtime_types :: pallet_support :: types :: trading :: FundModifyType , block_number : :: core :: primitive :: u32 , } , }
            }
        }
        pub mod pallet_trading_fees {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See `Pallet::update_base_fees`."]
                    update_base_fees {
                        id: ::core::primitive::u128,
                        side: runtime_types::pallet_support::types::trading::Side,
                        order_side: runtime_types::pallet_support::types::trading::OrderSide,
                        fee_details: ::std::vec::Vec<
                            runtime_types::pallet_support::types::trading_fees::BaseFee,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid fee"]
                    InvalidFee,
                    #[codec(index = 1)]
                    #[doc = "Invalid number of tokens"]
                    InvalidVolume,
                    #[codec(index = 2)]
                    #[doc = "There should be atleast one fee tier"]
                    ZeroFeeTiers,
                    #[codec(index = 3)]
                    #[doc = "Asset does not exist"]
                    AssetNotFound,
                    #[codec(index = 4)]
                    #[doc = "Asset is not a collateral"]
                    AssetNotCollateral,
                    #[codec(index = 5)]
                    #[doc = "Market does not exist"]
                    MarketNotFound,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Base fees details updated"]
                    BaseFeesUpdated { fee_tiers: ::core::primitive::u8 },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FeeDetails<_0> {
                    pub inclusion_fee: ::core::option::Option<
                        runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                    >,
                    pub tip: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InclusionFee<_0> {
                    pub base_fee: _0,
                    pub len_fee: _0,
                    pub adjusted_weight_fee: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RuntimeDispatchInfo<_0, _1> {
                    pub weight: _1,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub partial_fee: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct U256(pub [::core::primitive::u64; 4usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FixedI128(pub ::core::primitive::i128);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SlotDuration(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueMetadata(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::std::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::utils::H256,
                        pub extrinsics_root: ::subxt::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub provides: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod substrate_validator_set {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::add_validator`]."]
                    add_validator {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::remove_validator`]."]
                    remove_validator {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Target (post-removal) validator count is below the minimum."]
                    TooLowValidatorCount,
                    #[codec(index = 1)]
                    #[doc = "Validator is already in the validator set."]
                    Duplicate,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New validator addition initiated. Effective in ~2 sessions."]
                    ValidatorAdditionInitiated(::subxt::utils::AccountId32),
                    #[codec(index = 1)]
                    #[doc = "Validator removal initiated. Effective in ~2 sessions."]
                    ValidatorRemovalInitiated(::subxt::utils::AccountId32),
                }
            }
        }
    }
}
