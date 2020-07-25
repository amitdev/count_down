# Solution to countdown problem in Rust
This is a solution to the countdown problem presented in this [paper](http://www.cs.nott.ac.uk/~pszgmh/countdown.pdf) in Rust.

## How to run
```
$ cargo run
```

## Benchmark
Benchmarks with the [criterion](https://github.com/bheisler/criterion.rs) library.

```
# Serial Version
$ cargo bench
     Running target/release/deps/count_down_bench-05c9b5e9abb05e85
CountDown               time:   [368.36 ms 370.71 ms 373.22 ms]

# Parallel version
$ cargo bench
     Running target/release/deps/count_down_bench-3a06313c685b4b6b
CountDown               time:   [69.206 ms 71.958 ms 74.038 ms]
```
