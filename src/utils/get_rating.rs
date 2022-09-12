pub fn get_rating(total_rating: Option<f32>) -> String {
    total_rating
        .map(|rating| format!("{rating:.2}"))
        .unwrap_or_else(|| "Unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_exists() {
        let result = get_rating(Some(78.998));
        assert_eq!(result, "79.00")
    }

    #[test]
    fn rating_not_exists() {
        let result = get_rating(None);
        assert_eq!(result, "Unknown".to_string())
    }
}
