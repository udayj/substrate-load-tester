use sp_core::{OpaquePeerId};

fn main() {
    let x = OpaquePeerId(bs58::decode("12D3KooWNRuMucN4HdBQuSpf9nokkTaMyoQAj9hQHdMPB6HZMWwq").into_vec().unwrap());
    println!("{:#?}",x);
}