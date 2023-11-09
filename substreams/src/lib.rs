mod pb;

mod contracts;

use substreams_ethereum::block_view::LogView;

use pb::acme::call::v1::BlockHeader;
use substreams_ethereum::pb::eth::{self};

use crate::contracts::contracts::EVENTS_SELECTOR_TO_ABI;

fn handle_logs(logs: Vec<LogView>) -> String {

    let mut transfer_count = 0;
    let mut log_count = 0;
    for log_receipt in logs.into_iter() {
        log_count += 1;
        if let Some(topic0) = log_receipt.log.topics.get(0) {
            if let Some(items) = EVENTS_SELECTOR_TO_ABI.get(topic0.as_slice()) {
                for item in items {
                    if item.signature.contains("Transfer") {
                        transfer_count += 1;
                    }
                }
            }
        } else {
            continue;
        }
        
    }

    format!("{} {} transfers", log_count, transfer_count)
}

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<BlockHeader, substreams::errors::Error> {
    Ok(BlockHeader {
        hash: block.hash.clone(),
        test: handle_logs(block.logs().collect()),//"test".to_string()// handle_logs(block.logs().collect())
    })
}
