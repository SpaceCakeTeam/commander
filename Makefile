BIN_DIR=./target

.PHONY: build-commander
build-commander:
	cargo build --package commander

.PHONY: build-agent
build-agent:
	cargo build --package agent

.PHONY: build
build: build-commander build-agent

.PHONY: test-messages
test-messages:
	cargo test --package messages

.PHONY: test-commander
test-commander:
	cargo test --package commander

.PHONY: test-agent
test-agent:
	cargo test --package agent

.PHONY: test
test: test-commander test-agent test-messages

.PHONY: clean
clean:
	@rm -rf ${BIN_DIR}
