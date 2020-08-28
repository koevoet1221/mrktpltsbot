use crate::prelude::*;

pub fn tokenize(text: &str) -> HashSet<String> {
    text.replace(".", "")
        .replace("(", "")
        .replace(")", "")
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
            hashset! {"tqi".to_string(), "zender".to_string(), "traxxas".to_string()},
        );
        assert_eq!(
            tokenize("stans nr. 123"),
            hashset! {"stans".to_string(), "nr".to_string(), "123".to_string()},
        );
        assert_eq!(
            tokenize("Nike schoenen maat 23.5 (valt als 22)"),
            hashset! {"nike".to_string(), "schoenen".to_string(), "maat".to_string(), "235".to_string(), "valt".to_string(), "als".to_string(), "22".to_string()},
        );
    }
}
