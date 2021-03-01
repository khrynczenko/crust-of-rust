#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimeter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimeter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

pub trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D> 
    where D: Delimeter,
    //where 'delimeter: 'haystack,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = &mut self.remainder?;

        if let Some((delim_start, delim_end)) = self.delimeter.find_next(remainder) {
            let until_delimeter = &remainder[..delim_start];
            self.remainder = Some(&remainder[delim_end..]);
            Some(until_delimeter)
            // Some(delimeter) //would work with where clause
        } else {
            self.remainder.take()
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, format!("{}", c).as_str())
        .next()
        .expect("StrSplit always gives at least  one result.")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn it_works2() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
