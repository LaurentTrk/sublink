# Deployment

A few tips to deploy the whole project, this is not a plug and play setup :)

## Chainlink Node

```bash
# You need to create a kovan.env file from the kovan.env.template
kubectl create cm kovan-env --from-env-file=kovan.env
# And api.env and password.env secrets with api and password values to access chainlink nodes
# (see https://github.com/LaurentTrk/chainlink-polkadot/blob/sublink/runlog-demo/internal-scripts/common.sh)
kubectl apply -f kovan.yaml
```

## Reverse proxy and Apps

This part is very specific to the SubLink deployment on https://*.ltk.codes

```bash
kubectl create cm sublink-nginx-config --from-file proxy.conf
kubectl apply -f sublink-svc.yaml
kubectl apply -f nginx.yaml
kubectl apply -f apps.yaml
```

## Relay Chain

```bash
kubectl apply -f ./relaychain.yaml
# Register 2000 and 2001 parachains id
# https://sublink.ltk.codes/#/parachains/parathreads
```

## SubLink and Defi Chains

```bash
cargo build release
cd deployment/ && ./build_specs.sh && cd ..
cp target/release/parachain-collator . && docker build . -t laurenttrk/sublink(-defichain):v0.0.xx
docker push laurenttrk/sublink(-defichain):v0.0.xx
# Change image tag in sublink(2001).yaml
# Register parachain in relay chain with genesis and wasm 
# https://sublink.ltk.codes/#/sudo (sudoScheduleParaInitialize)
kubectl apply -f sublink(2001).yaml
```

# Adapters

```bash
# You need to create substrate_adapter env from template for 3 differents accounts (to generate)
# (see https://github.com/LaurentTrk/chainlink-polkadot/blob/sublink/runlog-demo/internal-scripts/add-ei.sh)
kubectl create cm substrate-adapter(x)-env --from-env-file=substrate_adapter(x).env
# Then deploy adapters
kubectl apply -f substrate_adapter.yaml
```

# Configuration

```bash
# Send funds to Chainlink Adapter Accounts (x3)

# Create bi directional XCM channel 
# https://sublink.ltk.codes/#/sudo (sudoEstablishHrmpChannel x 2)

# Create feed in Chainlink Price feed pallet
# https://sublink.ltk.codes/?rpc=wss%3A%2F%2Fsublinkchain.ltk.codes#/extrinsics

# Submit new price value
```

