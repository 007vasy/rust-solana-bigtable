use {
    enum_iterator::IntoEnumIterator,
    std::io::{self, BufReader, Read, Write},
    serde::{Serialize, Deserialize},
    solana_storage_bigtable::LedgerStorage
};

#[derive(Debug, Serialize, Deserialize, IntoEnumIterator)]
pub enum CompressionMethod {
    NoCompression,
    Bzip2,
    Gzip,
    Zstd,
}

fn main() {
    println!("Hello, world!");
    let method_size = bincode::serialized_size(&CompressionMethod::NoCompression).unwrap();
    println!("Method size: {}", method_size);
    let zstd = CompressionMethod::Zstd;
    println!("Zstd enum: {:?}", zstd as u8);
    let connection = LedgerStorage::new(true, Option("60"), ).await;
}
