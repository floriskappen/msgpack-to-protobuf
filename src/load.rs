
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

pub fn load_messagepack_data(data_filepath: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let input_file = File::open(data_filepath)?;
    let mut buf_reader = BufReader::new(input_file);
    let mut deserialized = Deserializer::new(&mut buf_reader);
    
    // Deserialize directly using MessagePack deserializer
    let data: Vec<Vec<u8>> = Deserialize::deserialize(&mut deserialized)?;

    return Ok(data);
}
