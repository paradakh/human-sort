[![Crates.io](https://img.shields.io/crates/v/human-sort.svg)](https://crates.io/crates/human-sort)
[![Build Status](https://travis-ci.org/vladnagih/human-sort.svg?branch=master)](https://travis-ci.org/vladnagih/human-sort)

# human-sort

Collection of utilities to sort and compare strings with numeric symbols
in human-friendly order.

Utilities built over iterators and compare strings char by char (except for numerals)
until the first difference found.

Utilities don't create Strings or another structures with whole data from provided &str,
so don't require lots of memory.

## Examples

```rust
use human_sort::sort;

let mut arr = ["file10.txt", "file2.txt", "file1.txt"];
sort(&mut arr);

assert_eq!(arr, ["file1.txt", "file2.txt", "file10.txt"]);
```

```rust
use std::cmp::Ordering;
use human_sort::compare;

assert_eq!(compare("item200", "item3"), Ordering::Greater);
```

## License

Licensed under MIT license.
