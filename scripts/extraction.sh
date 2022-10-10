
# !!!!!!!!!!! These scripts are not intented to be executed by calling this file directly as there are manual tasks involved.

#______________________________________________________________#
# Compile generic totem parachain node
#______________________________________________________________#

# cargo build --release -p totem-parachain-node

#______________________________________________________________#
# Then extract the local version of the chainspec.
# This will provide ther WASM blob that needs to be added to the readable chainspec
#______________________________________________________________#

# ./target/release/totem-parachain-collator build-spec --chain local > ./res/chainspec-for-wasm-new.json

#______________________________________________________________#
# Once the chain specific details have been added the file must be renamed according to the specific chain and placed in the appropriate directory. 
# IMPORTANT! Use the naming convention of the 'chain' below 
# Then execute the next step to convert the readable chainspec to raw format for sharing.
#______________________________________________________________#

# ./target/release/totem-parachain-collator build-spec --chain ./res/stagex/stagex-parachain-readable.json --raw > ./res/stagex/stagex-parachain-raw-new.json && \
# ./target/release/totem-parachain-collator build-spec --chain ./res/wapex/wapex-parachain-readable.json --raw > ./res/wapex/wapex-parachain-raw-new.json && \
# ./target/release/totem-parachain-collator build-spec --chain ./res/kapex/kapex-parachain-readable.json --raw > ./res/kapex/kapex-parachain-raw-new.json && \
# ./target/release/totem-parachain-collator build-spec --chain ./res/lego/lego-parachain-readable.json --raw > ./res/lego/lego-parachain-raw-new.json

#______________________________________________________________#
# The raw file is ready to be shared and also used as the basis to extract the genesis states for each file.
# IMPORTANT! the Genesis WASM is identical for all chains therefore does not need to be extracted more than once.
# IMPORTANT! the Genesis STATE is unique for each chain spec file, therefore must be uniquely extracted.
#______________________________________________________________#

# ./target/release/totem-parachain-collator export-genesis-wasm --chain ./res/kapex/kapex-parachain-raw.json > ./res/parachain-genesis-wasm-new.wasm

# ./target/release/totem-parachain-collator export-genesis-state --chain ./res/lego/lego-parachain-raw.json > ./res/lego/lego-genesis-state-new.state && \
# ./target/release/totem-parachain-collator export-genesis-state --chain ./res/stagex/stagex-parachain-raw.json > ./res/stagex/stagex-genesis-state-new.state && \
# ./target/release/totem-parachain-collator export-genesis-state --chain ./res/wapex/wapex-parachain-raw.json > ./res/wapex/wapex-genesis-state-new.state && \
# ./target/release/totem-parachain-collator export-genesis-state --chain ./res/kapex/kapex-parachain-raw.json > ./res/kapex/kapex-genesis-state-new.state


#______________________________________________________________#
# Cleanup
#______________________________________________________________#

# Rename all the *-new files to just their normal names

#______________________________________________________________#
# Note on how to extract a local chainspec from polkadot (not in this repo)

#./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json