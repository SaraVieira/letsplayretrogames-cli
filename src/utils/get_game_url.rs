use super::constants::DOMAIN;

pub fn get_game_url(console_id: &str, slug: &str) -> String {
    format!("{DOMAIN}/{console_id}/{slug}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_url() {
        let result = get_game_url("nes", "one-two");
        assert_eq!(
            result,
            "https://letsplayretro.games/nes/one-two".to_string()
        )
    }
}
