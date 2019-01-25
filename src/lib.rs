use std::{
    cmp::Ordering,
    iter::Peekable,
    str::Chars,
};

#[test]
fn should_sort() {
    let mut arr = ["lol10", "lol2"];
    sort(&mut arr);

    assert_eq!(arr, ["lol2", "lol10"]);
}

pub fn sort(arr: &mut [&str]) {
    arr.sort_by(|a, b| compare(a, b));
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
