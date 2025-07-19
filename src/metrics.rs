use warp::Filter;

#[derive(Debug)]
pub struct PrometheusMetrics {
    connection_up: prometheus::IntGaugeVec,
    request_count: prometheus::IntCounterVec,
    response_count: prometheus::IntCounterVec,
    poll_result_count: prometheus::IntCounterVec,
    response_time: prometheus::HistogramVec,
}

impl PrometheusMetrics {
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
            poll_result_count: prometheus::register_int_counter_vec!(
                "poll_result_count",
                "Number and type of responses received to poll commands",
                &["id", "command"]
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
            metrics: self.clone(),
            id,
        }
    }
}

pub trait Metrics: Clone + Send + Sync {
    type Subordinate: Metrics;

    fn connection_status(&self, up: bool);
    fn request_sent(&self);
    fn response_received(&self);
    fn poll_received(&self, command: &str);
    fn record_response_time(&self, command: &str) -> Option<prometheus::HistogramTimer>;
    fn subordinate(&self, extra: &str) -> Self::Subordinate;
}

#[derive(Debug, Clone)]
pub struct ScopedMetrics {
    metrics: std::sync::Arc<PrometheusMetrics>,
    id: String,
}

impl Metrics for ScopedMetrics {
    type Subordinate = ScopedMetrics;

    fn connection_status(&self, up: bool) {
        self.metrics
            .connection_up
            .with_label_values(&[&self.id])
            .set(if up { 1 } else { 0 });
    }

    fn request_sent(&self) {
        self.metrics.request_count.with_label_values(&[&self.id]).inc();
    }

    fn response_received(&self) {
        self.metrics.response_count.with_label_values(&[&self.id]).inc();
    }

    fn poll_received(&self, command: &str) {
        self.metrics.poll_result_count.with_label_values([&self.id, command].as_slice()).inc();
    }

    fn record_response_time(&self, command: &str) -> Option<prometheus::HistogramTimer> {
        Some(self.metrics.response_time
            .with_label_values([&self.id, command].as_slice())
            .start_timer())
    }

    fn subordinate(&self, extra: &str) -> Self {
        ScopedMetrics {
            metrics: self.metrics.clone(),
            id: format!("{}_{}", self.id, extra),
        }
    }
}

#[derive(Default, Clone)]
pub struct DummyMetrics {}

impl Metrics for DummyMetrics {
    type Subordinate = DummyMetrics;

    fn connection_status(&self, _up: bool) {}
    fn request_sent(&self) {}
    fn response_received(&self) {}
    fn poll_received(&self, _command: &str) {}
    fn record_response_time(&self, _command: &str) -> Option<prometheus::HistogramTimer> {
        None
    }
    fn subordinate(&self, _extra: &str) -> Self {
        DummyMetrics::default()
    }
}


pub fn start_metrics_server(addr: std::net::SocketAddr) {
    let metrics_route = warp::path!("metrics").and_then(metrics_handler);

    info!("Starting metrics server on {}", addr);
    tokio::task::spawn(async move {
        warp::serve(metrics_route).run(addr).await;
    });
}

async fn metrics_handler() -> Result<impl warp::Reply, warp::Rejection> {
    use prometheus::Encoder;

    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };

    let res = String::from_utf8(buffer.clone()).unwrap_or_else(|e| {
        eprintln!("custom metrics could not be from_utf8'd: {}", e);
        String::default()
    });

    Ok(res)
}