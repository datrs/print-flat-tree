//! Converts a [`flat tree`](http://docs.rs/flat-tree) to a string.
//!
//! ## Usage
//! ```rust,ignore
//! extern crate print_flat_tree;
//!
//! use print_flat_tree::print;
//!
//! let tree = vec!(0, 1, 2, 3, 7, 8, 9, 10);
//! println!("{:?}", print(tree));
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
#[macro_use]
extern crate structopt;

use std::cmp;

const DOWN: char = '│';
const LEFT: char = '─';
const TURN_DOWN: char = '┐';
const TURN_UP: char = '┘';

/// Configuration for the `print` function.
#[derive(Debug, StructOpt)]
#[structopt(about = "Converts a flat-tree to a string")]
pub struct Options {
  /// Toggle verbose mode.
  #[structopt(short = "v", long = "verbose", help = "Toggle verbose logging")]
  pub verbose: bool,
  /// Nodes that are part of the flat-tree.
  #[structopt(help = "For example '0 1 2 3 7 8 9 10'")]
  pub tree: Vec<usize>,
}

/// Converts a `flat_tree` to a string.
pub fn fmt(opts: &Options) {
  let tree = &opts.tree;

  // Fill a vec with bools, indicating if a value exists or not.
  let max = tree.iter().fold(0, |prev, curr| cmp::max(prev, *curr));
  let mut list = vec![false; max + 1];
  for &i in tree {
    list[i] = true;
  }

  let width = list.len() + 1;
  let last_block = list.len() - list.len() % 2;
  let _roots = flat_tree::full_roots(last_block as u64);

  let blank = format!("{:width$}", ' ', width = width + 1);
  let mut matrix = vec![vec![blank.to_string(); max as usize]; list.len()];

  println!("length: {:?}", list.len());
  for i in 0..list.len() {
    let depth = flat_tree::depth(i as u64);
    let val = format!("{:width$}", i, width = width + 1);
    matrix[i as usize][depth as usize] = val;

    if let Some(children) = flat_tree::children(i as u64) {
      println!("i: {:?}, children: {:?}, depth: {:?}", i, children, depth);
      add_path(&list, &mut matrix, children.0, i as u64, depth, 1);
      // if (children.1 as usize) < list.len() {
      //   add_path(list, &mut matrix, children.1, i as u64, depth, -1);
      // }
    }
  }

  // println!("val: {:?}", matrix);
}

fn add_path(
  list: &[bool],
  matrix: &mut Vec<Vec<String>>,
  child: u64,
  parent: u64,
  parent_depth: u64,
  direction: i32,
) -> () {
  if list.get(child as usize).is_none() {
    return;
  }

  let depth = flat_tree::depth(child);
  let ptr = depth + 1;

  for i in ptr..parent_depth {
    matrix[child as usize][i as usize] = pad(LEFT, LEFT);
  }

  let turn_char = if direction < 0 { TURN_UP } else { TURN_DOWN };
  matrix[child as usize][ptr as usize] = pad(turn_char, LEFT);

  let mut start: i32 = child as i32;
  // println!(
  //   "child {}, parent {}, direction {}",
  //   start, parent, direction
  // );
  loop {
    start += direction;
    // println!("start:{:?} dir:{:?} parent:{:?}", start, direction, parent);
    if start == parent as i32 {
      break;
    };
    matrix[child as usize][ptr as usize] = pad(DOWN, ' ');
  }
}

fn pad(str: char, pad_char: char) -> String {
  format!("{:width$}{}", pad_char, str, width = 5)
}
