../target/release/parachain-collator \
--alice \
--collator \
--force-authoring \
--chain ./specs/rococo-local-sublink-parachain-2001-raw.json \
--base-path ./chains/sublink2001/alice \
--port 40334 \
--ws-port 8845 \
-linfo,runtime::contracts=debug \
-- \
--execution wasm \
--chain ./specs/relaychain.spec \
--port 30344 \
--ws-port 9978 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLcNx6Lzrq6M3EtVfciBP6dXFt5S6iwavPJ6FA7A5nsPQ \
