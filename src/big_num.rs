use std::str::Chars;
use std::cmp::{PartialOrd, Ord, PartialEq, Eq, Ordering};

/// A reference to an arbitrary-sized positive integer within a string.
#[derive(Debug)]
pub struct BigNum<'a>(&'a [u8]);
impl<'a> BigNum<'a> {
    /// Extracts a [`BigNum`] from the given character sequence.
    /// If the sequence does not start with a valid `0-9` digit, returns [`Err`].
    /// If [`Err`] is returned, the iterator is not modified.
    pub fn extract(src: &mut Chars<'a>) -> Result<Self, ()> {
        let s = src.as_str();
        let start = s.chars().position(|ch| ch != '0').unwrap_or(s.len());
        let stop = s.chars().position(|ch| !ch.is_digit(10)).unwrap_or(s.len());
        if stop == 0 { return Err(()) }
        src.nth(stop - 1);
        Ok(Self(&s.as_bytes()[start..stop])) // ascii digits are 1 byte in utf8, so slicing and nth() are safe
    }
}
impl PartialOrd for BigNum<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BigNum<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => self.0.cmp(&other.0),
            x => x,
        }
    }
}
impl PartialEq for BigNum<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for BigNum<'_> {}

#[test]
fn test_bignum() {
    let mut chars1 = "58475730929537459873498243659824759837486536458973249865742985673498567984375698534769347569827569439%567498567".chars();
    let v1 = BigNum::extract(&mut chars1).unwrap();
    assert_eq!(v1.0, "58475730929537459873498243659824759837486536458973249865742985673498567984375698534769347569827569439".as_bytes());

    let mut chars2 = "0000000000000058475730929537459873498243659824759837486536458973249865742985673498567984375698534769347569827569439%567498567".chars();
    let v2 = BigNum::extract(&mut chars2).unwrap();
    assert_eq!(v2.0, "58475730929537459873498243659824759837486536458973249865742985673498567984375698534769347569827569439".as_bytes());

    assert_eq!(v1, v2);

    assert_eq!(chars1.next(), Some('%'));
    assert_eq!(chars2.next(), Some('%'));

    let mut chars3 = "7d".chars();
    let v2 = BigNum::extract(&mut chars3).unwrap();
    assert_eq!(v2.0, "7".as_bytes());
    assert_eq!(chars3.next(), Some('d'));

    let mut chars3 = "76f".chars();
    let v2 = BigNum::extract(&mut chars3).unwrap();
    assert_eq!(v2.0, "76".as_bytes());
    assert_eq!(chars3.next(), Some('f'));

    let mut chars3 = "0d".chars();
    let v2 = BigNum::extract(&mut chars3).unwrap();
    assert_eq!(v2.0, "".as_bytes());
    assert_eq!(chars3.next(), Some('d'));

    let mut chars3 = "000d".chars();
    let v2 = BigNum::extract(&mut chars3).unwrap();
    assert_eq!(v2.0, "".as_bytes());
    assert_eq!(chars3.next(), Some('d'));

    let mut chars3 = "d".chars();
    assert!(BigNum::extract(&mut chars3).is_err());
    assert_eq!(chars3.next(), Some('d'));

    let mut chars3 = "000".chars();
    let v2 = BigNum::extract(&mut chars3).unwrap();
    assert_eq!(v2.0, "".as_bytes());
    assert_eq!(chars3.next(), None);

    let mut chars3 = "".chars();
    assert!(BigNum::extract(&mut chars3).is_err());
    assert_eq!(chars3.next(), None);

    assert!(BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars()) == BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars()));
    assert!(BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464743".chars()) > BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars()));
    assert!(BigNum::extract(&mut "239457632479837654984659379837546498576394793275398476984576984756983765984576984567984576845769456948576546474".chars()) < BigNum::extract(&mut "2394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars()));
    assert!(BigNum::extract(&mut "7394576324798376549846593798375464985763947932753984769845769847569837659845769845679845768457694569485765464543".chars()) < BigNum::extract(&mut "23945763247983765498465937983754649857639479327539847698457698475698376598457698456798457684576945694857654645467".chars()));
}
