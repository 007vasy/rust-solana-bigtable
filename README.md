# rust-solana-bigtable

## using it in python
there will be 1 constructor and 6 functions exposed [from here](https://github.com/solana-labs/solana/blob/master/storage-bigtable/src/lib.rs)

### 1 constructor to create a ledger connection (automate google creds?)
#### constructor

```python
bigtable_conn = <package>.new(timeout: int, credential_path: str)
```

### 6 functinos
#### get_first_available_block - Return the available slot that contains a block 
```python
resp:int = bigtable_conn.get_first_available_block()
```
expected struct (in Rust for the time being)
```rust
// Result<Option<Slot>>
pub type Slot = u64;
```

#### get_confirmed_blocks (plural) - Fetch the next slots after the provided slot that contains a block
```python
resp:List[int] = bigtable_conn.get_confirmed_blocks(start_slot:int, limit: int)
```
expected struct (in Rust for the time being)
```rust
// Result<Vec<Slot>>
pub type Slot = u64;
```

#### get_confirmed_block - Fetch the confirmed block from the desired slot
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_block(slot:int)
```
expected struct (in Rust for the time being)
```rust
// Result<ConfirmedBlock>
#[derive(Clone, Debug, PartialEq)]
pub struct ConfirmedBlock {
    pub previous_blockhash: String,
    pub blockhash: String,
    pub parent_slot: Slot,
    pub transactions: Vec<TransactionWithStatusMeta>,
    pub rewards: Rewards,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

pub type UnixTimestamp = i64;

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionWithStatusMeta {
    // Very old transactions may be missing metadata
    MissingMetadata(Transaction),
    // Versioned stored transaction always have metadata
    Complete(VersionedTransactionWithStatusMeta),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VersionedTransactionWithStatusMeta {
    pub transaction: VersionedTransaction,
    pub meta: TransactionStatusMeta,
}

// https://github.com/solana-labs/solana/blob/f804ccdece06c3a5bdf7e04e342d68fc4128ce91/storage-proto/proto/confirmed_block.proto
message TransactionStatusMeta {
    TransactionError err = 1;
    uint64 fee = 2;
    repeated uint64 pre_balances = 3;
    repeated uint64 post_balances = 4;
    repeated InnerInstructions inner_instructions = 5;
    bool inner_instructions_none = 10;
    repeated string log_messages = 6;
    bool log_messages_none = 11;
    repeated TokenBalance pre_token_balances = 7;
    repeated TokenBalance post_token_balances = 8;
    repeated Reward rewards = 9;
    repeated bytes loaded_writable_addresses = 12;
    repeated bytes loaded_readonly_addresses = 13;
}

#[wasm_bindgen]
#[frozen_abi(digest = "FZtncnS1Xk8ghHfKiXE5oGiUbw2wJhmfXQuNgQR3K6Mc")]
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize, AbiExample)]
pub struct Transaction {
    /// A set of signatures of a serialized [`Message`], signed by the first
    /// keys of the `Message`'s [`account_keys`], where the number of signatures
    /// is equal to [`num_required_signatures`] of the `Message`'s
    /// [`MessageHeader`].
    ///
    /// [`account_keys`]: Message::account_keys
    /// [`MessageHeader`]: crate::message::MessageHeader
    /// [`num_required_signatures`]: crate::message::MessageHeader::num_required_signatures
    // NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    #[wasm_bindgen(skip)]
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,

    /// The message to sign.
    #[wasm_bindgen(skip)]
    pub message: Message,
}

#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize, AbiExample)]
pub struct VersionedTransaction {
    /// List of signatures
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,
    /// Message to sign.
    pub message: VersionedMessage,
}
```

#### get_signature_status - Get signature status
```python
resp:Dict[str,Any] = bigtable_conn.get_signature_status(signature: str)
```

expected struct (in Rust for the time being)
```rust
// Result<TransactionStatus>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatus {
    pub slot: Slot,
    pub confirmations: Option<usize>, // None = rooted
    pub status: Result<()>,           // legacy field
    pub err: Option<TransactionError>,
    pub confirmation_status: Option<TransactionConfirmationStatus>,
}

#[derive(
    Error, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, AbiExample, AbiEnumVisitor,
)]
pub enum TransactionError {
// redacted see full: https://github.com/solana-labs/solana/blob/7dbde2247d271511905a4b29df4ea631ee690c1d/sdk/src/transaction/error.rs#L13
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionConfirmationStatus {
    Processed,
    Confirmed,
    Finalized,
}

```

#### get_confirmed_transaction - Fetch a confirmed transaction
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_transaction(signature: str)
```

expected struct (in Rust for the time being)
```rust
// Result<Option<ConfirmedTransactionWithStatusMeta>>

#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmedTransactionWithStatusMeta {
    pub slot: Slot,
    pub tx_with_meta: TransactionWithStatusMeta,
    pub block_time: Option<UnixTimestamp>,
}

pub type UnixTimestamp = i64;

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionWithStatusMeta {
    // Very old transactions may be missing metadata
    MissingMetadata(Transaction),
    // Versioned stored transaction always have metadata
    Complete(VersionedTransactionWithStatusMeta),
}

#[wasm_bindgen]
#[frozen_abi(digest = "FZtncnS1Xk8ghHfKiXE5oGiUbw2wJhmfXQuNgQR3K6Mc")]
#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize, AbiExample)]
pub struct Transaction {
    /// A set of signatures of a serialized [`Message`], signed by the first
    /// keys of the `Message`'s [`account_keys`], where the number of signatures
    /// is equal to [`num_required_signatures`] of the `Message`'s
    /// [`MessageHeader`].
    ///
    /// [`account_keys`]: Message::account_keys
    /// [`MessageHeader`]: crate::message::MessageHeader
    /// [`num_required_signatures`]: crate::message::MessageHeader::num_required_signatures
    // NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    #[wasm_bindgen(skip)]
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,

    /// The message to sign.
    #[wasm_bindgen(skip)]
    pub message: Message,
}


#[derive(Clone, Debug, PartialEq)]
pub struct VersionedTransactionWithStatusMeta {
    pub transaction: VersionedTransaction,
    pub meta: TransactionStatusMeta,
}

```

#### get_confirmed_signatures_for_address - Get confirmed signatures for the provided address, in descending ledger order
```python
resp:List[Dict[str,Any]] = bigtable_conn.get_confirmed_signatures_for_address(address: str,before_signature: str|None, after_signature:str|None, limit: int|None)
```

expected struct (in Rust for the time being)
```rust
// Result<
//     Vec<(
//         ConfirmedTransactionStatusWithSignature,
//         u32, /*slot index*/
//     )>,
// >

#[derive(Clone, Debug, PartialEq)]
pub struct ConfirmedTransactionStatusWithSignature {
    pub signature: Signature,
    pub slot: Slot,
    pub err: Option<TransactionError>,
    pub memo: Option<String>,
    pub block_time: Option<UnixTimestamp>,
}

pub type Slot = u64;

pub enum TransactionError {
// redacted see full: https://github.com/solana-labs/solana/blob/7dbde2247d271511905a4b29df4ea631ee690c1d/sdk/src/transaction/error.rs#L13
}

pub type UnixTimestamp = i64;

pub struct Signature(GenericArray<u8, U64>);
```


## dev

1. clone the repo at this tag https://github.com/007vasy/rust-solana-bigtable/releases/tag/v0.2
1. install rust https://www.rust-lang.org/tools/install
1. build it (cargo build)
1. get service account api key with bigtable read access
1. update path in code
1. cargo run
1. get decoded info
1. PROFIT