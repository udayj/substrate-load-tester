
#![allow(missing_docs)]
use sp_core::H256;
use subxt::config::{DefaultExtrinsicParamsBuilder};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use subxt_signer::SecretUri;
use std::str::FromStr;
use substrate_load_tester::polkadot::polkadot;
use tokio::time::{sleep, Duration};
use primitive_types::U256;
use substrate_load_tester::support::*;
use std::time::{SystemTime, UNIX_EPOCH};
use substrate_load_tester::config::*;
use chrono::{Timelike, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = get_config()?;

    let start = SystemTime::now();
    let current_timestamp = start.duration_since(UNIX_EPOCH).unwrap();
    println!("Current timestamp - will be used for order & batch: {}",current_timestamp.as_millis());
    
    if config.setup_required {    
        setup(&config).await?;
    }
    
    // Provide funds to the different accounts which will be sending execute_trade txs in separate threads
    // Calibrate initial nonces for the different accounts
    let mut nonces = vec![];
    for i in 1..(config.tps as i32 + 1) {
        
        let api = OnlineClient::<PolkadotConfig>::from_insecure_url(config.url.as_str()).await.unwrap();
        let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
        println!("Providing funds to Account {}",i);
        let account_uri = format!("//Account{}",i);
        let temp_signer = Keypair::from_uri(&SecretUri::from_str(account_uri.as_str()).unwrap()).unwrap();
        let account = temp_signer.public_key().into();
        let temp_signer_id = temp_signer.public_key().into();
        let set_balance_tx = polkadot::Call::Balances(polkadot::balances::Call::force_set_balance { who: temp_signer_id, new_free: 2000000000000000});
        let sudo_tx = polkadot::tx().sudo().sudo(set_balance_tx);
        let _ = api.tx().sign_and_submit_then_watch_default(&sudo_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;
 
        let query = polkadot::storage().system().account(&account);
        let result = api
            .storage()
            .at_latest()
            .await?
            .fetch(&query)
            .await?;
        let nonce = result.unwrap().nonce;
        println!("Nonce for account {} : {}",i,nonce);
        nonces.push(nonce);
    }

    let start_time = Utc::now();
    println!("Start sending traffic at time UTC : {}:{}:{}", start_time.hour(), start_time.minute(), start_time.second());
    // Send execute_trade txs in separate threads from the different endowed accounts
    for j in 1..(config.duration as i32 +1){
        let mut threads = vec![];
        let thread_nonces = nonces.clone();
        for i in 1..(config.tps as i32 + 1) {
            let nonce = thread_nonces[i as usize - 1];
            let thread_url = config.url.clone();
            let thread = tokio::spawn(async move {
                let api = OnlineClient::<PolkadotConfig>::from_insecure_url(thread_url.as_str()).await.unwrap();
                let alice_id: U256 = get_trading_account_id(alice());
                let bob_id: U256 = get_trading_account_id(bob());
                let account_uri = format!("//Account{}",i);
                let temp_signer = Keypair::from_uri(&SecretUri::from_str(account_uri.as_str()).unwrap()).unwrap();
                
                let alice_order =
                        Order::new(U256::from(300000000_i32+(nonce as i32)*100000 + j*20+i), alice_id)
                        .set_timestamp(current_timestamp.as_millis() as u64)
                        .sign_order(get_private_key(U256(alice().pub_key.0)));
                let bob_order = Order::new(U256::from(500000000_i32+(nonce as i32)*100000+j*20+i), bob_id)
                        .set_direction(Direction::Short)
                        .set_order_type(OrderType::Market)
                        .set_timestamp(current_timestamp.as_millis() as u64)
                        .sign_order(get_private_key(U256(bob().pub_key.0)));

                let alice_order_polkadot = convert_to_polkadot_order(alice_order);
                let bob_order_polkadot = convert_to_polkadot_order(bob_order);
                let mut prices:Vec<polkadot::runtime_types::pallet_support::types::prices::MultiplePrices>=vec![];
                let index_price1 = polkadot::runtime_types::pallet_support::types::prices::MultiplePrices {
                    market_id: btc_usdc().market.id,
                    index_price: convert_to_fixed_i128(1.into()),
                    mark_price: convert_to_fixed_i128(1.into())
                };
                prices.push(index_price1);
                let prices_tx = polkadot::tx().prices().update_prices(prices, current_timestamp.as_millis() as u64 + 1);

                let _ = api.tx().sign_and_submit(&prices_tx, &temp_signer, DefaultExtrinsicParamsBuilder::new().nonce( nonce as u64 + (j-1) as u64).build())
                .await.unwrap_or(H256::zero());

                //println!("{:#?}",alice_order_polkadot);
                /*let execute_trade_tx = polkadot::tx().trading().execute_trade(
                    convert_to_u256(U256::from((nonce as i32)*100000+j*20+2+i)), 
                    convert_to_fixed_i128(1.into()), 
                    btc_usdc().market.id, 
                    convert_to_fixed_i128(1.into()), 
                    vec![alice_order_polkadot, bob_order_polkadot], 
                    current_timestamp.as_millis() as u64);
                let validation = api.tx().validate(&execute_trade_tx);
                assert_eq!(validation.is_ok(),true);
                println!("{}",format!("tx is valid: epoch cycle={} account={}",j, i).as_str());
                println!("nonce={:?}",nonce as u64 +(j-1) as u64);
    
                let _ = api.tx().sign_and_submit(&execute_trade_tx, &temp_signer, DefaultExtrinsicParamsBuilder::new().nonce( nonce as u64 + (j-1) as u64).build())
                .await.unwrap_or(H256::zero());*/
            });

            threads.push(thread);
        }

        for thread in threads.iter_mut() {
            thread.await.unwrap();
        }

        // This acts like a batch separator and we send approx tps number of transactions in a single slot time
        //sleep(Duration::from_millis(500)).await;
    }

    let stop_time = Utc::now();
    
    println!("Stop sending traffic at time UTC: {}:{}:{}", stop_time.hour(), stop_time.minute(), stop_time.second());
    println!("Total {} transactions sent in {} milli seconds", 
        (config.duration*config.tps as u32), (stop_time-start_time).num_milliseconds());

    Ok(())
}

pub async fn setup(config: &Config) -> Result<(), Box<dyn std::error::Error>>{

    let start = SystemTime::now();

    // Create a new API client, configured to talk to ZKX substrate nodes
    // This uses standard polkadot config

    let api = OnlineClient::<PolkadotConfig>::from_insecure_url(config.url.as_str()).await?;
    let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
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
    println!("Initialize trading accounts");
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
    
    /*println!("Sending a simple execute_trade");
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
        convert_to_fixed_i128(1.into()), 
        btc_usdc().market.id, 
        convert_to_fixed_i128(1.into()), 
        vec![alice_order_polkadot, bob_order_polkadot], 
        current_timestamp.as_millis() as u64);
    let validation = api.tx().validate(&execute_trade_tx);
    assert_eq!(validation.is_ok(),true);
    
    let _ = api.tx().sign_and_submit_then_watch_default(&execute_trade_tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await?;
    */
    Ok(())
}