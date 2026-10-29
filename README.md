# budgetgate-action-test

End-to-end test repository for [soroban-budget-action](https://github.com/BudgetGate/soroban-budget-action). Not a product; nothing here is meant for reuse.

`contracts/counter/` is the counter contract from [soroban-budget-core `testdata/counter/`](https://github.com/BudgetGate/soroban-budget-core/tree/main/testdata/counter) (v1 on `main`). Every run builds it with the Rust version pinned in `rust-toolchain.toml`, deploys it to testnet with a throwaway friendbot-funded key, and writes `budget/fixture.json` against that deployment (`budget/build-fixture.sh`, the Action's `examples/build-fixture.sh` with `WASM` and `CALLS` set), then runs the Action. A pull request is therefore measured on its own code: changing `contracts/counter/src/lib.rs` to the v2 code (a known cost rise) shows a regression. Testnet resets do not affect it, since nothing outlives a run.
