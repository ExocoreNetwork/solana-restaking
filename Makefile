export COPYFILE_DISABLE=1

export ENDPOINT_ADDRESS = "76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6"
export ULN302_ADDRESS = "7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH"
export EXECUTOR_ADDRESS = "6doghB248px58JSSwG4qejQ46kFMW4AMj7vzJnWZHNZn"
export PRICE_FEED_ADDRESS = "8ahPGPjEbpgGaZx2NV1iG5Shj7TDwvsjkEDcGWjt94TP"
export DVN_ADDRESS = "HtEYV4xB4wvsj5fgTkcfuChYpvGYzgzwvNhgDZQNh7wW"

export DEFAULT_MESSAGE_LIB = "DnW7HubBKvsS79dcsYTZg1G8jBAtDS9pkzBPpTruRpoP"
export SEND_LIBRARY_INFO = "526PeNZfw8kSnDU4nmzJFVJzJWNhwmZykEyJr5XWz5Fv"
export ENDPOINT_SETTINGS = "2uk9pQh3tB5ErV7LGQJcbWjb4KeJ2UJki5qJZ8QG56G3"
export ULN_SETTINGS = "2XgGZG4oP29U3w5h4nTk1V2LFHL23zKDPJjs3psGzLKQ"
export EXECUTOR_CONFIG = "AwrbHeCyniXaQhiJZkLhgWdUCteeWSGaSN1sTfLiY7xK"
export PRICE_FEED_CONFIG = "CSFsUupvJEQQd1F4SsXGACJaxQX4eropQMkGV2696eeQ"
export DVN_CONFIG = "4VDjp6XQaxoZf5RGwiPU9NR1EXSZn2TP4ATMmiSzLfhb"

export DEFAULT_SEND_LIBRARY_CONFIG = "gU5rYi3eVPqFJzUgths7ZHxmbfQh2j3KQX3Mg7vR6sr"
export DEFAULT_SEND_CONFIG = "5ro8ELnyfCmD8UnBgJ1yuZ3qgQ16WzaxTqgZDBRJmFBb"

export LST_RESTAKING_PROGRAM_ID = "3DsgkXpd7Hwc6Q1iZ4YGLFrfSQZvotGSDGYRAvcDL53V"
export LST_RESTAKING_PROGRAM_PATH = "target/deploy/lst_restaking.so"

export ENDPOINT_NAME = "Endpoint.so"
export ULN302_NAME = "uln302.so"
export EXECUTOR_NAME = "Executor.so"
export PRICE_FEED_NAME = "PriceFeed.so"
export DVN_NAME = "dvn.so"

.PHONY: local_validator deploy test

set-mainnet:
	solana config set --url https://api.mainnet-beta.solana.com

set-devnet:
	solana config set --url https://api.devnet.solana.com

set-localnet:
	solana config set --url localhost

dump: set-devnet dump-endpoint dump-uln302 dump-executor dump-price_feed dump-dvn

dump-endpoint:
	solana program dump -u m ${ENDPOINT_ADDRESS} ${ENDPOINT_NAME}

dump-uln302:
	solana program dump -u m ${ULN302_ADDRESS} ${ULN302_NAME}

dump-executor:
	solana program dump -u m ${EXECUTOR_ADDRESS} ${EXECUTOR_NAME}

dump-price_feed:
	solana program dump -u m ${PRICE_FEED_ADDRESS} ${PRICE_FEED_NAME}

dump-dvn:
	solana program dump -u m ${DVN_ADDRESS} ${DVN_NAME}


local_validator: set-localnet
	solana-test-validator \
	--bpf-program ${ENDPOINT_ADDRESS} ${ENDPOINT_NAME} \
	--bpf-program ${ULN302_ADDRESS} ${ULN302_NAME} \
	--bpf-program ${EXECUTOR_ADDRESS} ${EXECUTOR_NAME} \
	--bpf-program ${PRICE_FEED_ADDRESS} ${PRICE_FEED_NAME} \
	--bpf-program ${DVN_ADDRESS} ${DVN_NAME} \
	--clone ${DEFAULT_MESSAGE_LIB} \
	--clone ${ENDPOINT_SETTINGS} \
	--clone ${SEND_LIBRARY_INFO} \
	--clone ${ULN_SETTINGS} \
	--clone ${EXECUTOR_CONFIG} \
	--clone ${PRICE_FEED_CONFIG} \
	--clone ${DVN_CONFIG} \
	--clone ${DEFAULT_SEND_LIBRARY_CONFIG} \
	--clone ${DEFAULT_SEND_CONFIG} \
	--url "https://api.devnet.solana.com" \
	--ledger .anchor/test-ledger/ --reset

setup0:
	solana-install init 1.17.31

setup1:
	avm use 0.29.0

setup: setup0 setup1

build0: setup
	anchor build

deploy-lst:
	anchor deploy --program-name lst-restaking --provider.wallet .keys/dev.json # --provider.cluster "https://devnet.helius-rpc.com/?api-key=975e42bc-9bb2-4aa3-9fd5-7a8a10719df6"

upgrade-lst:
	anchor upgrade --program-id ${LST_RESTAKING_PROGRAM_ID} ${LST_RESTAKING_PROGRAM_PATH} --provider.wallet .keys/dev.json

upgrade-lst2:
	solana program deploy --buffer ${LST_RESTAKING_BUFFER} --program-id ${LST_RESTAKING_PROGRAM_ID} ${LST_RESTAKING_PROGRAM_PATH} --upgrade-authority .keys/dev.json


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

deposit:
	anchor run deposit

withdrawPrincipal:
	anchor run withdrawPrincipal

withdrawReward:
	anchor run withdrawReward

delegateTo:
	anchor run delegateTo

depositThenDelegateTo:
	anchor run depositThenDelegateTo


integrate: deploy initialize addToken deposit
