BIN_DIR=./bin

.PHONY: build-commander
build-commander:
	rustc server/src/main.rs --out-dir ./${BIN_DIR}/commander

.PHONY: build-agent
build-agent:
	rustc agent/src/main.rs --out-dir ./${BIN_DIR}/agent

.PHONY: build
build: build-commander build-agent

.PHONY: clean
clean:
	@rm -rf ${BIN_DIR}
