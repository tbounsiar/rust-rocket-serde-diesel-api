Rust API example
=================
## Intro
This is an example of GRUD Json api using rust lang and rocket, diesel orm and serde json libraries.
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
cargo build
diesel migration run
cargo run --package rocket-api --bin rocket-api
```