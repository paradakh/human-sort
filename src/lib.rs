//! Utilities to sort and compare strings with numeric symbols in human-friendly order.
//!
//! It built over iterators and compare string slices `char` by `char` (except for numerals)
//! until the first difference found without creating `Strings` or another structures with whole
//! data from provided `&str`, so doesn't require lots of memory.
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

use std::{cmp::Ordering, str::Chars};

mod big_num;

use big_num::BigNum;

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
pub fn sort<T: AsRef<str>>(arr: &mut [T]) {
    arr.sort_by(|a, b| compare(a.as_ref(), b.as_ref()));
}
#[test]
fn test_sort() {
    macro_rules! sorted {
        ($in:expr) => {{
            let mut v = $in;
            sort(&mut v);
            v
        }}
    }
    assert_eq!(sorted!(["file10.txt", "file2.txt", "file1.txt"]), ["file1.txt", "file2.txt", "file10.txt"]);
    assert_eq!(sorted!(["file10.txt".to_string(), "file2.txt".to_string(), "file1.txt".to_string()]), ["file1.txt", "file2.txt", "file10.txt"]);
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
    compare_chars_iters(s1.chars(), s2.chars())
}

/// ```
/// use std::cmp::Ordering;
/// use human_sort::compare_chars_iters;
/// assert_eq!(compare_chars_iters("aaa".chars(), "bbb".chars()), Ordering::Less);
/// ```
pub fn compare_chars_iters<'a>(mut c1: Chars<'a>, mut c2: Chars<'a>) -> Ordering {
    loop {
        // cloning std iterators is super cheap (only pedantic reasons why they aren't [`Copy`])
        // we could use [`Peekable`] here, but that doesn't support [`Chars::as_str`] used by [`BigNum::extract`].
        match (c1.clone().next(), c2.clone().next()) {
            (Some(x), Some(y)) => {
                let cmp = if x.is_digit(10) && y.is_digit(10) {
                    BigNum::extract(&mut c1).unwrap().cmp(&BigNum::extract(&mut c2).unwrap())
                } else {
                    x.cmp(&y)
                };

                match cmp {
                    Ordering::Equal => { c1.next(); c2.next(); },
                    a => return a,
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}
#[test]
fn test_compare_chars_iters() {
    assert_eq!(compare_chars_iters("x101".chars(), "x1000".chars()), Ordering::Less);
    assert_eq!(compare_chars_iters("x101".chars(), "x2000".chars()), Ordering::Less);

    assert_eq!(compare_chars_iters("1:".chars(), "15:".chars()), Ordering::Less);
    assert_eq!(compare_chars_iters("101:".chars(), "15:".chars()), Ordering::Greater);

    assert_eq!(compare_chars_iters(
        "PowerTools/x86_64/os/repodata/9379911671413f8a51cd04665cd9bafc8200f927505008e8a11145034b53c776-other.xml.gz".chars(),
        "PowerTools/x86_64/os/repodata/43ed191200dbc7c83be76c3410f118f931bbe21ff6a58f5f549d0e351f3aea94-other.sqlite.xz".chars()),
        Ordering::Greater);

    assert_eq!(compare_chars_iters("apple".chars(), "apples".chars()), Ordering::Less);
    assert_eq!(compare_chars_iters("apples".chars(), "apple".chars()), Ordering::Greater);
    assert_eq!(compare_chars_iters("apples".chars(), "apples".chars()), Ordering::Equal);
}
