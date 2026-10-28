# budgetgate-action-test

End-to-end test repository for [soroban-budget-action](https://github.com/BudgetGate/soroban-budget-action). Not a product; nothing here is meant for reuse.

`budget/fixture.json` calls `bump` on the counter contract from [soroban-budget-core `testdata/counter/`](https://github.com/BudgetGate/soroban-budget-core/tree/main/testdata/counter), deployed on testnet. `main` uses the v1 contract; pull requests switch the fixture to v2 (a known cost rise) or break the RPC URL to exercise the Action's regression and failed-run comments.

Testnet resets remove the contracts; redeploy them with the steps in core's `testdata/counter/README.md` and update the fixture.
