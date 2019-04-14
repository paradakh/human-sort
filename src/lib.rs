//! Utilities to sort and compare strings with numeric symbols in human-friendly order.
//!
//! It built over iterators and compare string slices char by char (except for numerals)
//! until the first difference found without creating Strings or another structures with whole
//! data from provided &str, so doesn't require lots of memory.
//!
//! # Examples
//!
//! ```
//! use human_sort::sort;
//!
//! let mut arr = ["file10.txt", "file2.txt", "file1.txt"];
//! sort(&mut arr);
//!
//! assert_eq!(arr, ["file1.txt", "file2.txt", "file10.txt"]);
//! ```
//!
//! ```
//! use std::cmp::Ordering;
//! use human_sort::compare;
//!
//! assert_eq!(compare("item200", "item3"), Ordering::Greater);
//! ```

mod iter_pair;

use iter_pair::IterPair;
use std::{cmp::Ordering, iter::Peekable, str::Chars};

/// Sorts [&str] in human-friendly order
///
/// # Example
///
/// ```
/// use human_sort::sort;
///
/// let mut arr = ["file10.txt", "file2.txt", "file1.txt"];
/// sort(&mut arr);
///
/// assert_eq!(arr, ["file1.txt", "file2.txt", "file10.txt"]);
/// ```
///
pub fn sort(arr: &mut [&str]) {
    arr.sort_by(|a, b| compare(a, b));
}

/// Compares string slices
///
/// # Example
///
/// ```
/// use std::cmp::Ordering;
/// use human_sort::compare;
///
/// assert_eq!(compare("item200", "item3"), Ordering::Greater);
/// ```
///
pub fn compare(s1: &str, s2: &str) -> Ordering {
    compare_chars_iters(s1.chars(), s2.chars()).unwrap_or(s1.cmp(s2))
}

///
/// ```
/// use std::cmp::Ordering;
/// use human_sort::compare_chars_iters;
/// assert_eq!(compare_chars_iters("aaa".chars(), "bbb".chars()), Ok(Ordering::Less));
/// ```
///
pub fn compare_chars_iters<'a>(c1: Chars<'a>, c2: Chars<'a>) -> Result<Ordering, ()> {
    let mut iters = IterPair::from(c1, c2);

    while let [Some(x), Some(y)] = iters.peek() {
        if x == y {
            iters.next();
        } else if x.is_numeric() && y.is_numeric() {
            match take_numeric(&mut iters.fst).cmp(&take_numeric(&mut iters.lst)) {
                Ordering::Equal => iters.next(),
                ref a => return Ok(*a),
            };
        } else {
            return Ok(x.cmp(y));
        }
    }

    Err(())
}

fn take_numeric(iter: &mut Peekable<Chars>) -> u32 {
    let mut sum = 0;

    while let Some(p) = iter.peek() {
        match p.to_string().parse::<u32>() {
            Ok(n) => {
                sum = sum * 10 + n;
                iter.next();
            }
            _ => break,
        }
    }

    sum
}
