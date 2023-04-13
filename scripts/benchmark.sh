#!/usr/bin/env bash
# To run this, you must be in the root directory.
# Execute ./scripts/benchmark.sh
# Runtime is dev by default because this contains the benchmarking methods
# After completion the generated benchmark files should be copied to the relevant pallets directory
# and renamed to `weights.rs`

# Set the number of steps and repeat for the benchmark
steps=50
repeat=20

# Set the default output directory for the benchmark
# this is used so as not to immediately overwrite the existing benchmark files
benchmarkOutput=./weights

# Set the pallets to benchmark
# Should include all pallets in the runtime
pallets=(
    pallet_balances_totem,
    pallet_teams
)

# Loop through the pallets and run the benchmark
for pallet in ${pallets[@]}
do
	output_file="${pallet//::/_}"
	extra_args=""

	./target/release/totem-parachain-collator benchmark pallet \
		$extra_args \
		--chain="dev" \
		--execution=wasm \
		--wasm-execution=compiled \
		--pallet=$pallet  \
		--extrinsic='*' \
		--steps=$steps  \
		--repeat=$repeat \
		--json \
		--output="${benchmarkOutput}/${output_file}-new-test.rs" >> $benchmarkOutput/${pallet}_benchmark.json
done
