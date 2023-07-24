use opentelemetry::sdk::trace::Tracer;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{layer::Layered, Layer, Registry};

/// A boxed tracing [Layer].
pub type DynLayer<S> = dyn Layer<S> + Send + Sync;
pub type BoxLayer<DynLayer> = Box<DynLayer>;

pub struct Logging {
    pub level: String,
    pub srv_name: String,
    pub with_jaeger: bool,
}

impl Logging {
    pub fn new(level: String, srv_name: String, with_jaeger: bool) -> Self {
        Self {
            level,
            srv_name,
            with_jaeger,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn init_jaeger(&self) -> OpenTelemetryLayer<Layered<EnvFilter, Registry>, Tracer> {
        let srv_name = self.srv_name.clone();
        // Start a new Jaeger trace pipeline.
        // Spans are exported in batch - recommended setup for a production application.
        global::set_text_map_propagator(TraceContextPropagator::new());
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(srv_name)
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Failed to install OpenTelemetry tracer.");

        // Create a `tracing` layer using the Jaeger tracer
        let telemetry_layer: OpenTelemetryLayer<Layered<EnvFilter, Registry>, Tracer> =
            tracing_opentelemetry::layer().with_tracer(tracer);
        telemetry_layer
    }

    pub fn init(&self) {
        let srv_name = self.srv_name.clone();
        let level = self.level.clone();

        let mut layers = Vec::new();

        let env_filter = EnvFilter::new(level);
        // layers.push(env_filter.boxed());
        // Create a `tracing` layer to emit spans as structured logs to stdout
        // let std_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
        let std_layer = fmt::layer().with_writer(std::io::stderr);
        layers.push(std_layer.boxed());
        // Combined them all together in a `tracing` subscriber

        if self.with_jaeger {
            global::set_text_map_propagator(TraceContextPropagator::new());
            let tracer: opentelemetry::sdk::trace::Tracer =
                opentelemetry_jaeger::new_agent_pipeline()
                    .with_service_name(srv_name)
                    .install_batch(opentelemetry::runtime::Tokio)
                    .expect("Failed to install OpenTelemetry tracer.");
            // Create a `tracing` layer using the Jaeger tracer
            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            layers.push(telemetry_layer.boxed());
        }

        let layer = tracing_subscriber::fmt::layer()
            // .with_filter(env_filter)
            .with_writer(std::io::stderr)
            .boxed();
        layers.push(layer);

        // // registry = registry.with(env_filter);
        // // Create a `tracing` layer to emit spans as structured logs to stdout
        // // let std_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
        // let std_layer = fmt::layer().with_writer(std::io::stderr);

        // // let mut layers = fmt::layer().with(env_filter);
        // // let mut layers = fmt::layer().with_writer(std::io::stderr);
        // // layers.and_then(layer)
        tracing_subscriber::registry()
            .with(env_filter)
            .with(layers)
            .with(JsonStorageLayer)
            .try_init()
            .expect("Failed to install `tracing` subscriber.");
    }
}

/// Initializes a new [Subscriber].
pub fn init_logging(srv_name: String, _level: String) -> WorkerGuard {
    let srv_name = std::env::var("APP_NAME").unwrap_or(srv_name.to_string());
    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production application.
    // global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(srv_name.clone())
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to install OpenTelemetry tracer.");
    // Create a `tracing` layer using the Jaeger tracer
    let telemetry_layer: OpenTelemetryLayer<Layered<EnvFilter, Registry>, Tracer> =
        tracing_opentelemetry::layer().with_tracer(tracer);

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let file_appended = tracing_appender::rolling::daily("./logs".to_string(), "api".to_string());
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appended);

    // Create a `tracing` layer to emit spans as structured logs to file system
    let file_layer = BunyanFormattingLayer::new(srv_name.into(), non_blocking);

    // Create a `tracing` layer to emit spans as structured logs to stdout
    let std_layer = fmt::layer().with_writer(std::io::stderr);

    // Combined them all together in a `tracing` subscriber
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(telemetry_layer)
        .with(file_layer)
        .with(JsonStorageLayer)
        .with(std_layer);

    let _ = subscriber.init();
    // .expect("Failed to install `tracing` subscriber.");
    guard
}

#[cfg(test)]
mod test {

    use super::init_logging;
    use tracing::{debug, error, info, warn};
    #[tokio::main]
    #[test]
    async fn test_init_logging() {
        let guard = init_logging("App".to_string(), "debug".to_string());
        debug!(target: "logging", "debug something...");
        info!(target: "logging", "info something...");
        warn!(target: "logging", "warn something...");
        error!(target: "logging", "error something...");
        drop(guard)
    }
}
