#[derive(Copy, Clone)]
pub enum Condition {
    New(New),
    Used(Used),
    Refurbished,
}

#[derive(Copy, Clone)]
pub enum New {
    Unspecified,
    WithoutTags,
    WithTags,
    AsGood,
}

#[derive(Copy, Clone)]
pub enum Used {
    Unspecified,
    VeryGood,
    Good,
    Satisfactory,
    NotFullyFunctional,
}
