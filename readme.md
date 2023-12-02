# Advent of Code 2023

This repo contains my solutions to this year's advent of code.

### File structure

- The solutions for each day the advent is stored in a directory uncreatively called "day" + the number in the advent (e.g. "day1" or "day24")
- The rust project solutions for each problem in each day is stored in the respective day directory, in its own directory called: "problem" + 1 or 2 (e.g. "problem1")

### How do I run this?

1. Create a plaintext (utf-8) file containing exactly the problem input (for this example I will call it `problem.input`)
2. Using the `cargo run` utility
   1. pass path of the day/problem via the `--manifest-path` flag
   2. pass the path to the problem input

```bash
cargo run --manifest-path day1/problem1/Cargo.toml problem.input
```
