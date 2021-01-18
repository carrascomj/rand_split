# Benchmarks

```
Split random slice n=10e7, 5 partitions
                        time:   [45.375 ms 45.465 ms 45.560 ms]
                        change: [+0.1280% +0.4098% +0.6871%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking Split random iterator n=10e7 (generalized), 3 partitions: Warming up for 3.0000 s
Split random iterator n=10e7 (generalized), 3 partitions
                        time:   [78.550 ms 78.593 ms 78.636 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking Split random iterator n=10e7, 3 partitions: Warming up for 3.0000 s
Split random iterator n=10e7, 3 partitions
                        time:   [54.958 ms 55.019 ms 55.080 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

## Run the benchmark yourself
```
cargo bench
```

## Plots and statistics
Criterion generates an informative HTML report after running `cargo bench` at
`target/criterion/report`.
