extern crate structopt;
#[macro_use]
extern crate structopt_derive;

#[derive(Debug, StructOpt)]
pub struct Opts {
  /// Nodes that are part of the flat-tree.
  #[structopt(name = "nodes", help = "For example '0 1 2 3 7 8 9 10'")]
  pub nodes: Vec<i32>,
}

pub fn print(opts: &Opts) -> Option<&str> {
  println!("{:?}", opts);
  None
}
