//! A convenient tracing config and init lib, with symlinking and local timezone.

use clia_time::UtcOffset as CliaUtcOffset;
use time::macros::format_description;
use time::UtcOffset;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;

const FORMAT_PRETTY: &'static str = "pretty";
const FORMAT_COMPACT: &'static str = "compact";
const FORMAT_JSON: &'static str = "json";
const FORMAT_FULL: &'static str = "full";

/// The tracing configuration properties.
#[derive(Debug, Clone)]
pub struct TracingConfig {
    filter_level: String,
    with_ansi: bool,
    to_stdout: bool,
    directory: String,
    file_name: String,
    rolling: tracing_appender::rolling::Rotation,
    format: String,
    with_level: bool,
    with_target: bool,
    with_thread_ids: bool,
    with_thread_names: bool,
    with_source_location: bool,
}

impl Default for TracingConfig {
    fn default() -> Self {
        TracingConfig {
            filter_level: "info".to_owned(),
            with_ansi: true,
            to_stdout: false,
            directory: "./logs".to_owned(),
            file_name: "my-service.log".to_owned(),
            rolling: tracing_appender::rolling::Rotation::DAILY,
            format: FORMAT_PRETTY.to_owned(),
            with_level: true,
            with_target: true,
            with_thread_ids: true,
            with_thread_names: true,
            with_source_location: true,
        }
    }
}

impl TracingConfig {
    /// Will try_from_default_env while not setted.
    ///
    /// You can use value like "info", or something like "mycrate=trace".
    /// 
    /// Default value if "info".
    ///
    pub fn filter_level(mut self, filter_level: &str) -> Self {
        self.filter_level = filter_level.to_owned();
        self
    }

    /// Show ANSI color symbols.
    pub fn with_ansi(mut self, with_ansi: bool) -> Self {
        self.with_ansi = with_ansi;
        self
    }

    /// Will append log to stdout.
    pub fn to_stdout(mut self, to_stdout: bool) -> Self {
        self.to_stdout = to_stdout;
        self
    }

    /// Set log file directory.
    pub fn directory(mut self, directory: &str) -> Self {
        self.directory = directory.to_owned();
        self
    }

    /// Set log file name.
    pub fn file_name(mut self, file_name: &str) -> Self {
        self.file_name = file_name.to_owned();
        self
    }

    /// Valid values: minutely | hourly | daily | never
    ///
    /// Will panic on other values.
    pub fn rolling(mut self, rolling: &str) -> Self {
        if rolling == "minutely" {
            self.rolling = tracing_appender::rolling::Rotation::MINUTELY;
        } else if rolling == "hourly" {
            self.rolling = tracing_appender::rolling::Rotation::HOURLY;
        } else if rolling == "daily" {
            self.rolling = tracing_appender::rolling::Rotation::DAILY;
        } else if rolling == "never" {
            self.rolling = tracing_appender::rolling::Rotation::NEVER;
        } else {
            panic!("Unknown rolling")
        }
        self
    }

    /// Valid values: pretty | compact | json | full
    ///
    /// Will panic on other values.
    pub fn format(mut self, format: &str) -> Self {
        if format != FORMAT_PRETTY
            && format != FORMAT_COMPACT
            && format != FORMAT_JSON
            && format != FORMAT_FULL
        {
            panic!("Unknown format")
        }
        self.format = format.to_owned();
        self
    }

    /// include levels in formatted output
    pub fn with_level(mut self, with_level: bool) -> Self {
        self.with_level = with_level;
        self
    }

    /// include targets
    pub fn with_target(mut self, with_target: bool) -> Self {
        self.with_target = with_target;
        self
    }

    /// include the thread ID of the current thread
    pub fn with_thread_ids(mut self, with_thread_ids: bool) -> Self {
        self.with_thread_ids = with_thread_ids;
        self
    }

    /// include the name of the current thread
    pub fn with_thread_names(mut self, with_thread_names: bool) -> Self {
        self.with_thread_names = with_thread_names;
        self
    }

    /// include source location
    pub fn with_source_location(mut self, with_source_location: bool) -> Self {
        self.with_source_location = with_source_location;
        self
    }

    /// Init tracing log.
    ///
    /// Caller should hold the guard.
    pub fn init(self) -> WorkerGuard {
        // Tracing appender init.
        let file_appender = match self.rolling {
            tracing_appender::rolling::Rotation::MINUTELY => {
                tracing_appender::rolling::minutely(self.directory, self.file_name)
            }
            tracing_appender::rolling::Rotation::HOURLY => {
                tracing_appender::rolling::hourly(self.directory, self.file_name)
            }
            tracing_appender::rolling::Rotation::DAILY => {
                tracing_appender::rolling::daily(self.directory, self.file_name)
            }
            tracing_appender::rolling::Rotation::NEVER => {
                tracing_appender::rolling::never(self.directory, self.file_name)
            }
        };
        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

        // Local offset timezone init, and set time format.
        let offset_sec = CliaUtcOffset::current_local_offset()
            .expect("Can not get local offset!")
            .whole_seconds();
        let offset =
            UtcOffset::from_whole_seconds(offset_sec).expect("Can not from whole seconds!");
        let timer = OffsetTime::new(
            offset,
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        );

        // Tracing subscriber init.
        let s = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
                    tracing_subscriber::EnvFilter::new(&format!("{}", self.filter_level)),
                ),
            )
            .with_timer(timer)
            .with_ansi(self.with_ansi);

        let s = if self.to_stdout {
            s.with_writer(std::io::stdout).with_writer(file_writer)
        } else {
            s.with_writer(file_writer)
        };

        // Format switch.
        if self.format == FORMAT_PRETTY {
            s.event_format(
                fmt::format()
                    .pretty()
                    .with_level(self.with_level)
                    .with_target(self.with_target)
                    .with_thread_ids(self.with_thread_ids)
                    .with_thread_names(self.with_thread_names)
                    .with_source_location(self.with_source_location),
            )
            .init();
        } else if self.format == FORMAT_COMPACT {
            s.event_format(
                fmt::format()
                    .compact()
                    .with_level(self.with_level)
                    .with_target(self.with_target)
                    .with_thread_ids(self.with_thread_ids)
                    .with_thread_names(self.with_thread_names)
                    .with_source_location(self.with_source_location),
            )
            .init();
        } else if self.format == FORMAT_JSON {
            s.event_format(
                fmt::format()
                    .json()
                    .with_level(self.with_level)
                    .with_target(self.with_target)
                    .with_thread_ids(self.with_thread_ids)
                    .with_thread_names(self.with_thread_names)
                    .with_source_location(self.with_source_location),
            )
            .init();
        } else if self.format == FORMAT_FULL {
            s.event_format(
                fmt::format()
                    .with_level(self.with_level)
                    .with_target(self.with_target)
                    .with_thread_ids(self.with_thread_ids)
                    .with_thread_names(self.with_thread_names)
                    .with_source_location(self.with_source_location),
            )
            .init();
        }

        // Caller should hold this handler.
        guard
    }
}

/// Build a default config.
pub fn build() -> TracingConfig {
    TracingConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _guard = build()
            .filter_level("info")
            .with_ansi(true)
            .to_stdout(false)
            .directory("./logs")
            .file_name("my-service.log")
            .rolling("daily")
            .init();

        tracing::info!("logged by tracing");
    }
}
