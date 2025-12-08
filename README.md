# Advent of Code 2025 Solutions in Rust

My solutions for [AoC 2025](https://adventofcode.com/2025) in Rust.

I created this project so that I could focus on solving the problem with less effort to other than problem solving.

## usage

```sh
# run
cargo run [--release] -- day1_p1

# benchmark
cargo run [--release] --bin bench [-- day1_p1 [day1_p2 ...]]
```

## How to add solutions

All solutions are named in {name}\_{part} format.

Solutions with same name share one input file.

You might want to add

- `src/solutions/{name}_{part}.rs` which includes following function
  ```rust
  pub fn solution(lines: Vec<&str>) -> String {
      ...
  }
  ```
- `resource/input/{name}.txt`
- solution name in `src/solutions/mod.rs` with comma seperated
  ```rust
  export_solutions!(
      day1_p1, day1_p2, ...
  );
  ```

## Benchmark Results (Intel N100)

Result of 100 iterations

| Solution Name |   Average |       Min |       Max |
| ------------- | --------: | --------: | --------: |
| day1_p1       |  0.053 ms |  0.045 ms |  0.084 ms |
| day1_p2       |  0.043 ms |  0.039 ms |  0.056 ms |
| day2_p1       |  0.007 ms |  0.007 ms |  0.013 ms |
| day2_p2       |  0.073 ms |  0.067 ms |  0.084 ms |
| day3_p1       |  0.029 ms |  0.028 ms |  0.038 ms |
| day3_p2       |  0.142 ms |  0.138 ms |  0.164 ms |
| day4_p1       |  0.498 ms |  0.493 ms |  0.550 ms |
| day4_p2       | 11.302 ms | 11.119 ms | 12.602 ms |
| day5_p1       |  0.048 ms |  0.045 ms |  0.074 ms |
| day5_p2       |  0.017 ms |  0.016 ms |  0.026 ms |
| day6_p1       |  0.102 ms |  0.099 ms |  0.118 ms |
| day6_p2       |  0.106 ms |  0.104 ms |  0.121 ms |
| day7_p1       |  0.025 ms |  0.024 ms |  0.033 ms |
| day7_p2       |  0.030 ms |  0.028 ms |  0.037 ms |
| day8_p1       | 30.595 ms | 26.579 ms | 35.494 ms |
| day8_p2       | 30.889 ms | 28.365 ms | 32.639 ms |

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
