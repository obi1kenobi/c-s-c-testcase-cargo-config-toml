pub fn get_max_steal_operations(metrics: &tokio_metrics::RuntimeMetrics) -> u64 {
    metrics.max_steal_operations
}

// `cargo doc` largely ignores type errors inside method bodies,
// to simplify running it on e.g. Linux while documenting Windows types or vice versa.
// But it can't ignore an explicit `std::compile_error!()` at top level.
#[cfg(not(tokio_unstable))]
std::compile_error!("not able to be used without `--cfg tokio_unstable`");
