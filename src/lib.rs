//! Converts a [`flat tree`](http://docs.rs/flat-tree) to a string.
//!
//! ## Usage
//! ```rust,ignore
//! extern crate print_flat_tree;
//!
//! use print_flat_tree::fmt;
//!
//! let tree = vec!(0, 1, 2, 3, 7, 8, 9, 10);
//! print!("{}", fmt(tree));
//! ```
//!
//! Which outputs:
//!
//! ```text
//!  0──┐
//!     1──┐
//!  2──┘  │
//!        3──┐
//!           │
//!           │
//!           │
//!           7
//!  8──┐
//!     9
//! 10──┘
//! ```
//!
//! As can be seen from the above diagram `7` is the parent of `3`, and `3` is
//! the parent of `1` etc.
//!
//! ## See Also
//! - [mafintosh/print-flat-tree (JavaScript)](https://github.com/mafintosh/print-flat-tree)
//! - [flat-tree](https://docs.rs/flat-tree)

#![deny(warnings)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate flat_tree;

use std::cmp;

const DOWN: char = '│';
const LEFT: char = '─';
const TURN_DOWN: char = '┐';
const TURN_UP: char = '┘';

/// Converts a `flat_tree` to a string.
pub fn fmt(tree: &Vec<usize>) -> String {
  // Fill a vec with bools, indicating if a value exists or not.
  let max = tree.iter().fold(0, |prev, curr| cmp::max(prev, *curr));
  let mut list = vec![false; max + 1];
  for &i in tree {
    list[i] = true;
  }

  let width = (list.len().to_string()).len() + 1;
  let last_block = list.len() - list.len() % 2;
  let _roots = flat_tree::full_roots(last_block as u64);

  let blank = format!("{:width$}", ' ', width = width);
  let mut matrix = vec![vec![blank.to_string(); max as usize]; list.len()];

  for i in 0..list.len() {
    if list[i] == false {
      continue;
    }
    let depth = flat_tree::depth(i as u64);
    let val = format!("{:width$}", i, width = width);
    matrix[i as usize][depth as usize] = val;

    if let Some(children) = flat_tree::children(i as u64) {
      add_path(&list, &mut matrix, children.0, i as u64, depth, 1);
      if (children.1 as usize) < list.len() {
        add_path(&list, &mut matrix, children.1, i as u64, depth, -1);
      }
    }
  }

  let mut flat_tree_str = String::from("");
  for arr in matrix {
    let partial = arr.join("") + "\n";
    flat_tree_str.push_str(partial.as_str());
  }

  flat_tree_str
}

fn add_path(
  list: &[bool],
  matrix: &mut Vec<Vec<String>>,
  child: u64,
  parent: u64,
  parent_depth: u64,
  dir: i32,
) -> () {
  if list[child as usize] == false {
    return;
  }

  let depth = flat_tree::depth(child);
  let width = (list.len().to_string()).len() + 1;
  let ptr = depth + 1;

  for i in ptr..parent_depth {
    matrix[child as usize][i as usize] = pad(LEFT, LEFT, width);
  }

  let turn_char = if dir < 0 { TURN_UP } else { TURN_DOWN };
  matrix[child as usize][ptr as usize] = pad(turn_char, LEFT, width);

  let mut _child: i32 = child as i32;
  loop {
    _child += dir;
    if _child == parent as i32 {
      break;
    };
    matrix[_child as usize][ptr as usize] = pad(DOWN, ' ', width);
  }
}

fn pad(str: char, pad_char: char, width: usize) -> String {
  let mut symbol = String::from("");
  for i in 0..width {
    if i == width - 1 {
      symbol.push(str);
    } else {
      symbol.push(pad_char);
    }
  }
  symbol
}
