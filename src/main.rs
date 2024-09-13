#![feature(test)]
extern crate test;

use std::{env::args, thread, time::Duration};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use test::Bencher;

const LENGTH: usize = 1_000_000;

fn main() {
    let what = args().nth(1);

    match what {
        None => panic!("Did not specify what to run"),
        Some(s) => match s.as_str() {
            "flat_map_with_arr" => flat_map_with_arr(),
            // "flat_map_with_iter" => flat_map_with_iter(),
            "flat_map_iter_with_arr" => flat_map_iter_with_arr(),
            "flat_map_iter_with_iter" => flat_map_iter_with_iter(),

            s => panic!("Unknown option {s}"),
        }
    }

    thread::sleep(Duration::from_secs(5));
}

fn flat_map_with_arr() {
    let _data: Vec<i32> = (0..LENGTH).into_par_iter().flat_map(|_| [0, 1, 2, 3]).collect();
}

#[test]
fn flat_map_with_iter() {
    let _data: Vec<i32> = (0..LENGTH).into_par_iter().flat_map(|_| rayon::iter::repeat(0).take(4)).collect();
}

fn flat_map_iter_with_arr() {
    let _data: Vec<i32> = (0..LENGTH).into_par_iter().flat_map_iter(|_| [0, 1, 2, 3]).collect();
}

fn flat_map_iter_with_iter() {
    let _data: Vec<i32> = (0..LENGTH).into_par_iter().flat_map_iter(|_| std::iter::repeat(0).take(4)).collect();
}

fn flat_map_huge_par_iter() {
    // This would be the worst case for the new implementation, since it would not split the inner (main) iterator instantly.
    // I would argue the case where the inner iterator is large and slow enough to warrant splitting it, while the outer is too small to split up, is rare.
    // And work stealing will still split the inner iterator when necessary (I think)
    let _data: Vec<i32> = (0..1).into_par_iter().flat_map(|_| rayon::iter::repeat(0).take(LENGTH)).collect();
}

fn flat_map_perfect_par_iter() {
    // This would be the worst case for the new implementation, since it would not split the inner (main) iterator instantly.
    // I would argue the case where the inner iterator is large and slow enough to warrant splitting it, while the outer is too small to split up, is rare.
    // And work stealing will still split the inner iterator when necessary (I think)
    let _data: Vec<i32> = (0..1).into_par_iter().flat_map(|_| (0..rayon::current_num_threads()).into_par_iter().map(|_| {
        thread::sleep(Duration::from_millis(10));
        1
    })).collect();
}

fn flat_map_perfect_perfect_par_iter() {
    let _data: Vec<i32> = (0..rayon::current_num_threads()).into_par_iter().flat_map(|_| (0..rayon::current_num_threads()).into_par_iter().map(|_| {
        thread::sleep(Duration::from_millis(10));
        1
    })).collect();
}

#[bench]
fn bench_flat_map_with_arr(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_with_arr();
    })
}

#[bench]
fn bench_flat_map_with_iter(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_with_iter();
    })
}

#[bench]
fn bench_flat_map_iter_with_arr(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_iter_with_arr();
    })
}

#[bench]
fn bench_flat_map_iter_with_iter(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_iter_with_iter();
    })
}

#[bench]
fn bench_flat_map_huge_par_iter(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_huge_par_iter();
    })
}

#[bench]
fn bench_flat_map_perfect_par_iter(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_perfect_par_iter();
    })
}

#[bench]
fn bench_flat_map_perfect_perfect_par_iter(bencher: &mut Bencher) {
    bencher.iter(|| {
        flat_map_perfect_perfect_par_iter();
    })
}
