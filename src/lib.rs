use std::str::FromStr;

use clia_time::UtcOffset as CliaUtcOffset;
use time::macros::format_description;
use time::UtcOffset;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;

pub struct TracingConfig {
    level: tracing::Level,
    ansi: bool,
    stdout: bool,
    directory: String,
    file_name: String,
    rolling: tracing_appender::rolling::Rotation,
}

impl Default for TracingConfig {
    fn default() -> Self {
        TracingConfig {
            level: tracing::Level::INFO,
            ansi: true,
            stdout: false,
            directory: "./logs".to_owned(),
            file_name: "my-service.log".to_owned(),
            rolling: tracing_appender::rolling::Rotation::DAILY,
        }
    }
}

impl TracingConfig {
    pub fn with_level(mut self, level: &str) -> Self {
        self.level = tracing::Level::from_str(level).unwrap();
        self
    }

    pub fn with_ansi(mut self, ansi: bool) -> Self {
        self.ansi = ansi;
        self
    }

    pub fn to_stdout(mut self, stdout: bool) -> Self {
        self.stdout = stdout;
        self
    }

    pub fn directory(mut self, directory: &str) -> Self {
        self.directory = directory.to_owned();
        self
    }

    pub fn file_name(mut self, file_name: &str) -> Self {
        self.file_name = file_name.to_owned();
        self
    }

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

    pub fn init(self) -> WorkerGuard {
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

        // Configure a custom event formatter
        let format = fmt::format()
            .pretty()
            .with_level(true) // don't include levels in formatted output
            .with_target(true) // don't include targets
            .with_thread_ids(true) // include the thread ID of the current thread
            .with_thread_names(true) // include the name of the current thread
            .with_source_location(true);

        // let mut fmt = tracing_subscriber::fmt()
        //     .event_format(format)
        //     .with_env_filter(
        //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
        //             tracing_subscriber::EnvFilter::new(&format!("{}", self.level)),
        //         ),
        //     )
        //     .with_timer(timer);

        // if self.stdout {
        //     fmt = fmt.with_writer(std::io::stdout);
        // } else {
        //     fmt = fmt.with_writer(file_writer);
        // }

        // if self.ansi {
        //     fmt = fmt.with_ansi(true);
        // }

        // fmt.init();

        if self.stdout {
            tracing_subscriber::fmt()
                .event_format(format)
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
                        tracing_subscriber::EnvFilter::new(&format!("{}", self.level)),
                    ),
                )
                .with_timer(timer)
                .with_writer(std::io::stdout)
                .with_ansi(self.ansi)
                .init();
        } else {
            tracing_subscriber::fmt()
                .event_format(format)
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
                        tracing_subscriber::EnvFilter::new(&format!("{}", self.level)),
                    ),
                )
                .with_timer(timer)
                .with_writer(file_writer)
                .with_ansi(self.ansi)
                .init();
        }

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
            .with_level("info")
            .with_ansi(true)
            .to_stdout(false)
            .directory("./logs")
            .file_name("my-service.log")
            .rolling("daily")
            .init();

        tracing::info!("logged by tracing");
    }
}
