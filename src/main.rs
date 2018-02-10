extern crate print_flat_tree as lib;

#[macro_use]
extern crate quicli;

use quicli::prelude::*;

main!({
  let args = lib::Opts::from_args();
  lib::print(&args);
});
