#![feature(let_chains)]

use ndarray::Array2;
use std::io::{stdin, BufRead};
use dashmap::DashMap;
use rand::Rng;
use rayon::prelude::*;

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

fn random_grid() -> Array2<u8> {
    let mut rng = rand::thread_rng();

    let mut grid = Array2::from_elem((3, 3), 0);
    grid[(0, 0)] = rng.gen_range(b'a'..=b'z');
    grid[(0, 1)] = rng.gen_range(b'a'..=b'z');
    grid[(0, 2)] = rng.gen_range(b'a'..=b'z');
    grid[(1, 0)] = rng.gen_range(b'a'..=b'z');
    grid[(1, 1)] = rng.gen_range(b'a'..=b'z');
    grid[(1, 2)] = rng.gen_range(b'a'..=b'z');
    grid[(2, 0)] = rng.gen_range(b'a'..=b'z');
    grid[(2, 1)] = rng.gen_range(b'a'..=b'z');
    grid[(2, 2)] = rng.gen_range(b'a'..=b'z');

    grid
}

fn todays_grid() -> Array2<u8> {
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

    grid
}

fn grid_nbs(grid: &Array2<u8>) -> Vec<Array2<u8>> {
    all_cells((3, 3)).flat_map(|c1| {
        (b'a'..=b'z').flat_map(move |v1| {
            all_cells((3, 3)).flat_map(move |c2| {
                (b'a'..=b'z').map(move |v2| {
                    let mut grid: Array2<u8> = grid.clone();
                    grid[c1] = v1;
                    grid[c2] = v2;
                    grid
                })
            })
        })
    }).collect()
}

fn single_iter(cache: &DashMap<Array2<u8>, usize>) -> Array2<u8> {
    let mut grid = random_grid();

    loop {
        let best = grid_nbs(&grid).into_par_iter().max_by_key(|nbgrid| {
            if let Some(v) = cache.get(nbgrid) {
                *v
            } else {
                let v = eval_grid(nbgrid).len();
                cache.insert(nbgrid.clone(), v);
                v
            }
        }).unwrap();

        if best == grid {
            return grid;
        }

        grid = best;
    }
}

fn main() {
    // let puzzle = read_input();
    // let sol = eval_grid(&puzzle);
    // println!("({}) {:?}", sol.len(), sol);

    let cache: DashMap<Array2<u8>, usize> = DashMap::new();
    let mut best_v = 0;

    loop {
        let next = single_iter(&cache);
        let v = *cache.get(&next).unwrap();
        if v > best_v {
            best_v = v;
            println!("Best: (Score: {})", *cache.get(&next).unwrap());
            println!("{}", next.map(|c| *c as char))
        }
    }
}

fn eval_grid<'a>(grid: &Array2<u8>) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    let mut position_stack: Vec<(usize, (usize, usize))> = Vec::with_capacity(grid.dim().0 * grid.dim().1 * 8);
    let mut level_stack: Vec<usize> = Vec::with_capacity(grid.dim().0 * grid.dim().1);
    let mut word_stack: Vec<u8> = Vec::with_capacity(grid.dim().0 * grid.dim().1);

    'wl: for word in include_str!("../resources/words.txt").split('\n') {
        // Throw away the irrelevant part of the stack
        let keep = word
            .bytes()
            .zip(word_stack.iter())
            .take_while(|(wb, sb)| *wb == **sb)
            .count();

        // If this word is a prefix of the level stack and the previous word was rejected (last frame is empty), we skip this word.
        // This is correct since the words are in alphabetical order, we could never have `ab` after `abc`
        if keep == level_stack.len() {
            if let Some(last_frame_start) = level_stack.last() {
                if *last_frame_start == position_stack.len() {
                    continue;
                }
            }
        } else {
            // Word is not a prefix of the level stack, so we remove the irrelevant part of the level stack
            position_stack.truncate(level_stack[keep]);
            level_stack.truncate(keep);
            word_stack.truncate(keep);
        }

        // Start processing the word, byte by byte
        for b in &word.as_bytes()[level_stack.len()..] {
            let frame_start = position_stack.len();

            word_stack.push(*b);
            if let Some(&last_start) = level_stack.last() {
                level_stack.push(frame_start);

                // Walk over positions in previous frame
                for p_prev in last_start..frame_start {
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
                level_stack.push(0);

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
        .flat_map(move |p0| (0..dims.1).map(move |p1| (p0, p1)))
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
