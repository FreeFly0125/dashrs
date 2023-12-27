//! Module exposing a ['memchr'] based iterator for splitting strings

use memchr::memmem;

#[derive(Debug)]
pub struct Split<'de> {
    input: &'de str,
    delimiter: &'static str,
    finder: memmem::FindIter<'de, 'static>,
    end_of_last_delimiter: Option<usize>,
}

impl<'de> Split<'de> {
    pub fn new(haystack: &'de str, needle: &'static str) -> Self {
        Split {
            input: haystack,
            delimiter: needle,
            finder: memmem::find_iter(haystack.as_ref(), needle),
            end_of_last_delimiter: Some(0),
        }
    }

    pub fn position(&self) -> usize {
        self.end_of_last_delimiter.unwrap_or(self.input.len())
    }
}

impl<'de> Iterator for Split<'de> {
    type Item = &'de str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.end_of_last_delimiter {
            Some(end_of_last) => match self.finder.next() {
                Some(next) => {
                    self.end_of_last_delimiter = Some(next + self.delimiter.len());
                    Some(&self.input[end_of_last..next])
                },
                None => {
                    self.end_of_last_delimiter = None;
                    Some(&self.input[end_of_last..])
                },
            },
            None => None,
        }
    }
}
