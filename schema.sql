CREATE TYPE ethereum_address As (VARCHAR(40));
CREATE TYPE ethereum_hash As (VARCHAR(66));


CREATE TABLE block_header (
    parent_hash ethereum_hash,
    logs_bloom BYTEA,
    timestamp BIGINT,
    hash ethereum_hash,
    number INT
);

CREATE TABLE contract {
  address ethereum_address,
  created_at TIMESTAMP 
};

CREATE TABLE log {
  block_hash ethereum_hash,
  address ethereum_address,

  topic0 BYTEA,
  decoded_topic0 STRING,

  topic1 BYTEA,
  decoded_topic1 STRING,

  topic2 BYTEA,
  decoded_topic2 STRING,

  topic3 BYTEA,
  decoded_topic3 STRING,

  data BYTEA,
  decoded_data STRING,

  FOREIGN KEY (block_hash) REFENRENCES (block_header.hash),
  FOREIGN KEY (address) REFENRENCES (contract.address)
};

