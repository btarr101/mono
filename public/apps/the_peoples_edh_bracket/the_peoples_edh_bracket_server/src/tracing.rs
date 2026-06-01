use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn setup_tracing(stage: &str) -> tracing::span::EnteredSpan {
    let subscriber = tracing_subscriber::fmt().with_env_filter(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );

    if cfg!(debug_assertions) {
        subscriber.compact().init();
    } else {
        subscriber.json().init();
    }

    tracing::info_span!("root", stage = stage).entered()
}
