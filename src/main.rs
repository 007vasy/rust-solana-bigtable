
use {
    enum_iterator::IntoEnumIterator,
    std::io::{self, BufReader, Read, Write},
    serde::{Serialize, Deserialize},
    serde::ser::{SerializeStruct,Serializer},
    solana_storage_bigtable::{LedgerStorage},
    futures::executor::block_on,
    std::fs::{self,File},
    std::time::Duration,
    solana_sdk::{
        clock::{Slot, UnixTimestamp},
        pubkey::Pubkey,
    },
    tokio::runtime
};

// #[derive(Clone, Debug, PartialEq)]
// pub struct ConfirmedBlockWithOptionalMetadata {
//     pub previous_blockhash: String,
//     pub blockhash: String,
//     pub parent_slot: Slot,
//     pub transactions: Vec<TransactionWithOptionalMetadata>,
//     pub rewards: Rewards,
//     pub block_time: Option<UnixTimestamp>,
//     pub block_height: Option<u64>,
// }

// https://serde.rs/impl-serialize.html

// impl Serialize for ConfirmedBlockWithOptionalMetadata {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // l7 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("ConfirmedBlockWithOptionalMetadata", 7)?;
//         state.serialize_field("previous_blockhash", &self.previous_blockhash)?;
//         state.serialize_field("blockhash", &self.blockhash)?;
//         state.serialize_field("parent_slot", &self.parent_slot)?;
//         state.serialize_field("transactions", &self.transactions)?;
//         state.serialize_field("rewards", &self.rewards)?;
//         state.serialize_field("block_time", &self.block_time)?;
//         state.serialize_field("block_height", &self.block_height)?;
//         state.end()
//     }
// }

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
    let connection = LedgerStorage::new(true, std::option::Option::Some(sixty_sec), std::option::Option::Some("./solana-sandbox-86de2dfd579b.json".to_owned())).await.unwrap();
    // let slot = 100010499;
    // let block_data = connection.get_confirmed_block(slot).await;
    // //let data_json_string = serde_json::to_string(&block_data.unwrap()).unwrap();
    // println!("{:?}", block_data);

    //get_confirmed_signatures_for_address
    let address = b"57X5Rq3o7k5z976kAjYTWu5yKfgX1aQxH4bXACpmTPPF";
    let pubkey = Pubkey::new(address);
    println!("_@_");
    println!("{:?}",connection.get_confirmed_signatures_for_address(&pubkey,std::option::Option::None,std::option::Option::None,10).await);
    
    //get_confirmed_transaction
}

fn main() {
    // println!("Hello, world!");
    // let method_size = bincode::serialized_size(&CompressionMethod::NoCompression).unwrap();
    // println!("Method size: {}", method_size);
    // let zstd = CompressionMethod::Zstd;
    // println!("Zstd enum: {:?}", zstd as u8);

    // let filename = "100010499.bin";
    // let cell_data = get_file_as_byte_vec(&filename.to_owned());

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(do_fetch())
    //println!("Cell data: {:?}", bigtable::CellData::Protobuf(cell_data));
    //get_confirmed_block
}
