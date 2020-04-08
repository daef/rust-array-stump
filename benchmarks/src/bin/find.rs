extern crate array_stump_benchmarks;

use array_stump_benchmarks::benchmarks::{BenchmarkMode, GeneratorMode, run_benchmarks};

fn main() {
    run_benchmarks(
        BenchmarkMode::Find{ measure_every: 100},
        GeneratorMode::Avg,
        1000000,
        3,
        false,
    );
}