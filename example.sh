#!/bin/bash

wsm="target/wasm32-wasip1/release-wasi/rs-ua2json.wasm"

##
# Prints multiple sample user agents to stdout.
#
sample_user_agents() {
	cat sample.txt
}

##
# Reads user agent strings from stdin and processes them using the WASM module.
#
process_lines() {
	while IFS= read -r ua; do
		# Skip empty lines
		[[ -z "${ua}" ]] && continue

		wasmtime run "${wsm}" --user-agent "${ua}"
	done
}

# Ensure the WASM module exists before running
if [[ ! -f "${wsm}" ]]; then
	echo "WASM module not found at ${wsm}. Please run ./wasi.sh first." >&2
	exit 1
fi

echo original user agents
sample_user_agents
echo

echo converted
sample_user_agents | process_lines | jq -c
