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

| Solution Name |  Average |      Min |      Max |
| ------------- | -------: | -------: | -------: |
| day1_p1       | 0.053 ms | 0.045 ms | 0.084 ms |
| day1_p2       | 0.043 ms | 0.039 ms | 0.056 ms |
| day2_p1       | 0.007 ms | 0.007 ms | 0.013 ms |
| day2_p2       | 0.073 ms | 0.067 ms | 0.084 ms |

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
