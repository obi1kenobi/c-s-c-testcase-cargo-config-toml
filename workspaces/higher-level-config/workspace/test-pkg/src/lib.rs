pub fn get_max_steal_operations(metrics: &tokio_metrics::RuntimeMetrics) -> u64 {
    metrics.max_steal_operations
}
