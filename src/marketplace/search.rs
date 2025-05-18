/// Search token.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Include(&'a str),
    Exclude(&'a str),
}
