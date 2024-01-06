# clia-tracing-config

A convenient tracing config and init lib, with symlinking and local timezone.

<img width="543" alt="image" src="https://user-images.githubusercontent.com/1589842/191417363-71f134a3-e23b-4e95-a0c7-22237799af8a.png">

Use these formats default, and can be configured:

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
    .filter_level("info")
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

## Changelog

- 0.2.7: Fix json fmt error. (2024-1-6)
- 0.2.6: Make pub use WorkerGuard. (2023-12-17)
- 0.2.5: Fix timer problem (no effect). (2022-10-25)
- 0.2.4: Fix to_stdout impl. (2022-10-22)
- 0.2.3: Change clia-time to clia-local-offset. (2022-10-22)
- 0.2.2: Make level support expr. (2022-10-22)
- 0.2.1: Add Debug & Clone. (2022-10-11)
- 0.2.0: Refacted impl. (2022-10-2)
- 0.1.0: Initial release. (2022-9-21)
