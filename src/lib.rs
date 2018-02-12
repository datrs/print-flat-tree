//! Converts a [flat tree](http://docs.rs/flat-tree) to a string.
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
pub fn print(opts: &Options) {
  let list = &opts.list;
  let width = list.len() + 1;

  let last_block = list.len() - list.len() % 2;
  let roots = flat_tree::full_roots(last_block as u64);

  let max = list.iter().fold(0, |prev, curr| cmp::max(prev, *curr));
  let blank = format!("{:width$}", ' ', width = 5);
  let matrix = vec![vec![&blank; max as usize]; list.len()];

  for i in 0..list.len() {
    let depth = flat_tree::depth(i as u64);
    let children = flat_tree::children(i as u64);
    // matrix[i as usize][depth as usize] = String::from("sup");
  }

  println!("val: {:?}", matrix);
}

// fn add_path(child: i32, parent: i32, parent_depth: i32, dir: i32) {}
