#![feature(let_chains)]

use ndarray::Array2;
use std::io::{stdin, BufRead};

fn main() {
    let mut words: Vec<&str> = include_str!("../resources/words.txt").split("\n").collect();
    words.sort();

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

    let testwords = ["mint", "mite"];

    let result = eval_grid((h, w), &grid, &words[..]);
    println!("{:?}", result)
}

fn eval_grid<'a>(dims: (usize, usize), grid: &Array2<u8>, words: &[&'a str]) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    let mut stack: Vec<(u8, Vec<(usize, usize)>)> = Vec::with_capacity(dims.0 * dims.1);

    'wl: for word in words {
        // First, pop what isn't needed
        while let Some((last, _)) = stack.last() {
            if word.len() < stack.len() && *last == word.as_bytes()[stack.len() - 1] {
                break;
            }
            stack.pop().unwrap();
        }

        if let Some((_, last)) = stack.last() && last.is_empty() { continue; }

        // Then, push what is needed
        for b in &word.as_bytes()[stack.len()..] {
            let next: Vec<(usize, usize)> = if let Some((_, last)) = stack.last() {
                last.iter().map(|p| neighbours(dims, *p)).flatten().filter(|p| grid[*p] == *b).collect()
            } else {
                all_cells(dims).filter(|p| grid[*p] == *b).collect()
            };
            let should_stop = next.is_empty();
            stack.push((*b, next));
            if should_stop {
                continue 'wl;
            }
        }

        //We finished the word!
        result.push(word)
    }


    result
}

fn all_cells(dims: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (0..dims.0).map(move |p0| {
        (0..dims.1).map(move |p1| (p0, p1))
    }).flatten()
}

fn neighbours(dims: (usize, usize), pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        (pos.0.wrapping_sub(1), pos.1.wrapping_sub(1)), // - -
        (pos.0, pos.1.wrapping_sub(1)),                     // . -
        (pos.0 + 1, pos.1.wrapping_sub(1)),                 // + -
        (pos.0 + 1, pos.1),                                     // + .
        (pos.0 + 1, pos.1 + 1),                                 // + +
        (pos.0, pos.1 + 1),                                     // . +
        (pos.0.wrapping_sub(1), pos.1 + 1),                 // - +
        (pos.0.wrapping_sub(1), pos.1),                     // - .
    ].into_iter().filter(move |(p0, p1)| *p0 < dims.0 && *p1 < dims.1)
}
