# Totem Parachain Repository

**This repo contains the code for the Totem Parachain Networks, Lego (intended for Rococo Locoal Relaychain), Stages (intended Parity Hosted Rococo Live Relaychain) and Kapex (intended for Polkadot Live Network).**

The code linked to Polkadot, Cumulus and Substrate is currently aligned with 

* `Polkadot v0.9.36` 

and is compatible with 

* `rustc version 1.67 nightly`

The project is a direct fork of the [Substrate Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template) modified for use with `Totem Balances Pallet` and `Totem Transaction Payment` pallets which are reliant on the `Core Accounting Engine` developed by the Totem Development Team.

## Build Instructions

This is Totem's generic parachain collator executor, but will need to be run using the appropriate chain spec raw files found in the `./res` directory.

To build the code rust binary:

    cargo build --release --locked

To build the Docker image:

Note this is still configured to build node by referencing the package, although this is no longer technically necessary.

    docker build \
    -t <your-tag> \
    -f parachain_collator_builder.Dockerfile \
    --build-arg chain=totem-parachain-node \
    --build-arg buildtype=build .

## Running the parachain

You can choose the appropriate arguments in your chain start script according to which network you wish to join:

#### Local Test Lego Network:

    chain=lego
    ...
    chain=rococo-local

#### Public Test Stagex Network:

    chain=stagex
    ...
    chain=rococo

#### Public Production Kapex Network:

    chain=kapex
    ...
    chain=polkadot


## Run the docker image

Use the following version from dockerhub 

    $ docker pull totemlive/totem-parachain-collator:v1.4.1-release

Refer to the [Technical Documentation](https://docs.totemaccounting.com) for the various different types of nodes.

#### Running natively from the compiled code

An example is as follows (you will have to substitute the appropriate chainspecs):

```shell
#!/usr/bin/env bash

./target/release/totem-parachain-collator \
--state-cache-size=1 \ # recommended
--chain kapex \ # mandatory - name of parachain you are running 
--name "<your-node-name>" \
--execution=wasm \ # mandatory
--collator \ # optional unless collatornode
--keystore-path <path/to/keystore> \ # optional unless collatornode
--node-key-file <path/to/keystore/file> \ # optional unless bootnode
--node-key-type 'ed25519' \ # optional unless bootnode
--public-addr /ip4/<your-ip>/tcp/30333 \ # optional
--port 30333 \ # recommended to specify port
-- \ # next args are for the relaychain
--chain polkadot \ # mandatory - name of relaychain you are running
--public-addr /ip4/<your-ip>/tcp/40333 \ # optional
--port 40333 # recommended to specify port
```

## Run Benchmarks

To run the benchmark you will first have to compile the node with the `runtime-benchmarks` feature set:

    cargo build --release --features runtime-benchmarks -p totem-parachain-node   

To build the new weights files you will need to use the following command in the root of the repo:

    ./scripts/benchmark.sh

This will produce benchmark files for each of the pallets and intsert them into the `weights` directory. From there they need to be analysed and the information included in the appropriate `weights.rs` file for each pallet.