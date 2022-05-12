# ../target/release/parachain-collator build-spec --disable-default-bootnode > ./specs/rococo-local-sublink-parachain-plain-2001.json
../target/release/parachain-collator build-spec --chain ./specs/rococo-local-sublink-parachain-plain-2001.json --raw --disable-default-bootnode > ./specs/rococo-local-sublink-parachain-2001-raw.json
../target/release/parachain-collator export-genesis-wasm --chain ./specs/rococo-local-sublink-parachain-2001-raw.json > ./specs/sublink-para-2001-wasm
../target/release/parachain-collator export-genesis-state --chain ./specs/rococo-local-sublink-parachain-2001-raw.json > ./specs/sublink-para-2001-genesis
