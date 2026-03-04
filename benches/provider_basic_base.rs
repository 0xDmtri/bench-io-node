//! Usage:
//! cargo bench --bench provider_basic_base

use std::{hint::black_box, path::Path};

use alloy_primitives::{Address, address};
use criterion::{Criterion, criterion_group, criterion_main};
use reth_optimism_chainspec::BASE_MAINNET;
use reth_optimism_node::OpNode;
use reth_provider::{BlockNumReader, providers::ReadOnlyConfig};

const DEFAULT_DATA_DIR_BASE: &str = "/data/base/op-reth";
const TARGET_ADDR: Address = address!("4200000000000000000000000000000000000006");

fn data_dir_base() -> String {
    std::env::var("BASE_OP_RETH_DATA_DIR").unwrap_or_else(|_| DEFAULT_DATA_DIR_BASE.to_owned())
}

fn bench_provider_basic_account(c: &mut Criterion) {
    let data_dir = data_dir_base();

    assert!(
        Path::new(&data_dir).exists(),
        "Base node dir not found at {data_dir}"
    );

    let config = ReadOnlyConfig::from_datadir(&data_dir);
    let factory = OpNode::provider_factory_builder()
        .open_read_only(BASE_MAINNET.clone(), config)
        .unwrap();
    let block_number = factory.best_block_number().unwrap();

    c.bench_function("base: ProviderFactory.history_by_block_number + basic_account", |b| {
        b.iter(|| {
            let provider = factory.history_by_block_number(block_number).unwrap();
            let account = provider.basic_account(&TARGET_ADDR).unwrap();
            black_box(account);
        })
    });
}

criterion_group!(benches, bench_provider_basic_account);
criterion_main!(benches);
