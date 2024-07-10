BIN_DIR=./target

.PHONY: build-commander
build-commander:
	cargo build --package commander

.PHONY: build-agent
build-agent:
	cargo build --package agent

.PHONY: build
build: build-commander build-agent

.PHONY: clean
clean:
	@rm -rf ${BIN_DIR}
