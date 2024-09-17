BIN_DIR=./target
AGENT_IMAGE=commander-agent
COMMANDER_IMAGE=commander
KIND_CLUSTER_NAME=commander

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

.PHONY: kind
kind:
	kind create cluster --name ${KIND_CLUSTER_NAME}

.PHONY: docker-build-agent
docker-build-agent:
	docker build -t ${AGENT_IMAGE} . -f ./agent/Dockerfile

.PHONY: docker-build-commander
docker-build-commander:
	docker build -t ${COMMANDER_IMAGE} . -f ./commander/Dockerfile

.PHONY: test-deploy
test-deploy:
	kind load docker-image ${AGENT_IMAGE} --name ${KIND_CLUSTER_NAME}
	kind load docker-image ${COMMANDER_IMAGE} --name ${KIND_CLUSTER_NAME}
	kubectl apply -f ./e2e/manifest.yaml

.PHONY: test-locally
test-locally: docker-build-agent docker-build-commander test-deploy
