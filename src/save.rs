use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use prost::Message;
use crate::proto::Data;

pub fn save_protobuf_data(data: Vec<Vec<u8>>, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = Data {
        data,
    };

    let mut buf = Vec::new();
    data.encode(&mut buf)?;

    let mut file = BufWriter::new(File::create(filepath)?);
    file.write_all(&buf)?;
    Ok(())
}

pub fn load_protobuf_data(filepath: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut file = BufReader::new(File::open(filepath)?);
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let data = Data::decode(&*buf)?;
    Ok(data.data)
}
