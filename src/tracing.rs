use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

use opentelemetry_sdk::trace::Tracer;
use tracing_opentelemetry::OpenTelemetryLayer;

use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

pub fn setup(level: Level) {
    let subscriber = registry()
        .with(
            OpenTelemetryLayer::new(init_tracer())
                .with_error_records_to_exceptions(true)
                .with_filter(LevelFilter::from_level(level)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_filter(LevelFilter::from_level(level)),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Could not setup tracing/logging");
}

// Construct Tracer for OpenTelemetryLayer
fn init_tracer() -> Tracer {
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_otlp::TonicExporterBuilder;
    use opentelemetry_sdk::trace::TracerProvider;
    let exporter = TonicExporterBuilder::default()
        .build_span_exporter()
        .expect("Init");
    let provider = TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build();
    let tracer = provider.tracer("service_conventions");

    opentelemetry::global::set_tracer_provider(provider);
    return tracer;
}
