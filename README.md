# Totem Parachain Repository

**This repo contains the code for the Totem Parachain Networks, Lego (intended for Rococo Relaychain - both local and Parity Hosted), Wapex (intended Westend Test Relaychain) and Kapex (intended for Polkadot Live Network).**

It is currently aligned with `Polkadot v0.9.18` which was the version of Polkadot the day before the genesis states were uploaded into Polkadot.

The Lego version will be updated before the upgrade is created for Polkadot.

The project is a direct fork of the [Substrate Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template) modified for use with 'Totem Balances' and `Totem Transaction Payment` pallets which are reliant on the `Core Accounting Engine` developed by the Totem Development Team.

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

Running the collator node no longer requires deciding which relaychain the node should be collating for, and obtaining an appropriate chainspec file to run it. 

Also the parachain chainspec file is now incorporated into the start commands of the parachain, so there is no need to include the chainspec for the parachain either.

## Run the docker image

Use the following version from dockerhub 

    $ docker pull totemlive/totem-parachain-collator:v1.1.0-release

Example of the start commands:

```shell
#!/usr/bin/env bash

docker run \
-it \
-p 30333:30333 \
-p 40333:40333 \
--name <your-node-name> \
--pull=always \
-v="/$(pwd)/<your-node-name>:/data" \
totemlive/totem-parachain-collator:v1.1.0-release \
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