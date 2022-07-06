use std::str::Chars;
use std::iter::Peekable;
use std::cmp::{PartialOrd, Ord, PartialEq, Eq, Ordering};

#[derive(Debug)]
pub struct BigNum(Vec<u8>);
impl BigNum {
    pub fn extract(chars: &mut Peekable<Chars>) -> Self {
        let mut res = vec![];
        while let Some(ch) = chars.peek().copied() {
            if !ch.is_digit(10) { break }
            let val = ch as u32 - '0' as u32;
            if val != 0 || !res.is_empty() { res.push(val as u8) }
            chars.next();
        }
        Self(res)
    }
}
impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => self.0.cmp(&other.0),
            x => x,
        }
    }
}
impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for BigNum {}

#[test]
fn test_bignum() {
    let mut chars1 = "58475730929537459873498243659824759837486536458973249865742985673498567984375698534769347569827569439%567498567".chars().peekable();
    let v1 = BigNum::extract(&mut chars1);
    assert_eq!(v1.0, &[5,8,4,7,5,7,3,0,9,2,9,5,3,7,4,5,9,8,7,3,4,9,8,2,4,3,6,5,9,8,2,4,7,5,9,8,3,7,4,8,6,5,3,6,4,5,8,9,7,3,2,4,9,8,6,5,7,4,2,9,8,5,6,7,3,4,9,8,5,6,7,9,8,4,3,7,5,6,9,8,5,3,4,7,6,9,3,4,7,5,6,9,8,2,7,5,6,9,4,3,9]);
    assert_eq!(chars1.next(), Some('%'));

    let mut chars2 = "0000000000000058475730929537459873498243659824759837486536458973249865742985673498567984375698534769347569827569439%567498567".chars().peekable();
    let v2 = BigNum::extract(&mut chars2);
    assert_eq!(v2.0, &[5,8,4,7,5,7,3,0,9,2,9,5,3,7,4,5,9,8,7,3,4,9,8,2,4,3,6,5,9,8,2,4,7,5,9,8,3,7,4,8,6,5,3,6,4,5,8,9,7,3,2,4,9,8,6,5,7,4,2,9,8,5,6,7,3,4,9,8,5,6,7,9,8,4,3,7,5,6,9,8,5,3,4,7,6,9,3,4,7,5,6,9,8,2,7,5,6,9,4,3,9]);
    assert_eq!(chars2.next(), Some('%'));

    assert_eq!(v1, v2);

    assert!(BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars().peekable()) == BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars().peekable()));
    assert!(BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464743".chars().peekable()) > BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars().peekable()));
    assert!(BigNum::extract(&mut "239457632479837654984659379837546498576394793275398476984576984756983765984576984567984576845769456948576546474".chars().peekable()) < BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars().peekable()));
    assert!(BigNum::extract(&mut "7394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars().peekable()) < BigNum::extract(&mut "23945763247983765498465937983754649857639479327539847698457698475698376598457698456798457684576945694857654645467".chars().peekable()));
}
