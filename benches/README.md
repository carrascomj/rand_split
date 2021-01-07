# Benchmarks
```
Split random slice n=10e7, 5 partitions
                        time:   [47.599 ms 47.766 ms 47.971 ms]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

Benchmarking Split random iterator n=10e5, 3 partitions: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.0s, enable flat sampling, or reduce sample count to 60.
Split random iterator n=10e5, 3 partitions
                        time:   [1.1922 ms 1.1931 ms 1.1940 ms]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe
```

## Run the benchmark yourself
```
cargo bench
```

## Plots and statistics
Criterion generates an informative HTML report after running `cargo bench` at
`target/criterion/report`.
