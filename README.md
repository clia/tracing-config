# clia-tracing-config

A personal tracing config initialize lib, with symlinking and local offset timezone.

Use these formats:

- pretty()
- with_level(true)
- with_target(true)
- with_thread_ids(true)
- with_thread_names(true)
- with_source_location(true)

## Usage

Code example and default values:

```rust
let _guard = clia_tracing_config::build()
    .with_level("info")
    .with_ansi(true)
    .to_stdout(false)
    .directory("./logs")
    .file_name("my-service.log")
    .rolling("daily")
    .init();

tracing::info!("logged by tracing");
log::info!("logged by tracing");
```

`rolling` supports:

- minutely
- hourly
- daily
- never
