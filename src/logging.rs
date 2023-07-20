use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

pub fn init_logging() {
    let srv_name = std::env::var("APP_NAME").unwrap_or("App".to_string());
    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production application.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer: opentelemetry::sdk::trace::Tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(srv_name)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to install OpenTelemetry tracer.");
    // Create a `tracing` layer using the Jaeger tracer
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    // Create a `tracing` layer to emit spans as structured logs to stdout
    // let std_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
    let std_layer = fmt::layer().with_writer(std::io::stderr);
    // Combined them all together in a `tracing` subscriber
    let registry = Registry::default()
        .with(env_filter)
        .with(telemetry_layer)
        .with(JsonStorageLayer)
        .with(std_layer);

    registry
        .try_init()
        .expect("Failed to install `tracing` subscriber.");
}
