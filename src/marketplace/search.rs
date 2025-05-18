use std::{borrow::Cow, collections::BTreeSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct NormalisedQuery {
    include: BTreeSet<String>,
    exclude: BTreeSet<String>,
}

impl NormalisedQuery {
    pub fn parse(text: &str) -> Self {
        let mut this = Self { include: BTreeSet::new(), exclude: BTreeSet::new() };
        for token in text.split_whitespace().sorted() {
            let token = token.to_lowercase();
            if let Some(token) = token.strip_prefix('-') {
                this.exclude.insert(token.to_string());
            } else {
                this.include.insert(token);
            }
        }
        this
    }

    pub fn unparse(&self) -> String {
        let positive = self.include.iter().map(Cow::Borrowed);
        let negative = self.exclude.iter().map(|token| Cow::<String>::Owned(format!("-{token}")));
        positive.chain(negative).join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let query = NormalisedQuery::parse("-samsung smartphone");
        assert_eq!(query.include.iter().collect_vec(), &["smartphone"]);
        assert_eq!(query.exclude.iter().collect_vec(), &["samsung"]);
    }

    #[test]
    fn to_string_ok() {
        let query = NormalisedQuery::parse("-samsung smartphone");
        assert_eq!(query.unparse(), "smartphone -samsung");
    }
}
