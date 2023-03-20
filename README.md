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
