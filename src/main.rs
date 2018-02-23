extern crate print_flat_tree as lib;
extern crate structopt;

use structopt::StructOpt;
use lib::Options;

fn main() {
  let args = Options::from_args();
  lib::fmt(&args);
}
