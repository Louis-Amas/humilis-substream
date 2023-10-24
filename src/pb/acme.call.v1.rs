// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageChange {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub preimg: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub old_value: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub new_value: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Call {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub storage_changes: ::prost::alloc::vec::Vec<StorageChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockCall {
    #[prost(message, repeated, tag="1")]
    pub calls: ::prost::alloc::vec::Vec<Call>,
}
// @@protoc_insertion_point(module)
