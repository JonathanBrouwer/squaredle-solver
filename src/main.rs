#![feature(let_chains)]

use ndarray::Array2;
use std::io::{stdin, BufRead};
use std::time::Instant;

fn read_input() -> Array2<u8> {
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

fn main() {
    let dims = (4, 4);
    let mut grid = Array2::from_elem((4, 4), 0);
    grid[(0, 0)] = b'd';
    grid[(0, 1)] = b'n';
    grid[(0, 2)] = b'n';
    grid[(0, 3)] = b'g';
    grid[(1, 0)] = b'n';
    grid[(1, 1)] = b'o';
    grid[(1, 2)] = b'i';
    grid[(1, 3)] = b'i';
    grid[(2, 0)] = b'e';
    grid[(2, 1)] = b't';
    grid[(2, 2)] = b'n';
    grid[(2, 3)] = b't';
    grid[(3, 0)] = b'r';
    grid[(3, 1)] = b'm';
    grid[(3, 2)] = b'i';
    grid[(3, 3)] = b'a';

    let t_before = Instant::now();
    for _ in 0..1000 {
        let vec = eval_grid(dims, &grid);
        assert_eq!(vec.len(), 119)
    }
    println!("Took: {:?}", t_before.elapsed());
}

fn eval_grid<'a>(dims: (usize, usize), grid: &Array2<u8>) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    let mut stack: Vec<(u8, Vec<(usize, (usize, usize))>)> = Vec::with_capacity(dims.0 * dims.1);

    let mut last_word = "";
    'wl: for word in include_str!("../resources/words.txt").split("\n") {
        // Throw away the irrelevant part of the stack
        let keep = word
            .bytes()
            .zip(last_word.bytes())
            .take_while(|(a, b)| a == b)
            .count();
        stack.truncate(keep);
        last_word = word;

        // If the last element of the stack is empty, continue.
        // This means we did not throw away any part of the stack
        if let Some((_, last)) = stack.last() && last.is_empty() { continue; }

        // Then, push what is needed
        for b in &word.as_bytes()[stack.len()..] {
            let next: Vec<(usize, (usize, usize))> = if let Some((_, last)) = stack.last() {
                last.iter()
                    .enumerate()
                    .map(|(i, (_, p))| neighbours(dims, *p).map(move |ps| (i, ps)))
                    .flatten()
                    .filter(|(_, p)| grid[*p] == *b)
                    .filter(|(i, e)| verify_chain(&stack[..], *i, *e))
                    .collect()
            } else {
                all_cells(dims)
                    .filter(|p| grid[*p] == *b)
                    .map(|p| (0, p))
                    .collect()
            };
            let should_stop = next.is_empty();
            stack.push((*b, next));
            if should_stop {
                continue 'wl;
            }
        }

        //We finished the word!
        result.push(word);
    }

    result
}

fn verify_chain<T: Eq>(words: &[(u8, Vec<(usize, T)>)], i: usize, e: T) -> bool {
    match words.split_last() {
        None => true,
        Some((next, rest)) => {
            let next = &next.1[i];
            if next.1 == e {
                false
            } else {
                verify_chain(rest, next.0, e)
            }
        }
    }
}

fn all_cells(dims: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (0..dims.0)
        .map(move |p0| (0..dims.1).map(move |p1| (p0, p1)))
        .flatten()
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
    ]
    .into_iter()
    .filter(move |(p0, p1)| *p0 < dims.0 && *p1 < dims.1)
}
