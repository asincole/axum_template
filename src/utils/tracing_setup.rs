#[cfg(debug_assertions)]
pub fn setup_subscriber() -> impl tracing::Subscriber {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt};

    let level_filter_layer = filter::LevelFilter::INFO;
    let (filter, _) = tracing_subscriber::reload::Layer::new(level_filter_layer);

    tracing_subscriber::registry().with(filter).with(
        fmt::layer()
            .pretty()
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true),
    )
}

#[cfg(not(debug_assertions))]
pub fn setup_subscriber() -> impl tracing::Subscriber {
    use tracing_bunyan_formatter::BunyanFormattingLayer;
    use tracing_subscriber::{filter, layer::SubscriberExt};

    let level_filter_layer = filter::LevelFilter::INFO;
    let (filter, _) = tracing_subscriber::reload::Layer::new(level_filter_layer);

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            "axum_template".into(),
            std::io::stdout,
        ))
}
