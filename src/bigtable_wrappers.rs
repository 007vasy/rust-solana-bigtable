use {
    enum_iterator::IntoEnumIterator,
    std::io::{self, BufReader, Read, Write},
    serde::{Serialize, Deserialize},
    serde::ser::{SerializeStruct,Serializer},
    solana_storage_bigtable::{LedgerStorage},
    futures::executor::block_on,
    std::fs::{self,File},
    std::str::FromStr,
    std::time::Duration,
    solana_sdk::{
        clock::{Slot, UnixTimestamp},
        pubkey::{self,Pubkey},
        signature::Signature,
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

    //get_confirmed_transaction
    println!(">> get_signature_status example START");
    let signature = Signature::from_str("2jucFJFnV7CyXH9X1esPfiou8hBqotNtQUUdfRAoASqtnYv6PnRKQF2deKZd4wmh99HtqBbNmChcp3KSV9EvwtzS").unwrap();
    let resp = connection.get_signature_status(&signature).await.unwrap();
    println!("{:?}",resp);
    let j = serde_json::to_string(&resp);

    println!("{:?}", j);
    println!(">> get_signature_status example END");
    
    // //get_confirmed_block
    // println!(">> get_confirmed_block example START");
    // let slot = 100010499;
    // let block_data = connection.get_confirmed_block(slot).await;
    // //let data_json_string = serde_json::to_string(&block_data.unwrap()).unwrap();
    // println!("{:?}", block_data);
    // println!(">> get_confirmed_block example END");

    // //get_confirmed_signatures_for_address
    // println!(">> get_confirmed_signatures_for_address example START");
    // let address = "57X5Rq3o7k5z976kAjYTWu5yKfgX1aQxH4bXACpmTPPF";
    // let pubkey = Pubkey::from_str(address).unwrap();
    // println!("{:?}",connection.get_confirmed_signatures_for_address(&pubkey,std::option::Option::None,std::option::Option::None,10).await);
    // println!(">> get_confirmed_signatures_for_address example END");

    // //get_confirmed_transaction
    // println!(">> get_confirmed_transaction example START");
    // let signature = Signature::from_str("2jucFJFnV7CyXH9X1esPfiou8hBqotNtQUUdfRAoASqtnYv6PnRKQF2deKZd4wmh99HtqBbNmChcp3KSV9EvwtzS").unwrap();
    // println!("{:?}",connection.get_confirmed_transaction(&signature).await);
    // println!(">> get_confirmed_transaction example END");
}

pub fn wrapper() {
    let rt = runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

    rt.block_on(do_fetch())
}

