build:
	cd program; cargo build-bpf
	cd client; npm install

localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 10 localnet/altruist.json -u localhost
	solana airdrop 10 localnet/fund.json -u localhost
	solana airdrop 10 localnet/program.json -u localhost

deploy:
	solana program deploy program/target/deploy/program.so -u localhost --program-id localnet/program.json

testnet-deploy:
	cd program 
	cargo build-bpf
	cd ../
	solana program deploy program/target/deploy/program.so -u testnet --program-id localnet/program.json --keypair localnet/altruist.json --upgrade-authority localnet/altruist.json

client-cli:
	cd client && ./node_modules/.bin/ts-node main.ts

client-browser:
	cd client/browser && npm start