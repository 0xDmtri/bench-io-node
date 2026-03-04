# base-provider-bench

Minimal Criterion benchmark crate for the Base provider access pattern you pasted.

This manifest pins the `reth` crates to commit `8e3b5e6a99439561b73c5dd31bd3eced2e994d60`, because that revision matches the two-argument `open_read_only(...)` API used by the benchmark.

The direct `reth-db` dependency is intentional: it enables the `op` feature so the optimism DB codecs needed by `OpNode` are present during compilation.

## Run

Use the default datadir:

```bash
cargo bench --bench provider_basic_base
```

Use a custom datadir without editing the source:

```bash
BASE_OP_RETH_DATA_DIR=/path/to/op-reth cargo bench --bench provider_basic_base
```

If you already have the dependencies cached locally, you can also run Cargo offline:

```bash
cargo bench --offline --bench provider_basic_base
```
