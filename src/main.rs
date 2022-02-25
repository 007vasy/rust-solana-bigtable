use {
    enum_iterator::IntoEnumIterator,
    std::io::{self, BufReader, Read, Write},
};

use serde::{Serialize, Deserialize};


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
    println!("{}", method_size)
    print(CompressionMethod::Zstd)
}
