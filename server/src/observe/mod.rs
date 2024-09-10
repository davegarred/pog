use crate::POG_METRIC;
pub use metric::Metrics;
use std::sync::MutexGuard;
pub use timer::Timer;

pub mod emf;
pub mod metric;
pub mod metric_payload;
pub mod timer;

pub fn metric(f: impl Fn(MutexGuard<Metrics>)) {
    let pog_metric = match POG_METRIC.get() {
        Some(metric) => metric.clone(),
        None => {
            println!("no metrics configured");
            return;
        }
    };
    let metric = pog_metric.lock().unwrap();
    f(metric);
}
pub async fn reset_metric(namespace: &str) {
    metric(|mut m| {
        // let payload = m.finish("pog_dev");
        let payload = m.finish(namespace);
        let payload = serde_json::to_string(&payload).unwrap();
        println!("{}", payload);
    });
}

#[cfg(test)]
mod test_ser {
    use crate::observe::metric::Metrics;
    #[test]
    fn serialize() {
        let mut metrics = Metrics::default();
        metrics.dimension("environment", "prod");
        metrics.count("testA");
        metrics.count("testB");
        metrics.count("testA");

        let payload = metrics.finish("test-namespace");
        let ser = serde_json::to_string(&payload).unwrap();
        println!("{}", ser);
    }
}
