#![deny(warnings)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate print_flat_tree as lib;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

/// Configuration for the `print` function.
#[derive(Debug, StructOpt)]
#[structopt(about = "Converts a flat-tree to a string")]
pub struct Options {
  /// Nodes that are part of the flat-tree.
  #[structopt(help = "For example '0 1 2 3 7 8 9 10'")]
  pub tree: Vec<usize>,
}

fn main() {
  let args = Options::from_args();
  let tree_str = lib::fmt(&args.tree);
  print!("{}", tree_str);
}
