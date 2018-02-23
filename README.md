# print-flat-tree
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Converts a flat-tree to a string. Adapted from
[mafintosh/print-flat-tree](https://github.com/mafintosh/print-flat-tree).

## Installation
```sh
$ cargo add print-flat-tree
```

## Usage
```txt
print-flat-tree
Converts a flat-tree to a string

USAGE:
    print-flat-tree [tree]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <tree>...    For example '0 1 2 3 7 8 9 10'
```

### Output
```txt
print-flat-tree 0 1 2 3 5 6 7 10 11

  0──┐
     1──┐
  2──┘  │
        3──┐
           │
           │
           │
           7
  8──┐     │
     9──┐  │
 10──┘  │  │
       11──┘
 12──┐  │
    13──┘
 14──┘
````

## Links
- [documentation][8]
- [crate][2]

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/print-flat-tree.svg?style=flat-square
[2]: https://crates.io/crate/print-flat-tree
[3]: https://img.shields.io/travis/datrs/print-flat-tree.svg?style=flat-square
[4]: https://travis-ci.org/datrs/print-flat-tree
[5]: https://img.shields.io/crates/d/print-flat-tree.svg?style=flat-square
[6]: https://crates.io/crate/print-flat-tree
[7]: https://docs.rs/print-flat-tree/badge.svg
[8]: https://docs.rs/print-flat-tree
