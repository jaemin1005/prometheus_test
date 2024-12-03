use prometheus::{Encoder, TextEncoder, Counter, Opts, Registry};
use warp::Filter;

// https://docs.rs/prometheus/latest/prometheus/
#[tokio::main]
async fn main() {
    // 1. 레지스토리 생성 (Prometheus 메트릭을 관리하는 중앙 저장소)
    let registry = Registry::new();

    // 2. 카운터 메트릭 생성
    let counter_opt = Opts::new("test_counter", "test_counter_help");
    let counter = Counter::with_opts(counter_opt).unwrap();    

    // 3. 레지스토리에 메트릭 등록
    registry.register(Box::new(counter.clone())).unwrap();

    let metrics_route = warp::path("metrics").map(move || {
        let encoder = TextEncoder::new();
        let mut buffer = vec![];
        let metric_families = registry.gather();

        encoder.encode(&metric_families, &mut buffer).unwrap();  // 메트릭을 텍스트로 변환
        String::from_utf8(buffer).unwrap()
    });

    println!("http://localhost:8080/metrics");

    warp::serve(metrics_route).run(([127, 0, 0, 1], 8080)).await;
}
