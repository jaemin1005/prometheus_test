use prometheus::{Counter, Encoder, Gauge, Histogram, Opts, Registry, TextEncoder};
use warp::Filter;

// https://docs.rs/prometheus/latest/prometheus/
#[tokio::main]
async fn main() {
    // 1. 레지스토리 생성 (Prometheus 메트릭을 관리하는 중앙 저장소)
    let registry = Registry::new();

    // 2. Counter : 현재 요청 수
    let request_counter_opts = Opts::new("http_requests_total", "Total number of HTTP requests");
    let request_counter = Counter::with_opts(request_counter_opts).unwrap();
    registry
        .register(Box::new(request_counter.clone()))
        .unwrap();

    // 3. Gauge : 현재 연결 수
    let active_connections_opts = Opts::new("active_connections", "Number of active connections");
    let active_connections = Gauge::with_opts(active_connections_opts).unwrap();
    registry
        .register(Box::new(active_connections.clone()))
        .unwrap();

    // 4. Histogram : 요청 응답 시간 분포
    let response_time_opts = Opts::new("http_response_time_seconds", "Response time in seconds");
    let response_time_histogram = Histogram::with_opts(response_time_opts.into()).unwrap();
    registry
        .register(Box::new(response_time_histogram.clone()))
        .unwrap();

    let metrics_route = warp::path("metrics").map(move || {
        let encoder = TextEncoder::new();
        let mut buffer = vec![];
        let metric_families = registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    });

    let hello_route = warp::path::end().map(move || {
        request_counter.inc(); // 요청 수 증가
        active_connections.inc(); // 활성 연결 수 증가

        // 요청 처리 시간 측정
        let start = std::time::Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(100)); // 처리 시뮬레이션
        let elapsed = start.elapsed().as_secs_f64();

        response_time_histogram.observe(elapsed); // 응답 시간 기록 (Histogram)
        active_connections.dec(); // 처리 후 활성 연결 수 감소

        "Hello, Prometheus!"
    });

    println!("Server running at http://localhost:8080/");
    println!("Metrics available at http://localhost:8080/metrics");

    // Warp 서버 실행
    warp::serve(metrics_route.or(hello_route)).run(([127, 0, 0, 1], 8080)).await;
}
