localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 10 localnet/alice.json -u localhost

deploy:
	cd donations_system; cargo build-bpf
	solana donations_system deploy donations_system/target/deploy/donations_system.so -u localhost --donations_system-id localnet/donations_system.json

testnet-deploy:
	cd donations_system; cargo build-bpf
	solana donations_system deploy donations_system/target/deploy/donations_system.so -u testnet --donations_system-id localnet/donations_system.json --keypair localnet/alice.json --upgrade-authority localnet/alice.json

client-cli:
	cd client/cli && npm install && ./node_modules/.bin/ts-node main.ts

client-browser:
	cd client/browser && npm install && npm start