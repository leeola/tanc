use anyhow::Result;
use clap::{ArgAction, Parser};
use std::{fs::File, path::PathBuf};
use tanc_core::server::Backend;
use tracing::{metadata::LevelFilter, subscriber};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, EnvFilter};

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// If defined, log to a file in addition to stderr.
    #[arg(long)]
    pub log_file: Option<PathBuf>,
    #[arg(long)]
    pub trunc_log: bool,
    #[arg(long)]
    pub dont_log_stderr: bool,

    /// Verbosity of the logs. Opposite of [`Self::quiet`]. Default is `warn`, so `-v` is `info`,
    /// `-vv` is `debug`, etc.
    ///
    /// Does nothing if both [`Self::log_file`] is `None` and [`Self::dont_log_stderr`] is true.
    #[arg(
        long,
        short = 'v',
        action = ArgAction::Count,
        global = true,
    )]
    pub verbose: u8,

    /// Reduce verbosity of the logs. Opposite of [`Self::verbose`]. Default is `warn`, so `-q` is
    /// error, `-qq` suppresses all logs.
    ///
    /// Does nothing if both [`Self::log_file`] is `None` and [`Self::dont_log_stdout`] is true.
    #[arg(
        long,
        short = 'q',
        action = ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
    pub quiet: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = CliConfig::parse();

    let (log_file, _log_file_guard) =
        config
            .log_file
            .as_ref()
            .map_or(Ok((None, None)), |path| -> Result<_> {
                let f = File::options()
                    .create(true)
                    .write(true)
                    // NIT: Not sure if this inversion is required, but the behavior seemed odd
                    // without this.
                    .append(!config.trunc_log)
                    .truncate(config.trunc_log)
                    .open(path)?;
                let (non_blocking, guard) = tracing_appender::non_blocking(f);
                let w = Layer::new().with_writer(non_blocking);
                Ok((Some(w), Some(guard)))
            })?;
    let (log_stderr, _log_stderr_guard) = if !config.dont_log_stderr {
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stderr());
        let stderr = Layer::new().with_writer(non_blocking);
        (Some(stderr), Some(guard))
    } else {
        (None, None)
    };

    let env_filter = EnvFilter::builder()
        .with_default_directive(
            match config.verbose as i8 - config.quiet as i8 {
                i if i <= -2 => LevelFilter::OFF,
                -1 => LevelFilter::ERROR,
                0 => LevelFilter::WARN,
                1 => LevelFilter::INFO,
                2 => LevelFilter::DEBUG,
                3 | _ => LevelFilter::TRACE,
            }
            .into(),
        )
        .with_env_var("RUST_LOG")
        .from_env_lossy();

    subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(env_filter)
            .with(log_stderr)
            .with(log_file),
    )
    .unwrap();

    tracing::info!("main");
    Backend::new().await;
    Ok(())
}
