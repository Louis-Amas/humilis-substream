mod pb;

use substreams_database_change::{pb::database::{DatabaseChanges, table_change::Operation}, change::AsString};
use substreams_ethereum::{block_view::LogView, pb::eth::{self, v2::BlockHeader}};

use abis::contracts::EVENTS_SELECTOR_TO_ABI;

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

fn push_create(
    changes: &mut DatabaseChanges,
    header: BlockHeader,
) {
    let hash = header.hash;
    let timestamp = header.timestamp.unwrap();
    changes
        .push_change("block_header", hash.clone().as_string(), 1, Operation::Create)
        .change("hash", (None, hash))
        .change("parent_hash", (None, header.parent_hash))
        .change("logs_bloom", (None, header.logs_bloom))
        .change("timestamp", (None, timestamp))
        .change("number", (None, header.number));
}

#[substreams::handlers::map]
fn db_out(
    block: eth::v2::Block,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut changes: DatabaseChanges = Default::default();

    push_create(&mut changes, block.header.unwrap());

    // handle_logs(&mut changes, block.logs().collect());
    Ok(changes)
}
