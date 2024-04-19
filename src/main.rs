
use dotenv::dotenv;
use load::load_messagepack_data;

use crate::save::{load_protobuf_data, save_protobuf_data};

mod proto {
    include!("proto/build/_.rs");
}
mod load;
mod save;

fn main() {
    dotenv().ok();
    let load_filepath = std::env::var("DATA_PATH").expect("No DATA_PATH env var found");
    let data = load_messagepack_data(&load_filepath).expect("Failed to load in data");

    println!("data: {:?}", data[..10].to_vec());

    save_protobuf_data(data, "./exports/export.bin").expect("Failed export");
}
