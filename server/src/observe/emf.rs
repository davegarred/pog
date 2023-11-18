use chrono::Utc;
use serde::{Deserialize, Serialize};

//
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricMetadata {
    #[serde(rename = "Timestamp")]
    timestamp: u64,
    #[serde(rename = "CloudWatchMetrics")]
    cloudwatch_metrics: Vec<CloudWatchMetrics>,
}

impl MetricMetadata {
    pub fn new(namespace: &str, dimensions: Vec<Dimensions>, metrics: Vec<Metric>) -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            cloudwatch_metrics: vec![CloudWatchMetrics {
                namespace: namespace.to_string(),
                dimensions,
                metrics,
            }],
        }
    }
    pub fn update(&mut self) {
        self.timestamp = Utc::now().timestamp_millis() as u64;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchMetrics {
    #[serde(rename = "Namespace")]
    namespace: String,
    #[serde(rename = "Dimensions")]
    dimensions: Vec<Dimensions>,
    #[serde(rename = "Metrics")]
    metrics: Vec<Metric>,
}

pub type Dimensions = Vec<String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Unit")]
    unit: String,
}

impl Metric {
    pub fn new_count(metric_name: &str) -> Self {
        Self {
            name: metric_name.to_string(),
            unit: "Count".to_string(),
        }
    }
    pub fn new_timer(metric_name: &str) -> Self {
        Self {
            name: metric_name.to_string(),
            unit: "Microseconds".to_string(),
        }
    }
}
