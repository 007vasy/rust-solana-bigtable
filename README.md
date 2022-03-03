# rust-solana-bigtable

## using it in python
there will be 1 constructor and 6 functions exposed [from here](https://github.com/solana-labs/solana/blob/master/storage-bigtable/src/lib.rs)

### 1 constructor to create a ledger connection (automate google creds?)
#### constructor

```python
bigtable_conn = <package>.new(timeout: int, credential_path: str)
```

### 5 functinos
#### get_first_available_block - Return the available slot that contains a block 
```python
resp:Dict[str,Any] = bigtable_conn.get_first_available_block()
```

#### get_confirmed_block - Fetch the confirmed block from the desired slot
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_block(slot:int)
```

#### get_confirmed_blocks (plural) - Fetch the next slots after the provided slot that contains a block
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_blocks(start_slot:int, limit: int)
```

#### get_signature_status - Get signature status
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_blocks(signature: str)
```

#### get_confirmed_transaction - Fetch a confirmed transaction
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_transaction(signature: str)
```

#### get_confirmed_signatures_for_address - Get confirmed signatures for the provided address, in descending ledger order
```python
resp:Dict[str,Any] = bigtable_conn.get_confirmed_signatures_for_address(address: str,before_signature: str|None, after_signature:str|None, limit: int|None)
```
