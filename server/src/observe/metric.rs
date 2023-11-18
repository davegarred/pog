use std::collections::HashMap;
use std::mem;

use crate::observe::emf::{Dimensions, Metric, MetricMetadata};
use crate::observe::metric_payload::MetricPayload;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Metrics {
    dimensions: HashMap<String, String>,
    counts: HashMap<String, u64>,
    timers: HashMap<String, u64>,
}

impl Metrics {
    pub fn finish(&mut self, namespace: &str) -> MetricPayload {
        let dimension_metadata: Dimensions =
            self.dimensions.keys().map(|k| k.to_string()).collect();
        let mut metric_metadata: Vec<Metric> =
            self.counts.keys().map(|k| Metric::new_count(k)).collect();
        let mut timer_metric_metadata: Vec<Metric> =
            self.timers.keys().map(|k| Metric::new_timer(k)).collect();
        metric_metadata.append(&mut timer_metric_metadata);
        let dimensions: HashMap<String, String> = self.dimensions.clone();
        let mut counts: HashMap<String, u64> = HashMap::default();
        let mut timers: HashMap<String, u64> = HashMap::default();
        mem::swap(&mut counts, &mut self.counts);
        mem::swap(&mut timers, &mut self.timers);
        let aws = MetricMetadata::new(namespace, vec![dimension_metadata], metric_metadata);
        MetricPayload::new(aws, dimensions, counts, timers)
    }

    pub fn dimension(&mut self, key: &str, value: &str) {
        self.dimensions.insert(key.to_string(), value.to_string());
    }
    pub fn count(&mut self, key: &str) {
        match self.counts.get_mut(key) {
            Some(count) => {
                *count += 1;
            }
            None => {
                self.counts.insert(key.to_string(), 1);
            }
        }
    }
    pub fn timer(&mut self, key: &str, millis: u64) {
        self.timers.insert(key.to_string(), millis);
    }
}
