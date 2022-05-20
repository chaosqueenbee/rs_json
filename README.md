# rs-json
JSON parsing library for Rust

## Useful Commands
#### Cargo
`cargo new <cargo_name>`
`cargo build`
`cargo run`
`cargo check`

#### Creating a library
`rustc --crate-type=lib --crate-name=<lib_name> src/lib.rs`

#### Creating an executable that uses a library
`rustc main.rs --extern rs_json=lib<lib_name>.rlib --edition=2021 && ./main`


[Rust Lib Tutorial](https://doc.rust-lang.org/rust-by-example/crates/lib.html)
