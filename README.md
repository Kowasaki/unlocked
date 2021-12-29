# unlocked
rust to python modules based on PyO3

## Runtimes based on WSL2 (Ubuntu) from laptop

46 ~ 100MB files to S3

ray: 263.604 seconds

threads: 258.387 seconds

processes: 250.194 seconds

rust async: 227 seconds


100 ~ 10MB files to S3

ray: 119.540 seconds

threads: 52.476 seconds

processes: 51.446 seconds

rust async: 55 seconds


## Compiling

For development use `maturin develop`

## Testing in Rust

Use `cargo test --no-default-features`