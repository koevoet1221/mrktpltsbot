use crate::prelude::*;

lazy_static! {
    pub static ref CLEAR_REGEX: Regex = Regex::new(r"[\.\(\)💥]+").unwrap();
}

pub fn tokenize(text: &str) -> HashSet<String> {
    CLEAR_REGEX
        .replace_all(text, " ")
        .replace("é", "e")
        .split_whitespace()
        .map(str::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;

    #[test]
    fn tokenize_ok() {
        assert_eq!(
            tokenize("Tqi zender traxxas"),
            hashset! {"tqi".into(), "zender".into(), "traxxas".into()},
        );
        assert_eq!(
            tokenize("stans nr. 123"),
            hashset! {"stans".into(), "nr".into(), "123".into()},
        );
        assert_eq!(
            tokenize("Nike schoenen maat 23.5 (valt als 22)"),
            hashset! {"nike".into(), "schoenen".into(), "maat".into(), "23".into(), "5".into(), "valt".into(), "als".into(), "22".into()},
        );
        assert_eq!(
            tokenize("💥Behringer BG412F"),
            hashset! {"behringer".into(), "bg412f".into()}
        );
    }
}
