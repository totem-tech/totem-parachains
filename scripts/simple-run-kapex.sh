#!/usr/bin/env bash

../target/release/totem-parachain-collator \
--chain=kapex \
--execution=wasm \
--name "quick-run-kapex" \
--port 30333 \
-- \
--chain=polkadot \
--port 40333