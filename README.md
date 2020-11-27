Rocket API example
=================
## Requirements
* Rust (nightly)
* Postgres
## Usage
# Install diesel-cli
```
cargo install diesel_cli --no-default-features --features postgres
```
if you're in windows use rust msvc toolchain and be sure that postgres bin in your path variable

# Run
```
diesel migration run
cargo run --package rocket-api --bin rocket-api
```