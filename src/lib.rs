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

extern crate flat_tree;
#[macro_use]
extern crate structopt;

use std::cmp;

const VERT_CHAR: char = '│';
const HORI_CHAR: char = '─';
const LEFT_BOT_CHAR: char = '┐';
const LEFT_TOP_CHAR: char = '┘';

/// Configuration for the `print` function.
#[derive(Debug, StructOpt)]
#[structopt(about = "Converts a flat-tree to a string")]
pub struct Options {
  #[structopt(short = "v", long = "verbose", help = "Toggle verbose logging")]
  pub verbose: bool,
  /// Nodes that are part of the flat-tree.
  #[structopt(help = "For example '0 1 2 3 7 8 9 10'")]
  pub list: Vec<i32>,
}

/// Converts a flat_tree to a string.
pub fn fmt(opts: &Options) {
  let list = &opts.list;
  println!("{:?}", list);
  let width = list.len() + 1;

  let last_block = list.len() - list.len() % 2;
  let roots = flat_tree::full_roots(last_block as u64);

  let max = list.iter().fold(0, |prev, curr| cmp::max(prev, *curr));
  let blank = format!("{:width$}", ' ', width = width + 1);
  let mut matrix = vec![vec![blank.to_string(); max as usize]; list.len()];

  for i in 0..list.len() {
    let depth = flat_tree::depth(i as u64);
    let val = format!("{:width$}", i, width = width + 1);
    matrix[i as usize][depth as usize] = val;

    if let Some(children) = flat_tree::children(i as u64) {
      add_path(&list, &mut matrix, children.0, i as u64, depth, 1);
      if (children.1 as usize) < list.len() {
        add_path(&list, &mut matrix, children.1, i as u64, depth, -1);
      }
    }
  }

  // println!("val: {:?}", matrix);
}

fn add_path(
  list: &Vec<i32>,
  matrix: &mut Vec<Vec<String>>,
  child: u64,
  parent: u64,
  parent_depth: u64,
  direction: i32,
) -> () {
  if list.get(child as usize).is_some() {
    let depth = flat_tree::depth(child);
    let ptr = depth + 1;

    for i in ptr..parent_depth {
      println!("child {:?}", child);
      matrix[child as usize][i as usize] = pad(HORI_CHAR, HORI_CHAR);
    }
    let turn_char = if direction < 0 {
      LEFT_TOP_CHAR
    } else {
      LEFT_BOT_CHAR
    };
    matrix[child as usize][ptr as usize] = pad(turn_char, HORI_CHAR);

    let mut start: i32 = child as i32;
    loop {
      start += direction;
      println!("start:{:?} dir:{:?} parent:{:?}", start, direction, parent);
      if start == parent as i32 {
        break;
      };
      matrix[child as usize][ptr as usize] = pad(VERT_CHAR, ' ');
    }
  }
}

#[inline(always)]
fn pad(str: char, pad_char: char) -> String {
  format!("{:width$}{}", pad_char, str, width = 5)
}
