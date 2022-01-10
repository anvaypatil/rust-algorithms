//#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]

#[derive(Debug)]
pub struct StrSplit<'haystack, DelimType> {
    remainder: Option<&'haystack str>,
    delimiter: DelimType,
}

impl<'haystack, DelimType> StrSplit<'haystack, DelimType> {
    pub fn new(haystack: &'haystack str, delimiter: DelimType) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self)
            .map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<'haystack, StringType> Iterator for StrSplit<'haystack, StringType>
    where StringType: Delimiter {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // other alternate statements
        // let ref mut remainder = self.remainder?;
        // let remainder = self.remainder.as_mut()?;
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit: no char found")
}


#[cfg(test)]
mod tests {
    use crate::{StrSplit, until_char};

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }

    #[test]
    fn basic_working() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail_test() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }
}
