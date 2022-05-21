# rs-json
JSON parsing library for Rust

## Useful Commands
#### Cargo
`cargo new <cargo_name>`
`cargo build`
`cargo run`
`cargo check`

#### Creating a library (rs_json) and an executable (main) that uses library and run executable (main)
`rustc --crate-type=lib --crate-name=rs_json src/lib.rs && rustc main.rs --extern rs_json=librs_json.rlib --edition=2021 && ./main`


[Rust Lib Tutorial](https://doc.rust-lang.org/rust-by-example/crates/lib.html)
