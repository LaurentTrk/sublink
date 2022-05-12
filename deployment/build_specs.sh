../target/release/parachain-collator build-spec --disable-default-bootnode > ./specs/rococo-local-sublink-parachain-plain.json
../target/release/parachain-collator build-spec --chain ./specs/rococo-local-sublink-parachain-plain.json --raw --disable-default-bootnode > ./specs/rococo-local-sublink-parachain-2000-raw.json
../target/release/parachain-collator export-genesis-wasm --chain ./specs/rococo-local-sublink-parachain-2000-raw.json > ./specs/sublink-para-2000-wasm
../target/release/parachain-collator export-genesis-state --chain ./specs/rococo-local-sublink-parachain-2000-raw.json > ./specs/sublink-para-2000-genesis
