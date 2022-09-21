# clia-tracing-config

A personal tracing config initialize lib, with symlinking and local offset timezone.

<img width="530" alt="image" src="https://user-images.githubusercontent.com/1589842/191416973-d6cb8a6a-7d46-4221-b794-6a1ab418da34.png">

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
