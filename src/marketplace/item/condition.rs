#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    New(New),
    Used(Used),
    Refurbished,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum New {
    Unspecified,
    WithoutTags,
    WithTags,
    AsGood,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Used {
    Unspecified,
    VeryGood,
    Good,
    Satisfactory,
    NotFullyFunctional,
}
