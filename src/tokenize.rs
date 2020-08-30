use crate::prelude::*;

lazy_static! {
    static ref CLEAR_REGEX: Regex = Regex::new(r"[,\*\.\(\)€/’'\+\-💥✅⭐🚴🌿💿🏅🤩]+").unwrap();
}

pub fn tokenize(text: &str) -> Vec<String> {
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

    #[test]
    fn tokenize_ok() {
        assert_eq!(
            tokenize("Tqi zender traxxas"),
            vec![
                "tqi".to_string(),
                "zender".to_string(),
                "traxxas".to_string()
            ],
        );
        assert_eq!(
            tokenize("stans nr. 123"),
            vec!["stans".to_string(), "nr".to_string(), "123".to_string()],
        );
        assert_eq!(
            tokenize("Nike schoenen maat 23.5 (valt als 22)"),
            vec![
                "nike".to_string(),
                "schoenen".to_string(),
                "maat".to_string(),
                "23".to_string(),
                "5".to_string(),
                "valt".to_string(),
                "als".to_string(),
                "22".to_string()
            ],
        );
        assert_eq!(
            tokenize("💥Behringer BG412F"),
            vec!["behringer".to_string(), "bg412f".to_string()],
        );
    }
}
