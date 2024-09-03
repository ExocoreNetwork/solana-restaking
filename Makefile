export COPYFILE_DISABLE=1

export ENDPOINT_ADDRESS = "76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6"
export SENDULN302_ADDRESS = "7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH"
export RECEIVEULN302_ADDRESS = "7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH"
export EXECUTOR_ADDRESS = "6doghB248px58JSSwG4qejQ46kFMW4AMj7vzJnWZHNZn"

export ENDPOINT_NAME = "Endpoint.so"
export SENDULN302_NAME = "Senduln302.so"
export RECEIVEULN302_NAME = "Receiveuln302.so"
export EXECUTOR_NAME = "Executor.so"

.PHONY: local_validator deploy test

set-mainnet:
	solana config set --url https://api.mainnet-beta.solana.com

set-devnet:
	solana config set --url https://api.devnet.solana.com

dump: dump-endpoint dump-senduln302 dump-receiveuln302 dump-executor

dump-endpoint: set-devnet
	solana program dump -u m ${ENDPOINT_ADDRESS} ${ENDPOINT_NAME}

dump-senduln302: set-devnet
	solana program dump -u m ${SENDULN302_ADDRESS} ${SENDULN302_NAME}

dump-receiveuln302: set-devnet
	solana program dump -u m ${RECEIVEULN302_ADDRESS} ${RECEIVEULN302_NAME}

dump-executor: set-devnet
	solana program dump -u m ${EXECUTOR_ADDRESS} ${EXECUTOR_NAME}

set-localnet:
	solana config set --url localhost

local_validator: set-localnet
	solana-test-validator \
	--bpf-program ${ENDPOINT_ADDRESS} ${ENDPOINT_NAME} \
	--bpf-program ${SENDULN302_ADDRESS} ${SENDULN302_NAME} \
	--bpf-program ${RECEIVEULN302_ADDRESS} ${RECEIVEULN302_NAME} \
	--bpf-program ${EXECUTOR_ADDRESS} ${EXECUTOR_NAME} \
	--ledger .anchor/test-ledger/ --reset

setup0:
	solana-install init 1.17.31

setup1:
	avm use 0.29.0

setup: setup0 setup1

build0: setup
	anchor build

deploy:
	anchor deploy

build:
	anchor build

initialize:
	anchor run initialize

addToken:
	anchor run addToken

addTokenAgain:
	anchor run addTokenAgain

deactivateToken:
	anchor run deactivateToken

deactivateTokenAgain:
	anchor run deactivateTokenAgain

transferOwnership:
	anchor run transferOwnership

transferOwnershipFailure:
	anchor run transferOwnershipFailure

accept:
	anchor run accept

acceptFailure:
	anchor run acceptFailure
