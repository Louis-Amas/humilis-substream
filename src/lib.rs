mod pb;

use pb::acme::call::v1::{BlockCall, Call as MyCall, StorageChange};
use substreams::Hex;
use substreams_ethereum::pb::eth::{self};

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<BlockCall, substreams::errors::Error> {
    let mut calls: Vec<MyCall> = Vec::new();

    for tx in block.transactions() {
        for call in tx.calls.to_vec() {
            let mut changes: Vec<StorageChange> = Vec::new();
            for change in call.storage_changes {
                let key_as_hex = Hex::encode(change.key);
                let result = call.keccak_preimages.get(&key_as_hex);

                let mut preimg = "".to_string();
                if result.is_some() {
                    preimg = result.unwrap().clone();
                }

                changes.push(StorageChange { 
                    address: Hex::encode(change.address),
                    key: key_as_hex, 
                    preimg, 
                    old_value: Hex::encode(change.old_value), 
                    new_value: Hex::encode(change.new_value), 
                    ordinal: change.ordinal,
                });
            }
            calls.push(MyCall {
                address: Hex::encode(call.address),
                storage_changes: changes,
            });
        }
    }

    Ok(BlockCall {
        calls,
    })
}
