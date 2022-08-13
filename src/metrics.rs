use lazy_static::lazy_static;
use prometheus::{
    opts, register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec,
};

const HTTP_RESPONSE_TIME_BUCKETS: &[f64; 8] = &[0.1, 0.2, 0.3, 0.5, 0.8, 1.0, 1.5, 2.0];

lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec =
        register_int_counter_vec!(opts!("http_requests_total", "HTTP requests total"), &[])
            .expect("Can't create a metric");
    pub static ref HTTP_REQUESTS_ERRORS: IntCounterVec =
        register_int_counter_vec!(opts!("http_requests_errors", "HTTP requests errors"), &[])
            .expect("Can't create a metric");
    pub static ref HTTP_RESPONSE_TIME_SECONDS: HistogramVec = register_histogram_vec!(
        "http_response_time_seconds",
        "HTTP response time",
        &[],
        HTTP_RESPONSE_TIME_BUCKETS.to_vec()
    )
    .expect("Can't create a metric");
}
