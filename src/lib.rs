pub mod block;
pub mod chain;
pub mod ledger;
pub mod tx;

pub fn tracing_init(level: tracing::Level, color: bool) -> anyhow::Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

    let layer_stderr = fmt::Layer::new()
        .with_writer(std::io::stderr)
        .with_ansi(color)
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_filter(EnvFilter::from_default_env().add_directive(level.into()));
    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(layer_stderr))?;
    Ok(())
}
