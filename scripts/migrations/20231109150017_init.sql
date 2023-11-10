CREATE TABLE block_header (
    hash BYTEA PRIMARY KEY,
    parent_hash BYTEA,
    logs_bloom BYTEA,
    timestamp timestamp,
    number INT
);

-- CREATE TABLE interface (
--   name TEXT PRIMARY KEY
-- );
--
-- CREATE TABLE event_type (
--   signature TEXT PRIMARY KEY
-- );
--
-- CREATE TABLE mapping_event_interface (
--   name TEXT,
--   signature TEXT,
--   FOREIGN KEY (name) REFERENCES interface(name),
--   FOREIGN KEY (signature) REFERENCES event_type(signature),
--
--   PRIMARY KEY (name, signature)
-- );
--
-- CREATE TABLE contract (
--   address ethereum_address PRIMARY KEY,
--   createdAt time
-- );
--
-- CREATE TABLE log (
--   block_hash ethereum_hash,
--   address ethereum_address,
--
--   index INTEGER,
--
--   topic0 BYTEA,
--   decoded_topic0 TEXT,
--
--   topic1 BYTEA,
--   decoded_topic1 TEXT,
--
--   topic2 BYTEA,
--   decoded_topic2 TEXT,
--
--   topic3 BYTEA,
--   decoded_topic3 TEXT,
--
--   topic4 BYTEA,
--   decoded_topic4 TEXT,
--
--   data BYTEA,
--   decoded_data TEXT,
--
--   FOREIGN KEY (block_hash) REFERENCES block_header(hash),
--   FOREIGN KEY (address) REFERENCES contract(address),
--
--   signature TEXT,
--   FOREIGN kEY (signature) REFERENCES event_type(signature),
--
--   PRIMARY KEY(address, index)
-- );

CREATE TABLE cursors
(
    id         text not null constraint cursor_pk PRIMARY KEY,
    cursor     text,
    block_num  bigint,
    block_id   text
);
