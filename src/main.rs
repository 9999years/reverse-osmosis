use std::path::Path;

use eyre::Context;
use ignore::WalkBuilder;
use structopt::StructOpt;

fn main() -> eyre::Result<()> {
    let args = Opt::from_args();
    install_tracing(&args.tracing_filter);
    color_eyre::install()?;

    for result in WalkBuilder::new("./").hidden(false).build() {
        match result {
            Ok(entry) => {
                if let Some(err) = entry.error() {
                    tracing::error!("{err}");
                }
                let path = entry.path();
                let metadata = match entry.metadata() {
                    Ok(metadata) => metadata,
                    Err(err) => {
                        tracing::error!("Failed to access metadata for {path:?}: {err}");
                        continue;
                    }
                };
                if metadata.is_dir() {
                    tracing::trace!(?path, "Directory");
                } else {
                    tracing::debug!(?path, "Processing file");
                    match do_file(path) {
                        Ok(()) => {}
                        err @ Err(_) => {
                            let err = err.wrap_err(format!("Failed to process {path:?}"));
                            tracing::error!("{err:?}");
                        }
                    };
                }
            }
            Err(err) => {
                tracing::error!("{err}");
            }
        }
    }

    Ok(())
}

fn do_file(path: &Path) -> eyre::Result<()> {
    let original =
        std::fs::read_to_string(path).wrap_err_with(|| format!("Failed to read {path:?}"))?;

    for line in original.lines() {
        // todo
    }

    Ok(())
}

fn install_tracing(filter_directives: &str) {
    use tracing_subscriber::fmt;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .without_time();
    let filter_layer = EnvFilter::try_new(filter_directives)
        .or_else(|_| EnvFilter::try_from_default_env())
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Tracing filter.
    ///
    /// Can be any of "error", "warn", "info", "debug", or
    /// "trace". Supports more granular filtering, as well; see documentation for
    /// [`tracing_subscriber::EnvFilter`][EnvFilter].
    ///
    /// [EnvFilter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/struct.EnvFilter.html
    #[structopt(long, default_value = "info")]
    tracing_filter: String,
}
