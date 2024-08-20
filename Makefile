export COPYFILE_DISABLE=1

.PHONY: local_validator deploy test

set-localnet:
	solana config set --url localhost

local_validator: set-localnet
	solana-test-validator \
	--ledger .anchor/test-ledger/ --reset

setup0:
	solana-install init 1.17.34

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

removeToken:
	anchor run removeToken

removeTokenAgain:
	anchor run removeTokenAgain
