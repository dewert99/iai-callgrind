use std::hint::black_box;

use benchmark_tests::bubble_sort;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};

fn setup_worst_case_array(start: i32) -> Vec<i32> {
    if start.is_negative() {
        (start..0).rev().collect()
    } else {
        (0..start).rev().collect()
    }
}

#[library_benchmark]
#[bench::worst_case(setup_worst_case_array(20))]
fn bench_bubble_sort(array: Vec<i32>) -> Vec<i32> {
    black_box(bubble_sort(array))
}

library_benchmark_group!(
    name = bench_group;
    benchmarks = bench_bubble_sort
);

main!(library_benchmark_groups = bench_group);
