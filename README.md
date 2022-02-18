# Totem Parachain Repository

**This repo contains the code for the Totem Parachain Networks, Lego (intended for Rococo Relaychain - both local and Parity Hosted), Wapex (intended Westend Test Relaychain) and Kapex (intended for Polkadot Live Network).**

It is currently aligned with `Polkadot v0.9.16`

The project is a direct fork of the [Substrate Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template) modified for use with Totem Balances and Transaction Payment pallets which are reliant on the core Accounting Engine developed by the Totem Development Team.

## Build Instructions

This is Totem's generic parachain collator executor, but will need to be run using the appropriate chain spec raw files found in the `./res` directory.

To build the code rust binary:

    cargo build --release -p totem-parachain-node

To build the Docker image

    docker build \
    -t <your-tag> \
    -f parachain_collator_builder.Dockerfile \
    --build-arg chain=totem-parachain-node \
    --build-arg buildtype=build .

## Run the collator

Running the collator node requires deciding which relaychain the node should be collating for, and obtaining an appropriate chainspec file to pass in to the execution command. Consult the Substrate Parachain Template documentation for more information. This is also the case when running a Docker container.

An example is as follows (you will have to substitute the appropriate chainspecs):

```shell
#!/usr/bin/env bash

docker run \
-it \
-p 40333:40333 \
-p 30333:30333 \
--name <your-node-name> \
--pull=always \
-v="/$(pwd)/<your-node-name>:/data" \
totemlive/totem-parachain-collator:latest \
parachain-collator \
--state-cache-size=1 \
--chain <path/to/genesis-files>lego-parachain-raw.json \
--name "<your-node-name>" \
--collator \
--execution=wasm \
--keystore-path <path/to/keystore> \
--node-key-file <path/to/keystore/file> \
--node-key-type 'ed25519' \
--public-addr /ip4/<your-ip>/tcp/30333 \
--port 30333 \
-- \
--chain <path/to/genesis-files>relay-chain-spec.json \
--execution=wasm \
--public-addr /ip4/<your-ip>/tcp/40333 \
--port 40333
```