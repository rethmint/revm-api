use std::{env, path::PathBuf, sync::Once};

use chrono::Utc;
use chrono_tz::Asia::Seoul;
use once_cell::sync::OnceCell;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

static TRACER_INIT: Once = Once::new();
static TRACER_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

fn log_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let timestamp = Utc::now()
        .with_timezone(&Seoul)
        .format("%Y-%m-%d-%H-%M-%S %z")
        .to_string();

    let file = format!("{}.log", timestamp);

    PathBuf::from(home_dir)
        .join(".rethmint")
        .join("log")
        .join(file)
}

pub fn init_tracer() {
    TRACER_INIT.call_once(|| {
        let file = rolling::never(".", log_path());
        let (non_blocking, guard) = tracing_appender::non_blocking(file);

        TRACER_GUARD.set(guard).ok();

        let env_filter = EnvFilter::new("info");

        let format = fmt::format()
            .with_level(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .compact();

        let subscriber = Registry::default()
            .with(env_filter)
            // async dump to log
            .with(
                fmt::layer()
                    .event_format(format)
                    .with_ansi(false)
                    .with_writer(non_blocking)
                    .with_target(false),
            )
            // console
            .with(fmt::layer().with_writer(std::io::stdout));

        tracing::debug!("Test log - debug");
        tracing::error!("Test log - error");
        tracing::info!("Test log - info");

        tracing::subscriber::set_global_default(subscriber).unwrap();
    });
}
