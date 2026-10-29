#!/usr/bin/env bash
# Deploys this commit's contract to testnet and writes budget/fixture.json, so Budget Gate
# measures the code in this commit rather than a contract deployed earlier.
# Uses a throwaway key funded by friendbot: no secrets needed. Needs the stellar CLI and
# the wasm32v1-none Rust target.
set -euo pipefail

network=testnet
key=budget-gate-run

# Testnet RPC calls sometimes time out; try each network step up to three times.
retry() {
  local attempt
  for attempt in 1 2 3; do
    "$@" && return 0
    echo "Attempt $attempt of '$1 $2 $3' failed." >&2
    [ "$attempt" -lt 3 ] && sleep 15
  done
  return 1
}

retry stellar keys generate "$key" --network "$network" --fund --overwrite
stellar contract build
id="$(retry stellar contract deploy --wasm target/wasm32v1-none/release/counter.wasm --source "$key" --network "$network")"
echo "Deployed this commit's contract: $id" >&2

# One entry per function to measure. Add arguments after the function name, as for
# `stellar contract invoke`.
bump="$(retry stellar contract invoke --id "$id" --source "$key" --network "$network" --build-only -- bump)"

mkdir -p budget
printf '[{"name":"bump","fixture_id":"counter_bump","xdr":"%s"}]\n' "$bump" > budget/fixture.json
