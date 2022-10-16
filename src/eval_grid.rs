use ndarray::Array2;
use crate::{Trie};

pub fn eval_grid<'a>(grid: &Array2<u8>, trie: &Trie) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    type Pos = (usize, (usize, usize));
    let mut position_stack: Vec<Pos> = Vec::with_capacity(grid.dim().0 * grid.dim().1 * 8);
    let mut level_stack: Vec<usize> = Vec::with_capacity(grid.dim().0 * grid.dim().1);
    let mut word_stack: Vec<u8> = Vec::with_capacity(grid.dim().0 * grid.dim().1);

    let mut trie_stack: Vec<(usize, &[Option<Box<Trie>>])> = Vec::with_capacity(grid.dim().0 * grid.dim().1);
    trie_stack.push((0, &trie.children[..]));

    // While there are tries left on the stack
    'wl: while let Some((trie_len, trie)) = trie_stack.pop() {
        // Find the first subtrie that exists, if any
        if let Some((subtrie, rest)) = first_sub(trie) {
            // Push rest for later
            trie_stack.push((trie_len, rest));

            // For recovery
            let pos_prev = position_stack.len();
            let lev_prev = level_stack.len();

            // Push bytes of subtrie to state
            for &b in subtrie.bytes {
                let frame_start = position_stack.len();

                word_stack.push(b);
                if let Some(&last_start) = level_stack.last() {
                    level_stack.push(frame_start);

                    // Walk over positions in previous frame
                    for p_prev in last_start..frame_start {
                        // Check all neighbours, if we can go there (correct letter & haven't been there), push it.
                        for nb in neighbours(grid.dim(), position_stack[p_prev].1) {
                            // Wrong letter
                            if grid[nb] != b {
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
                            .filter(|p| grid[*p] == b)
                            .map(|p| (usize::MAX, p)),
                    );
                }

                if frame_start == position_stack.len() {
                    // Revert to state before this subtrie, and continue
                    position_stack.truncate(pos_prev);
                    level_stack.truncate(lev_prev);
                    word_stack.truncate(lev_prev);
                    continue 'wl;
                }
            }

            // This was a success, if there is an end, we take it
            trie_stack.push((subtrie.bytes.len(), &subtrie.children));
            if subtrie.end {
                result.push(String::from_utf8(word_stack.clone()).unwrap());
            }
        } else {
            // We need to pop the current trie off the stack
            if trie_len == 0 { continue 'wl; }

            let new_pos = level_stack.len() - trie_len;
            position_stack.truncate(level_stack[new_pos]);
            level_stack.truncate(new_pos);
            word_stack.truncate(new_pos);
        }
    }

    result
}

#[inline(always)]
fn first_sub(els: &[Option<Box<Trie>>]) -> Option<(&Trie, &[Option<Box<Trie>>])> {
    for i in 0..els.len() {
        if let Some(e) = &els[i] {
            return Some((e, &els[i+1 ..]))
        }
    }
    None
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
pub fn all_cells(dims: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
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
