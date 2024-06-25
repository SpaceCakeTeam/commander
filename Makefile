BIN_DIR=./bin

.PHONY: build-commander
build-commander:
	rustc commander/main.rs --out-dir ./${BIN_DIR}/commander

.PHONY: build-hoplite
build-hoplite:
	rustc hoplite/main.rs --out-dir ./${BIN_DIR}/hoplite

.PHONY: build
build: build-commander build-hoplite

.PHONY: clean
clean:
	@rm -rf ${BIN_DIR}

