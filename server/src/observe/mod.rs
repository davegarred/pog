pub use metric::Metrics;
pub use timer::Timer;

mod emf;
mod metric;
mod metric_payload;
mod timer;

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
