ENDPOINT ?= mainnet.eth.streamingfast.io:443

DSN ?= postgres://postgres:postgres@localhost/substreams?sslmode=disable
ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -t +10

.PHONY: sink_setup_db_out
sink_setup_db_out: build
	substreams-sink-sql setup "$(DSN)" --ignore-duplicate-table-errors ./substreams.yaml

.PHONY: sink_db_out
sink_db_out: build
	substreams-sink-sql run "$(DSN)" ./substreams.yaml --undo-buffer-size 0
