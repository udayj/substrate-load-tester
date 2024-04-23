use sp_core::{OpaquePeerId};
use sp_core::ecdsa::{Pair as KeyPair};
use sp_core::sr25519::{Pair as SrKeyPair};
use sp_io::crypto::{ecdsa_verify, secp256k1_ecdsa_recover, secp256k1_ecdsa_recover_compressed, sr25519_generate, sr25519_verify};
use secp256k1::{
    Secp256k1,
	ecdsa::{RecoverableSignature, RecoveryId},
	Message, PublicKey, SecretKey, Signature
};
use sp_core_hashing::blake2_256;
use hex;
use schnorrkel::{signing_context, verify_batch_deterministic};
use schnorrkel::keys::{PublicKey as Sr25519_PublicKey, SecretKey as Sr25519_SecretKey, SECRET_KEY_LENGTH};

fn main() {
    let x = OpaquePeerId(bs58::decode("12D3KooWNRuMucN4HdBQuSpf9nokkTaMyoQAj9hQHdMPB6HZMWwq").into_vec().unwrap());
    println!("{:#?}",x);
}

fn utility() {

    let alice_id: U256 = get_trading_account_id(alice());
    let mut bytes:[u8;32] = [0_u8;32];
    alice_id.to_little_endian(&mut bytes);
    println!("account id: {:?}",hex::encode(bytes));
    let alice_order =
                        Order::new(U256::from(1), alice_id)
                        .set_timestamp(1 as u64)
                        .sign_order(get_private_key(U256(alice().pub_key.0)));
    let order_hash = alice_order.hash(&HashType::Poseidon).unwrap();
    println!("order:{:?}", alice_order);
    println!("order hash:{:?}", order_hash);
    let order_bytes = alice_order.to_bytes();
    let order_fixed:[u8;157] = order_bytes.try_into().unwrap();
    let secp = Secp256k1::new();
    let secret_key_ecdsa = SecretKey::from_slice(&[1;32]).unwrap();
    let message = Message::from_slice(&blake2_256(&order_fixed)).unwrap();
    let signature = secret_key_ecdsa.sign_ecdsa(message);
    let public_key = PublicKey::from_secret_key(&secp, &secret_key_ecdsa);
    let signature_2 = secp.sign_ecdsa_recoverable(&message, &secret_key_ecdsa);
    let (recovery_id, sig_bytes) = signature_2.serialize_compact();
    let mut signature_65 = [0_u8;65];
    signature_65[64] = recovery_id.to_i32() as u8;
    signature_65[..64].copy_from_slice(&sig_bytes);
    println!("secret key: {:?}  public key:{:?}", secret_key_ecdsa, public_key);
    println!("signature: {:?}", signature);
    println!("message: {:?}", message);
    println!("signature 2: {:?}", signature_2);
    println!("signature 65 bytes:{:?}",signature_65);
    let res = signature.verify(&message,&public_key).is_ok();
    println!("signature verification: {:?}", res);
    println!("public key string: {:?} len:{}",public_key.to_string(), public_key.to_string().len());
    println!("signature string: {:?} len:{}",signature.to_string(), signature.to_string().len());
    let recovered_key = secp256k1_ecdsa_recover_compressed(&signature_65, &blake2_256(&order_fixed)).unwrap_or([0_u8;33]);
    println!("recovered key: {:?}", recovered_key);
    println!("recovered key in hex: {:?}", hex::encode(recovered_key));
    println!("message bytes: {:?}", &blake2_256(&order_fixed));
    /*println!("{}", order_fixed.len());
    println!("{:?}", order_fixed);
    let key_pair:KeyPair = KeyPair::from_string("//Alice", None).unwrap();
    let key_pair_2:KeyPair = KeyPair::from_string("//Bob", None).unwrap();
    println!("{:?}",key_pair.public());
    let signature = key_pair.sign(&order_fixed);
    println!("{:?}",signature);
    let res = ecdsa_verify(&signature, &order_fixed, &key_pair.public());
    println!("{:?}", res);
    let res = ecdsa_verify(&signature, &order_fixed, &key_pair_2.public());
    println!("{:?}", res);*/

    //let sr25519_key:SrKeyPair = SrKeyPair::from_seed_slice(&[1_u8]).unwrap();
    //let sr25519_key_2:SrKeyPair = SrKeyPair::from_seed_slice(&[2_u8]).unwrap();
    let sr25519_key = sp_core::sr25519::Pair::generate();
    let sr25519_key_2 = sp_core::sr25519::Pair::generate();
    println!("{:?}",sr25519_key.0.public());
    let signature = sr25519_key.0.sign(&order_fixed);
    println!("{:?}",signature);
    let res = sr25519_verify(&signature, &order_fixed, &sr25519_key.0.public());
    println!("{:?}", res);
    let res = sr25519_verify(&signature, &order_fixed, &sr25519_key_2.0.public());
    println!("{:?}", res);

    const SIGNING_CTX: &[u8] = b"substrate";
    let context = signing_context(SIGNING_CTX);
    let message = blake2_256(&order_fixed);
    println!("message for sr25519: {:?}", message);
    let secret_key = Sr25519_SecretKey::from_bytes(&[1_u8;SECRET_KEY_LENGTH]).unwrap();
    let public_key = secret_key.to_public();
    let signature = secret_key.sign(context.bytes(&message), &public_key);
    println!("Sr25519 signature in bytes: {:?}", signature.to_bytes());
    println!("Sr25519 Public key bytes:{:?}", public_key.to_bytes());
    let res = public_key.verify_simple(SIGNING_CTX, &message, &signature);
    println!("Sr25519 signature verification:{:?}", res.is_ok());

    let alice_order =
                        Order::new(U256::from(1), alice_id)
                        .set_timestamp(2 as u64)
                        .sign_order(get_private_key(U256(alice().pub_key.0)));
    let order_bytes = alice_order.to_bytes();
    let order_fixed:[u8;157] = order_bytes.try_into().unwrap();
    let context = signing_context(SIGNING_CTX);
    let message = blake2_256(&order_fixed);
    println!("message for sr25519: {:?}", message);
    let secret_key = Sr25519_SecretKey::from_bytes(&[2_u8;SECRET_KEY_LENGTH]).unwrap();
    let public_key = secret_key.to_public();
    let signature = secret_key.sign(context.bytes(&message), &public_key);

    println!("Sr25519 signature in bytes: {:?}", signature.to_bytes());
    println!("Sr25519 Public key bytes:{:?}", public_key.to_bytes());
    let res = public_key.verify_simple(SIGNING_CTX, &message, &signature);
    println!("Sr25519 signature verification:{:?}", res.is_ok());

    let alice_order =
                        Order::new(U256::from(1), alice_id)
                        .set_timestamp(3 as u64)
                        .sign_order(get_private_key(U256(alice().pub_key.0)));
    let secret_key = Sr25519_SecretKey::from_bytes(&[2_u8;SECRET_KEY_LENGTH]).unwrap();
    let order_bytes = alice_order.to_bytes();
    let order_fixed:[u8;157] = order_bytes.try_into().unwrap();
    let public_key_2 = secret_key.to_public();
    let message_2 = blake2_256(&order_fixed);
    let signature_2 = secret_key.sign(context.bytes(&message_2), &public_key);
    let transcripts = [context.bytes(&message), context.bytes(&message_2)];
    let res = verify_batch_deterministic(transcripts, &[signature, signature_2], &[public_key, public_key_2], false).is_ok();
    println!("batch verification: {:?}", res);

}
