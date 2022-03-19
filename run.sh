cargo build -p totem-parachain-node
./target/debug/totem-parachain-collator \
    --chain lego \
    --execution=wasm \
    --name "lego-ui-node-1" \
    --bootnodes  "/ip4/159.89.1.153/tcp/31436/ws/p2p/12D3KooWCwPGSVmQSLTRMgaU1GaK6oHXijKdFcMiCBzzKygxVcXL" \
    --port 30333 \
    -- \
    --chain 'relaychain-local' \
    --execution=wasm \
    --bootnodes  "/ip4/159.89.1.153/tcp/31433/p2p/12D3KooWJV6xLXVuV8o571YigbHFBcyYuRdfCVctzh5wVTHHt9CH" \
    --port 40333;
