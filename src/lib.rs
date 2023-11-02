mod pb;

use std::collections::HashMap;

use pb::acme::call::v1::{BlockHeader, TransactionTrace, Call, StorageChange, BalanceChange, TransactionReceipt, Log};
use prost::Message;
use substreams::Hex;
use substreams_ethereum::pb::eth::{self};
use substreams_ethereum::pb::eth::v2 as substream_lib;

fn get_storage_changes(keccak_preimages: HashMap<String, String>, storage_changes: Vec<substream_lib::StorageChange>) -> Vec<StorageChange> {
    let mut changes: Vec<StorageChange> = Vec::new();
    for change in storage_changes.into_iter() {
        let key_as_hex = Hex::encode(&change.key);
        let result = keccak_preimages.get(&key_as_hex);

        let mut preimg = "".to_string();
        if result.is_some() {
            preimg = result.unwrap().clone();
        }

        changes.push(StorageChange { 
            // address: change.address,
            // key: change.key,
            preimg: preimg.encode_to_vec(),
            // old_value: change.old_value,
            new_value: change.new_value,
        });
    }

    changes
}

fn get_balance_change(raw_balance_changes: Vec<substream_lib::BalanceChange>) -> Vec<BalanceChange> {
    let mut balance_changes = Vec::new();

    for change in raw_balance_changes.into_iter() {
        balance_changes.push(BalanceChange { 
            // address: change.address, 
            // old_value: change.old_value.map(|val| val.bytes), 
            new_value: change.new_value.map(|val| val.bytes), 
            // reason: change.reason,
        })
    }

    balance_changes
}

fn get_calls(raw_calls: Vec<substream_lib::Call>) -> Vec<Call> {
    let mut calls: Vec<Call> = Vec::new();

    for call in raw_calls.into_iter() {
        calls.push(Call { 
            // index: call.index, 
            // parent_index: call.parent_index, 
            // call_type: call.call_type, 
            // caller: call.caller, 
            address: call.address, 
            // value: call.value.map(|val| val.bytes), 
            // gas_limit: call.gas_limit, 
            // gas_consumed: call.gas_consumed, 
            // return_data: call.return_data, 
            // input: call.input, 
            // exectued_code: call.executed_code, 
            // suicide: call.suicide, 
            storage_changes: get_storage_changes(call.keccak_preimages, call.storage_changes), 
            // balance_changes: get_balance_change(call.balance_changes) 
        })
    }

    calls
}

fn get_logs(raw_logs: Vec<substream_lib::Log>) -> Vec<Log> {
    let mut logs: Vec<Log> = Vec::new();
    for log in raw_logs.into_iter() {
        logs.push(Log { 
            // address: log.address, 
            topics: log.topics, 
            data: log.data, 
            // index: log.index, 
            // block_index: log.block_index
        })
    }

    logs
}

fn get_transactions_receipt(raw_receipt: Option<substream_lib::TransactionReceipt>) -> Option<TransactionReceipt> {
    if raw_receipt.is_none() {
        return None;
    }

    let receipt = raw_receipt.unwrap();
    Some(TransactionReceipt {
        // state_root: receipt.state_root,
        // cumulative_gas_used: receipt.cumulative_gas_used,
        // logs_bloom: receipt.logs_bloom,
        logs: get_logs(receipt.logs)
    }) 
}

fn get_transaction_traces(raw_transactions: Vec<substream_lib::TransactionTrace>) -> Vec<TransactionTrace> {
    let mut traces: Vec<TransactionTrace> = Vec::new();
    for tx in raw_transactions.into_iter() {
        traces.push(TransactionTrace { 
            to: tx.to,
            // nonce: tx.nonce,
            // gas_price: tx.gas_price.map(|val| val.bytes),
            // gas_limit: tx.gas_limit,
            // input: tx.input,
            // gas_used: tx.gas_used,
            // r#type: tx.r#type,
            // max_fee_per_gas: tx.max_fee_per_gas.map(|val| val.bytes),
            // max_priority_fee_per_gas: tx.max_priority_fee_per_gas.map(|val| val.bytes),
            // index: tx.index,
            status: tx.status,
            hash: tx.hash,
            from: tx.from,
            return_data: tx.return_data,
            receipt: get_transactions_receipt(tx.receipt),
            calls: get_calls(tx.calls),
        })
    }

    traces
}

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<BlockHeader, substreams::errors::Error> {
    let timestamp = block.timestamp_seconds();
    let block_header = block.header.unwrap();
    Ok(BlockHeader {
        parent_hash: block_header.parent_hash,
        // uncle_hash: block_header.uncle_hash,
        // coinbase: block_header.coinbase,
        // state_root: block_header.state_root,
        // transaction_root: block_header.transactions_root,
        logs_bloom: block_header.logs_bloom,
        // difficulty: block_header.difficulty.map(|val| val.bytes),
        // total_difficulty: block_header.total_difficulty.map(|val| val.bytes),
        // number: block_header.number,
        // gas_limit: block_header.gas_limit,
        // gas_used: block_header.gas_used,
        timestamp,
        // extra_data: block_header.extra_data,
        // mix_hash: block_header.mix_hash,
        // nonce: block_header.nonce,
        hash: block_header.hash,
        // base_fee_per_gas: block_header.base_fee_per_gas.map(|val| val.bytes),
        // withdrawals_root: block_header.withdrawals_root,
        transactions: get_transaction_traces(block.transaction_traces),
    })
}
