use std::io::{BufRead, stdin};
use ndarray::Array2;
use rand::Rng;

pub fn read_input() -> Array2<u8> {
    let stdin = stdin();
    let lock = stdin.lock();
    let mut input = lock.lines();

    println!("> Width:");
    let w: usize = input.next().unwrap().unwrap().parse().unwrap();
    println!("> Height:");
    let h: usize = input.next().unwrap().unwrap().parse().unwrap();

    println!("> Graph:");
    let mut grid = Array2::from_elem((h, w), b'_');
    for i in 0..h {
        for (j, c) in input.next().unwrap().unwrap().bytes().enumerate() {
            grid[(i, j)] = c;
        }
    }
    grid
}

pub fn random_grid(dims: (usize, usize)) -> Array2<u8> {
    let mut rng = rand::thread_rng();

    let mut grid = Array2::from_elem(dims, 0);
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            grid[(i, j)] = rng.gen_range(b'a'..=b'z');
        }
    }
    grid
}