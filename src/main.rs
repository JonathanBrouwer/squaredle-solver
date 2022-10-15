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
        let vec = eval_grid(&grid);
        assert_eq!(vec.len(), 119)
    }
    println!("Took: {:?}", t_before.elapsed());
}

fn eval_grid<'a>(grid: &Array2<u8>) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    let mut position_stack: Vec<(usize, (usize, usize))> =
        Vec::with_capacity(grid.dim().0 * grid.dim().1 * 8);
    let mut level_stack: Vec<(u8, usize)> = Vec::with_capacity(grid.dim().0 * grid.dim().1);

    'wl: for word in include_str!("../resources/words.txt").split("\n") {
        // Throw away the irrelevant part of the stack
        let keep = word
            .bytes()
            .zip(level_stack.iter())
            .take_while(|(wb, (sb, _))| *wb == *sb)
            .count();

        // If this word is a prefix of the level stack and the previous word was rejected (last frame is empty), we skip this word.
        // This is correct since the words are in alphabetical order, we could never have `ab` after `abc`
        if keep == level_stack.len() {
            if let Some(last_frame_start) = level_stack.last() {
                if last_frame_start.1 == position_stack.len() {
                    continue;
                }
            }
        } else {
            // Word is not a prefix of the level stack, so we remove the irrelevant part of the level stack
            position_stack.truncate(level_stack[keep].1);
            level_stack.truncate(keep);
        }

        // Start processing the word, byte by byte
        for b in &word.as_bytes()[level_stack.len()..] {
            let frame_start = position_stack.len();

            if let Some(&last_start) = level_stack.last() {
                level_stack.push((*b, frame_start));
                // Walk over positions in previous frame
                for p_prev in last_start.1..frame_start {
                    // Check all neighbours, if we can go there (correct letter & haven't been there), push it.
                    for nb in neighbours(grid.dim(), position_stack[p_prev].1) {
                        // Wrong letter
                        if grid[nb] != *b {
                            continue;
                        }
                        // Have been there
                        if !verify_chain(&position_stack, p_prev, nb) {
                            continue;
                        }
                        position_stack.push((p_prev, nb));
                    }
                }
            } else {
                level_stack.push((*b, 0));
                position_stack.extend(
                    all_cells(grid.dim())
                        .filter(|p| grid[*p] == *b)
                        .map(|p| (usize::MAX, p)),
                );
            }

            if frame_start == position_stack.len() {
                continue 'wl;
            }
        }

        //We finished the word!
        result.push(word);
    }

    result
}

#[inline(always)]
fn verify_chain(
    position_stack: &Vec<(usize, (usize, usize))>,
    i: usize,
    e: (usize, usize),
) -> bool {
    if i == usize::MAX {
        return true;
    }
    if position_stack[i].1 == e {
        return false;
    }
    verify_chain(position_stack, position_stack[i].0, e)
}

#[inline(always)]
fn all_cells(dims: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (0..dims.0)
        .map(move |p0| (0..dims.1).map(move |p1| (p0, p1)))
        .flatten()
}

#[inline(always)]
fn neighbours(dims: (usize, usize), pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        (pos.0.wrapping_sub(1), pos.1.wrapping_sub(1)), // - -
        (pos.0, pos.1.wrapping_sub(1)),                 // . -
        (pos.0 + 1, pos.1.wrapping_sub(1)),             // + -
        (pos.0 + 1, pos.1),                             // + .
        (pos.0 + 1, pos.1 + 1),                         // + +
        (pos.0, pos.1 + 1),                             // . +
        (pos.0.wrapping_sub(1), pos.1 + 1),             // - +
        (pos.0.wrapping_sub(1), pos.1),                 // - .
    ]
    .into_iter()
    .filter(move |(p0, p1)| *p0 < dims.0 && *p1 < dims.1)
}
