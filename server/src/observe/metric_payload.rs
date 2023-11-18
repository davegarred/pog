use crate::observe::emf::MetricMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricPayload {
    #[serde(rename = "_aws")]
    aws: MetricMetadata,
    #[serde(flatten)]
    dimensions: HashMap<String, String>,
    #[serde(flatten)]
    counts: HashMap<String, u64>,
    #[serde(flatten)]
    timers: HashMap<String, u64>,
}

impl MetricPayload {
    pub fn new(
        aws: MetricMetadata,
        dimensions: HashMap<String, String>,
        counts: HashMap<String, u64>,
        timers: HashMap<String, u64>,
    ) -> Self {
        Self {
            aws,
            dimensions,
            counts,
            timers,
        }
    }
}
