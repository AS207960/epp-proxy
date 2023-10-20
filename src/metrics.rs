#[derive(Debug)]
pub struct Metrics {
    connection_up: prometheus::IntGaugeVec,
    request_count: prometheus::IntCounterVec,
    response_count: prometheus::IntCounterVec,
    response_time: prometheus::HistogramVec,
}

impl Metrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            connection_up: prometheus::register_int_gauge_vec!(
                "connection_up",
                "Is the client connected to the EPP server",
                &["id"]
            )?,
            request_count: prometheus::register_int_counter_vec!(
                "request_count",
                "Number of requests sent to the EPP server",
                &["id"]
            )?,
            response_count: prometheus::register_int_counter_vec!(
                "response_count",
                "Number of responses received from the EPP server",
                &["id"]
            )?,
            response_time: prometheus::register_histogram_vec!(
                "response_time",
                "Time the EPP server took to respond to commands",
                &["id", "command"]
            )?,
        })
    }

    pub fn new_scope(self: &std::sync::Arc<Self>, id: String) -> ScopedMetrics {
        ScopedMetrics {
            metrics: Some(self.clone()),
            id,
        }
    }

    pub fn null() -> ScopedMetrics {
        ScopedMetrics {
            metrics: None,
            id: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScopedMetrics {
    metrics: Option<std::sync::Arc<Metrics>>,
    id: String,
}

impl ScopedMetrics {
    pub(crate) fn connection_status(&self, up: bool) {
        if let Some(metrics) = &self.metrics {
            metrics
                .connection_up
                .with_label_values(&[&self.id])
                .set(if up { 1 } else { 0 })
        }
    }

    pub(crate) fn request_sent(&self) {
        if let Some(metrics) = &self.metrics {
            metrics.request_count.with_label_values(&[&self.id]).inc();
        }
    }

    pub(crate) fn response_received(&self) {
        if let Some(metrics) = &self.metrics {
            metrics.response_count.with_label_values(&[&self.id]).inc();
        }
    }

    pub(crate) fn record_response_time(&self, command: &str) -> Option<prometheus::HistogramTimer> {
        self.metrics.as_ref().map(|metrics| metrics
                    .response_time
                    .with_label_values(&[&self.id, command])
                    .start_timer())
    }

    pub(crate) fn subordinate(&self, extra: &str) -> ScopedMetrics {
        ScopedMetrics {
            metrics: self.metrics.clone(),
            id: format!("{}_{}", self.id, extra),
        }
    }
}
