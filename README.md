# Jaro-Winkler distance

A Rust implementation of Jaro-Winkler distance.

## Quick start

### How to install

Write following code into `Cargo.toml`.

```toml
[dependencies]
jaro_winkler_distance = { git = "https://github.com/r4ai/jaro_winkler_distance.rs", branch = "main" }
```

### How to use

```rs
use jaro_winkler_distance::{jaro_winkler_distance, PrefixLength};

fn main() {
    let s1 = "MARTHA";
    let s2 = "MARHTA";
    let distance = jaro_winkler_distance(s1, s2, &PrefixLength::Four);
    assert_eq!(distance, 0.9611111111111111);
}
```

## Development

```bash
# Run command to get jaro-winkler distance between two strings
$ cargo run --bin jarowinkler "string1" "string2"

# Test
$ cargo test

# Build
$ cargo build --release

# Benchmark (results are in target/criterion/report/jaro_winkler_distance/report/index.html)
$ cargo bench
```
