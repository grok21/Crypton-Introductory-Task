build:
	cd program; cargo build-bpf
	cd client; npm install

localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 10 localnet/alice.json -u localhost
	solana airdrop 10 localnet/bob.json -u localhost

deploy:
	solana program deploy program/target/deploy/program.so -u localhost --program-id localnet/program.json 

testnet-deploy:
	cd program; cargo build-bpf
	solana program deploy program/target/deploy/program.so -u testnet --program-id localnet/program.json --keypair localnet/alice.json --upgrade-authority localnet/alice.json

client-cli:
	cd client && ./node_modules/.bin/ts-node main.ts

client-browser:
	cd client && npm start

sollet-balance-init:
	solana airdrop 10 4m6sxBWs7QBGn9sAz1aSh5no87Az5wp87DJQaqgp2qSy -u localhost