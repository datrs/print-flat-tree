extern crate print_flat_tree as lib;
extern crate structopt;

use structopt::StructOpt;

fn main() {
  let args = lib::Opts::from_args();
  lib::print(&args);
}
