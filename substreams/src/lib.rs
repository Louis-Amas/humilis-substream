mod pb;

use alloy_primitives::{U256, Address};
use substreams_database_change::{
    change::AsString,
    pb::database::{table_change::Operation, DatabaseChanges},
};
use substreams_ethereum::{
    pb::eth::{self, v2::{BlockHeader, TransactionTrace}},
};


// fn handle_logs(changes: &mut DatabaseChanges,  logs: Vec<LogView>) -> String {
//     let mut transfer_count = 0;
//     let mut log_count = 0;
//     for log_receipt in logs.into_iter() {
//         log_count += 1;
//         if let Some(topic0) = log_receipt.log.topics.get(0) {
//             if let Some(items) = EVENTS_SELECTOR_TO_ABI.get(topic0.as_slice()) {
//                 for item in items {
//                     if item.signature.contains("Transfer") {
//                         transfer_count += 1;
//                     }
//                 }
//             }
//         } else {
//             continue;
//         }
//
//     }
//
//     format!("{} {} transfers", log_count, transfer_count)
// }

fn handle_contract_storage(changes: &mut DatabaseChanges, block_number: u64, transaction_traces: Vec<TransactionTrace>) {
    for tx in transaction_traces {
        for call in tx.calls {
            for change in call.storage_changes {
                let primary_key = format!("{}-{:#?}", block_number, change.address);
                let pk = format!("{}-{}", primary_key, change.ordinal);

                let address = Address::try_from(change.address.as_slice()).unwrap();
                let slot = U256::try_from_be_slice(change.key.as_slice()).unwrap();

                let old_value = U256::try_from_be_slice(change.old_value.as_slice()).unwrap();
                let new_value = U256::try_from_be_slice(change.new_value.as_slice()).unwrap();

                changes.push_change(
                    "contract_storage",
                    pk.clone(),
                    change.ordinal, 
                    Operation::Create
                )
                 .change("id", (None, pk))
                 .change("address", (None, address.to_string()))
                 .change("slot", (None, slot.to_string()))
                 .change("old_value", (None, old_value.to_string()))
                 .change("new_value", (None, new_value.to_string()))
                 .change("block_number", (None, block_number));
            }
        }
    }
}

fn create_header(changes: &mut DatabaseChanges, header: BlockHeader) {
    let hash = header.hash;
    let timestamp = header.timestamp.unwrap();
    changes
        .push_change(
            "block_header",
            hash.clone().as_string(),
            1,
            Operation::Create,
        )
        .change("hash", (None, hash))
        .change("parent_hash", (None, header.parent_hash))
        .change("logs_bloom", (None, header.logs_bloom))
        .change("timestamp", (None, timestamp))
        .change("number", (None, header.number));
}

#[substreams::handlers::map]
fn db_out(block: eth::v2::Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut changes: DatabaseChanges = Default::default();

    let number = block.number;
    create_header(&mut changes, block.header.unwrap());

    handle_contract_storage(&mut changes, number, block.transaction_traces);

    // handle_logs(&mut changes, block.logs().collect());
    Ok(changes)
}
