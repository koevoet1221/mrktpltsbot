use itertools::Itertools;

use crate::marketplace::SearchToken;

/// Search token.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Include(&'a str),
    Exclude(&'a str),
}

pub struct Tokens<'a>(Vec<Token<'a>>);

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(text: &'a str) -> Self {
        Self(
            text.split_whitespace()
                .map(|token| {
                    token
                        .strip_prefix('-')
                        .map_or(SearchToken::Include(token), SearchToken::Exclude)
                })
                .collect(),
        )
    }
}

impl Tokens<'_> {
    pub fn to_search_text(&self) -> String {
        self.0
            .iter()
            .filter_map(|token| match token {
                SearchToken::Include(token) => Some(token),
                SearchToken::Exclude(_) => None,
            })
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_query_to_tokens_ok() {
        let tokens = Tokens::from("-samsung smartphone");
        assert_eq!(
            tokens.0,
            &[SearchToken::Exclude("samsung"), SearchToken::Include("smartphone")]
        );
    }
}
