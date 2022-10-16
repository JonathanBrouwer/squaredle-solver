#![feature(let_chains)]

pub mod eval_grid;
pub mod trie;
pub mod read_input;

use ndarray::Array2;
use dashmap::DashMap;
use lazy_static::lazy_static;
use rayon::prelude::*;
use crate::eval_grid::{all_cells, eval_grid};
use crate::read_input::random_grid;
use crate::trie::Trie;

fn grid_nbs<'a>(grid: &'a Array2<u8>, dims: (usize, usize)) -> impl Iterator<Item=Array2<u8>> + 'a {
    all_cells(dims).flat_map(move |c1| {
        (b'a'..=b'z').flat_map(move |v1| {
            all_cells(dims).flat_map(move |c2| {
                (b'a'..=b'z').flat_map(move |v2| {
                    all_cells(dims).flat_map(move |c3| {
                        (b'a'..=b'z').map(move |v3| {
                            let mut grid: Array2<u8> = grid.clone();
                            grid[c1] = v1;
                            grid[c2] = v2;
                            grid[c3] = v3;
                            grid
                        })
                    })
                })
            })
        })
    })
}

fn single_iter(cache: &DashMap<Array2<u8>, usize>) -> Array2<u8> {
    let mut grid = random_grid(SEARCH_DIMS);

    loop {
        let best = grid_nbs(&grid, SEARCH_DIMS)
            .par_bridge()
            .filter(|nb| check_grid(nb))
            .max_by_key(|nbgrid| {
            if let Some(v) = cache.get(nbgrid) {
                *v
            } else {
                let v = eval_grid(nbgrid, &WORD_TRIE).len();
                cache.insert(nbgrid.clone(), v);
                v
            }
        }).unwrap();

        if best == grid {
            return grid;
        }
        grid = best;
        println!("Status: {}", *cache.get(&grid).unwrap());
    }
}

fn check_grid(grid: &Array2<u8>) -> bool {
    eval_grid(grid, &BAN_TRIE).len() == 0
}

const SEARCH_DIMS: (usize, usize) = (3, 3);

lazy_static! {
    static ref WORD_TRIE: Trie = {
        let mut trie = Trie::new();
        for word in include_str!("../resources/words.txt").split('\n') {
            trie.insert(word.as_bytes());
        }
        trie
    };
    static ref BAN_TRIE: Trie = {
        let mut trie = Trie::new();
        trie.insert(b"s");
        trie.insert(b"ed");
        trie.insert(b"er");
        trie.insert(b"ing");
        trie
    };
}

fn main() {
    // let puzzle = read_input::read_input();
    // let sol = eval_grid(&puzzle, &trie);
    // println!("({}) {:?}", sol.len(), sol.iter().filter(|x| x.len() > 9).collect::<Vec<_>>());

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


    // let grid = todays_grid();
    // let t_before = Instant::now();
    // for _ in 0..1000 {
    //     let vec = eval_grid(&grid, &trie);
    //     assert_eq!(vec.len(), 119)
    // }
    // println!("Took: {:?}", t_before.elapsed());
}


