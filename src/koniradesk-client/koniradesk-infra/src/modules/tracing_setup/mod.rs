use std::error::Error;

pub fn init_tracing()->Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
    .with_writer(std::io::stderr)
    .with_max_level(tracing::Level::INFO)
    .init();
    Ok(())
}