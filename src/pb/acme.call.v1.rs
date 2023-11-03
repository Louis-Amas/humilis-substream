// @generated
/// bytes address = 1;
/// bytes key = 2;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageChange {
    #[prost(bytes="vec", tag="3")]
    pub preimg: ::prost::alloc::vec::Vec<u8>,
    /// bytes old_value = 4;
    #[prost(bytes="vec", tag="5")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// optional bytes old_value = 2;
    #[prost(bytes="vec", optional, tag="3")]
    pub new_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Call {
    /// uint32 index = 1;
    /// uint32 parent_index = 2;
    #[prost(enumeration="CallType", tag="3")]
    pub call_type: i32,
    #[prost(bytes="vec", tag="4")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// optional bytes value = 6;
    /// uint64 gas_limit = 7;
    /// uint64 gas_consumed = 8;
    ///
    #[prost(bytes="vec", tag="9")]
    pub return_data: ::prost::alloc::vec::Vec<u8>,
    ///
    /// bool exectued_code = 11;
    /// bool suicide = 12;
    #[prost(bytes="vec", tag="10")]
    pub input: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="13")]
    pub storage_changes: ::prost::alloc::vec::Vec<StorageChange>,
    #[prost(message, repeated, tag="14")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    /// bytes address = 1;
    #[prost(bytes="vec", repeated, tag="2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// uint32 index = 4;
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// bytes state_root = 1;
/// uint64 cumulative_gas_used = 2;
/// bytes logs_bloom = 3;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionReceipt {
    #[prost(message, repeated, tag="4")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTrace {
    #[prost(bytes="vec", tag="1")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    /// uint64 nonce = 2;
    /// optional bytes gas_price = 3;
    /// uint64 gas_limit = 4;
    /// bytes input = 5;
    /// uint64 gas_used = 6;
    /// int32 type = 7;
    ///
    /// optional bytes max_fee_per_gas = 8;
    /// optional bytes max_priority_fee_per_gas = 9;
    ///
    /// uint32 index = 10;
    #[prost(enumeration="TransactionTraceStatus", tag="11")]
    pub status: i32,
    #[prost(bytes="vec", tag="12")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub return_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="15")]
    pub receipt: ::core::option::Option<TransactionReceipt>,
    #[prost(message, repeated, tag="16")]
    pub calls: ::prost::alloc::vec::Vec<Call>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(bytes="vec", tag="1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    /// bytes uncle_hash = 2;
    /// bytes coinbase = 3;
    /// bytes state_root = 4;
    /// bytes transaction_root = 5;
    #[prost(bytes="vec", tag="6")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    // optional bytes difficulty = 7;
    // optional bytes total_difficulty = 8;

    // uint64 number = 9;
    // uint64 gas_limit = 10;
    // uint64 gas_used = 11;

    #[prost(uint64, tag="12")]
    pub timestamp: u64,
    /// bytes extra_data = 13;
    /// bytes mix_hash = 14;
    /// uint64 nonce = 15;
    #[prost(bytes="vec", tag="16")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    // optional bytes base_fee_per_gas = 17;
    // bytes withdrawals_root = 18;

    #[prost(message, repeated, tag="19")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionTrace>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceChangeReason {
    Unknown = 0,
    RewardMineUncle = 1,
    RewardMineBlock = 2,
    DaoRefundContract = 3,
    DaoAdjustBalance = 4,
    Transfer = 5,
    GenesisBalance = 6,
    GasBuy = 7,
    RewardTransactionFee = 8,
    RewardFeeReset = 14,
    GasRefund = 9,
    TouchAccount = 10,
    SuicideRefund = 11,
    SuicideWithdraw = 13,
    CallBalanceOverride = 12,
    /// / Used on chain(s) where some Ether burning happens
    Burn = 15,
    Withdrawal = 16,
}
impl BalanceChangeReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BalanceChangeReason::Unknown => "BalanceChangeReasonUnknown",
            BalanceChangeReason::RewardMineUncle => "RewardMineUncle",
            BalanceChangeReason::RewardMineBlock => "RewardMineBlock",
            BalanceChangeReason::DaoRefundContract => "DaoRefundContract",
            BalanceChangeReason::DaoAdjustBalance => "DaoAdjustBalance",
            BalanceChangeReason::Transfer => "Transfer",
            BalanceChangeReason::GenesisBalance => "GenesisBalance",
            BalanceChangeReason::GasBuy => "GasBuy",
            BalanceChangeReason::RewardTransactionFee => "RewardTransactionFee",
            BalanceChangeReason::RewardFeeReset => "RewardFeeReset",
            BalanceChangeReason::GasRefund => "GasRefund",
            BalanceChangeReason::TouchAccount => "TouchAccount",
            BalanceChangeReason::SuicideRefund => "SuicideRefund",
            BalanceChangeReason::SuicideWithdraw => "SuicideWithdraw",
            BalanceChangeReason::CallBalanceOverride => "CallBalanceOverride",
            BalanceChangeReason::Burn => "Burn",
            BalanceChangeReason::Withdrawal => "Withdrawal",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BalanceChangeReasonUnknown" => Some(Self::Unknown),
            "RewardMineUncle" => Some(Self::RewardMineUncle),
            "RewardMineBlock" => Some(Self::RewardMineBlock),
            "DaoRefundContract" => Some(Self::DaoRefundContract),
            "DaoAdjustBalance" => Some(Self::DaoAdjustBalance),
            "Transfer" => Some(Self::Transfer),
            "GenesisBalance" => Some(Self::GenesisBalance),
            "GasBuy" => Some(Self::GasBuy),
            "RewardTransactionFee" => Some(Self::RewardTransactionFee),
            "RewardFeeReset" => Some(Self::RewardFeeReset),
            "GasRefund" => Some(Self::GasRefund),
            "TouchAccount" => Some(Self::TouchAccount),
            "SuicideRefund" => Some(Self::SuicideRefund),
            "SuicideWithdraw" => Some(Self::SuicideWithdraw),
            "CallBalanceOverride" => Some(Self::CallBalanceOverride),
            "Burn" => Some(Self::Burn),
            "Withdrawal" => Some(Self::Withdrawal),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CallType {
    Unspecified = 0,
    /// / direct? what's the name for `Call` alone?
    NormalCall = 1,
    Callcode = 2,
    Delegate = 3,
    Static = 4,
    /// / create2 ? any other form of calls?
    Create = 5,
}
impl CallType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CallType::Unspecified => "Unspecified",
            CallType::NormalCall => "NormalCall",
            CallType::Callcode => "Callcode",
            CallType::Delegate => "Delegate",
            CallType::Static => "Static",
            CallType::Create => "Create",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "NormalCall" => Some(Self::NormalCall),
            "Callcode" => Some(Self::Callcode),
            "Delegate" => Some(Self::Delegate),
            "Static" => Some(Self::Static),
            "Create" => Some(Self::Create),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionTraceStatus {
    Unknown = 0,
    Succeeded = 1,
    Failed = 2,
    Reverted = 3,
}
impl TransactionTraceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionTraceStatus::Unknown => "Unknown",
            TransactionTraceStatus::Succeeded => "Succeeded",
            TransactionTraceStatus::Failed => "Failed",
            TransactionTraceStatus::Reverted => "Reverted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Succeeded" => Some(Self::Succeeded),
            "Failed" => Some(Self::Failed),
            "Reverted" => Some(Self::Reverted),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
