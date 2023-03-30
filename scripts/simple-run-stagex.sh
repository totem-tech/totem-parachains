#!/usr/bin/env bash

../target/release/totem-parachain-collator \
--chain=stagex \
--execution=wasm \
--name "quick-run-stagex" \
--port 30333 \
-- \
--chain=rococo \
--port 40333
