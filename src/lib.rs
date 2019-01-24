use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    iter::Peekable,
    str::Chars,
};

#[test]
fn compare_by_alphabet() {
    assert_eq!(
        HumanStr::from_str("aaa").cmp(&HumanStr::from_str("bbb")),
        Ordering::Less
    );
}

#[test]
fn compare_with_numeric() {
    assert_eq!(
        HumanStr::from_str("a1aa").cmp(&HumanStr::from_str("a2aa")),
        Ordering::Less
    )
}

#[test]
fn should_sort() {
    let mut arr = ["lol10", "lol2"];
    human_sort(&mut arr);

    assert_eq!(arr, ["lol2", "lol10"]);
}

pub struct HumanStr<'a> {
    s: &'a str,
}

impl<'a> HumanStr<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self { s }
    }
}

impl<'a> PartialEq for HumanStr<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl<'a> Eq for HumanStr<'a> {}

impl<'a> Ord for HumanStr<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self.s, other.s)
    }
}

impl<'a> PartialOrd for HumanStr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn human_sort(arr: &mut [&str]) {
    arr.sort_by(|a, b| HumanStr::from_str(a).cmp(&HumanStr::from_str(b)));
}

pub fn compare(s1: &str, s2: &str) -> Ordering {
    let mut s1_iter = s1.chars().peekable();
    let mut s2_iter = s2.chars().peekable();

    loop {
        let (x, y) = (s1_iter.next(), s2_iter.next());

        if x.is_some() && y.is_some() {
            let (x, y) = (x.unwrap(), y.unwrap());

            if x == y {
                continue;
            } else {
                match (x.is_numeric(), y.is_numeric()) {
                    (false, false) => return x.to_lowercase().cmp(y.to_lowercase()),
                    (true, false) => return Ordering::Greater,
                    (false, true) => return Ordering::Less,
                    (true, true) => {
                        let x_sum = parse_numeric_part(x, &mut s1_iter);
                        let y_sum = parse_numeric_part(y, &mut s2_iter);

                        if x_sum != y_sum {
                            return x_sum.cmp(&y_sum);
                        }
                    }
                }
            };
        } else {
            return s1.len().cmp(&s2.len());
        };
    }
}

fn parse_numeric_part(val: char, iter: &mut Peekable<Chars>) -> u32 {
    let mut sum = val.to_string();

    while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
        sum.push(iter.next().unwrap());
    }

    sum.parse().unwrap()
}
