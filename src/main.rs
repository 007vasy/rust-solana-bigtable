
use {
    enum_iterator::IntoEnumIterator,
    std::io::{self, BufReader, Read, Write},
    serde::{Serialize, Deserialize},
    solana_storage_bigtable::{LedgerStorage},
    futures::executor::block_on,
    std::fs::{self,File},
    std::time::Duration
};



fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

#[derive(Debug, Serialize, Deserialize, IntoEnumIterator)]
pub enum CompressionMethod {
    NoCompression,
    Bzip2,
    Gzip,
    Zstd,
}

async fn do_fetch() {
    let sixty_sec = Duration::new(60, 0);
    let connection = LedgerStorage::new(true, std::option::Option::Some(sixty_sec), std::option::Option::Some("/home/ben/Projects/ChainLinkLabs/bigtable-decode/solana-sandbox-86de2dfd579b.json".to_owned()));
}

fn main() {
    println!("Hello, world!");
    let method_size = bincode::serialized_size(&CompressionMethod::NoCompression).unwrap();
    println!("Method size: {}", method_size);
    let zstd = CompressionMethod::Zstd;
    println!("Zstd enum: {:?}", zstd as u8);

    let filename = "100010499.bin";
    let cell_data = get_file_as_byte_vec(&filename.to_owned());


    let do_fetch = do_fetch();
    block_on(do_fetch)
    //println!("Cell data: {:?}", bigtable::CellData::Protobuf(cell_data));
    //get_confirmed_block
}
