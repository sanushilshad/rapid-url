
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, EnvFilter, Layer, Registry,
};
use opentelemetry::trace::TracerProvider as _;
pub fn get_subscriber<Sink>(
    _name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let format_layer = fmt::Layer::default()
        .with_ansi(true)
        .with_writer(sink)
        .with_filter(LevelFilter::INFO);
    Registry::default().with(env_filter).with(format_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}



pub fn get_subscriber_with_jeager<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let tracer: opentelemetry_sdk::trace::Tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
    
        .expect("Couldn't create OTLP tracer").tracer(name);
    let telemetry_layer: tracing_opentelemetry::OpenTelemetryLayer<
        Registry,
        opentelemetry_sdk::trace::Tracer,
    > = tracing_opentelemetry::layer().with_tracer(tracer);
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let format_layer = fmt::Layer::default()
        .with_ansi(true)
        .with_writer(sink)
        .with_filter(LevelFilter::INFO);
    Registry::default()
        .with(telemetry_layer)
        .with(env_filter)
        .with(format_layer)
}
