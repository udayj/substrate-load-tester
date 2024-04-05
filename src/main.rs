
#![allow(missing_docs)]
use sp_core::H256;
use subxt::config::{DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
//use substrate-load-tester::api;
use subxt_signer::sr25519::Keypair;
use subxt_signer::SecretUri;
use std::str::FromStr;
use substrate_load_tester::polkadot::polkadot;
use std::thread;
use tokio::time::{sleep, Duration};
use polkadot::runtime_types::pallet_support::types::asset::*;
use primitive_types::U256;
use sp_core::bounded_vec::BoundedVec;
use substrate_load_tester::support::*;
use std::time::{SystemTime, UNIX_EPOCH};
use substrate_load_tester::config::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = get_config()?;

    let start = SystemTime::now();
    let current_timestamp = start.duration_since(UNIX_EPOCH).unwrap();
    println!("Current timestamp - will be used for order & batch: {}",current_timestamp.as_millis());

    // Create a new API client, configured to talk to ZKX substrate nodes
    // This uses standard polkadot config
    let api = OnlineClient::<PolkadotConfig>::from_insecure_url(config.url.as_str()).await?;
    let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
    
    if config.setup_required {    
       
        let account = dev::alice().public_key().into();
        let storage_query = polkadot::storage().system().account(&account);

        // Use that query to `fetch` a result. This returns an `Option<_>`, which will be
        // `None` if no value exists at the given address. You can also use `fetch_default`
        // where applicable, which will return the default value if none exists.
        
        let result = api
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_query)
            .await?;

        println!("Alice has free balance: {}", result.unwrap().data.free);

        /*let query = polkadot::storage().prices().users_per_batch();

        let result = api
            .storage()
            .at_latest()
            .await?
            .fetch(&query)
            .await?;
        println!("No. of batches: {}", result.unwrap());
        let set_user_tx = polkadot::Call::Prices(polkadot::prices::Call::set_no_of_users_per_batch { new_no_of_users_per_batch: 800_u64 });

        //let set_user_tx = polkadot::tx().prices().set_no_of_users_per_batch(800_u64);
        let sudo_tx = polkadot::tx().sudo().sudo(set_user_tx);
        let transaction = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

        let query = polkadot::storage().prices().users_per_batch();

        let result = api
            .storage()
            .at_latest()
            .await?
            .fetch(&query)
            .await?;
        println!("No. of batches after update : {}", result.unwrap());
        */

        // Initialize assets
        println!("Initializing assets");
        let assets = vec![eth(), btc(), usdc()];
        let asset_tx = polkadot::tx().assets().replace_all_assets(assets);

        let _ = api.tx().sign_and_submit_then_watch_default(&asset_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

        // Initialize markets
        println!("Initialize markets");
        let markets = vec![btc_usdc()];
        let market_tx = polkadot::tx().markets().replace_all_markets(markets);
        
        let _ = api.tx().sign_and_submit_then_watch_default(&market_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

        // Initialize accounts which will trade
        println!("Initialize accounts");
        let accounts = vec![alice(),bob()];
        let account_tx = polkadot::tx().trading_account().add_accounts(accounts);

        let _ = api.tx().sign_and_submit_then_watch_default(&account_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

        // Set matching limit required for execute_trade to correctly check timestamps
        println!("Setting matching time limit");
        let matching_time_limit_tx = polkadot::Call::Trading(polkadot::trading::Call::set_matching_time_limit { time_limit: 2419200 });

        let sudo_tx = polkadot::tx().sudo().sudo(matching_time_limit_tx);
        let _ = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;
        
        // Set fees
        println!("Setting up fees");
        let (fee_details_maker, fee_details_taker) = setup_fee();

        let fee_tx = polkadot::Call::TradingFees(
            polkadot::trading_fees::Call::update_base_fees { 
                id: usdc().asset.id, 
                side: polkadot::runtime_types::pallet_support::types::trading::Side::Buy, 
                order_side: polkadot::runtime_types::pallet_support::types::trading::OrderSide::Maker, 
                fee_details: convert_to_polkadot_base_fee(fee_details_maker) }
        );

        let sudo_tx = polkadot::tx().sudo().sudo(fee_tx);
        let _ = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

        let fee_tx = polkadot::Call::TradingFees(
            polkadot::trading_fees::Call::update_base_fees { 
                id: usdc().asset.id, 
                side: polkadot::runtime_types::pallet_support::types::trading::Side::Buy, 
                order_side: polkadot::runtime_types::pallet_support::types::trading::OrderSide::Taker, 
                fee_details: convert_to_polkadot_base_fee(fee_details_taker) }
        );

        let sudo_tx = polkadot::tx().sudo().sudo(fee_tx);
        let _ = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;
        
        println!("Sending a simple execute_trade");
        let alice_id: U256 = get_trading_account_id(alice());
        let bob_id: U256 = get_trading_account_id(bob());

        let alice_order =
                Order::new(U256::from(201), alice_id)
                .set_timestamp(current_timestamp.as_millis() as u64)
                .sign_order(get_private_key(U256(alice().pub_key.0)));
        let bob_order = Order::new(U256::from(202), bob_id)
                .set_direction(Direction::Short)
                .set_order_type(OrderType::Market)
                .set_timestamp(current_timestamp.as_millis() as u64)
                .sign_order(get_private_key(U256(bob().pub_key.0)));

        let alice_order_polkadot = convert_to_polkadot_order(alice_order);
        let bob_order_polkadot = convert_to_polkadot_order(bob_order);
        
        //println!("{:#?}",alice_order_polkadot);
        let execute_trade_tx = polkadot::tx().trading().execute_trade(
            convert_to_u256(U256::from(1_u8)), 
            convertToFixedI128(1.into()), 
            btc_usdc().market.id, 
            convertToFixedI128(1.into()), 
            vec![alice_order_polkadot, bob_order_polkadot], 
            current_timestamp.as_millis() as u64);
        let validation = api.tx().validate(&execute_trade_tx);
        assert_eq!(validation.is_ok(),true);
        println!("Tx is valid");

      
        let _ = api.tx().sign_and_submit_then_watch_default(&execute_trade_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

        /*let temp_signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
        let temp_signer_id = temp_signer.public_key().into();
        let set_balance_tx = polkadot::Call::Balances(polkadot::balances::Call::force_set_balance { who: temp_signer_id, new_free: 2000000000000000});
            let sudo_tx = polkadot::tx().sudo().sudo(set_balance_tx);
            let transaction = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
            .await?
            .wait_for_finalized_success()
            .await?;
        */

        // Provide fund to the different accounts which will be sending execute_trade txs in separate threads
        for i in 1..(config.tps as i32 + 1) {
            println!("Providing funds to Account {}",i);
            let account_uri = format!("//Account{}",i);
            let temp_signer = Keypair::from_uri(&SecretUri::from_str(account_uri.as_str()).unwrap()).unwrap();
            let temp_signer_id = temp_signer.public_key().into();
            let set_balance_tx = polkadot::Call::Balances(polkadot::balances::Call::force_set_balance { who: temp_signer_id, new_free: 2000000000000000});
            let sudo_tx = polkadot::tx().sudo().sudo(set_balance_tx);
            let _ = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
            .await?
            .wait_for_finalized_success()
            .await?;
           
        }
    }
    
    // Send execute_trade txs in separate threads from the different endowed accounts
    for j in 1..(config.duration as i32 +1){
        let mut threads = vec![];
        
        for i in 1..(config.tps as i32 + 1) {
            let thread_url = config.url.clone();
            let thread = tokio::spawn(async move {
                let api = OnlineClient::<PolkadotConfig>::from_insecure_url(thread_url.as_str()).await.unwrap();
                let alice_id: U256 = get_trading_account_id(alice());
                let bob_id: U256 = get_trading_account_id(bob());
                let account_uri = format!("//Account{}",i);
                let temp_signer = Keypair::from_uri(&SecretUri::from_str(account_uri.as_str()).unwrap()).unwrap();
                
                let alice_order =
                        Order::new(U256::from(300000000_i32+(config.nonce as i32)*100000 + j*20+i), alice_id)
                        .set_timestamp(current_timestamp.as_millis() as u64)
                        .sign_order(get_private_key(U256(alice().pub_key.0)));
                let bob_order = Order::new(U256::from(500000000_i32+(config.nonce as i32)*100000+j*20+i), bob_id)
                        .set_direction(Direction::Short)
                        .set_order_type(OrderType::Market)
                        .set_timestamp(current_timestamp.as_millis() as u64)
                        .sign_order(get_private_key(U256(bob().pub_key.0)));

                let alice_order_polkadot = convert_to_polkadot_order(alice_order);
                let bob_order_polkadot = convert_to_polkadot_order(bob_order);
                
                //println!("{:#?}",alice_order_polkadot);
                let execute_trade_tx = polkadot::tx().trading().execute_trade(
                    convert_to_u256(U256::from((config.nonce as i32)*100000+j*20+2+i)), 
                    convertToFixedI128(1.into()), 
                    btc_usdc().market.id, 
                    convertToFixedI128(1.into()), 
                    vec![alice_order_polkadot, bob_order_polkadot], 
                    current_timestamp.as_millis() as u64);
                let validation = api.tx().validate(&execute_trade_tx);
                assert_eq!(validation.is_ok(),true);
                println!("{}",format!("tx is valid: epoch cycle={} account={}",j, i).as_str());
                println!("nonce={:?}",config.nonce as u64 +(j-1) as u64);
    
                let _ = api.tx().sign_and_submit(&execute_trade_tx, &temp_signer, DefaultExtrinsicParamsBuilder::new().nonce(config.nonce as u64 + (j-1) as u64).build())
                .await.unwrap_or(H256::zero());
            });

            threads.push(thread);
        }

        for thread in threads.iter_mut() {
            thread.await.unwrap();
        }
        sleep(Duration::from_millis(500)).await;
    }
    
    Ok(())
}