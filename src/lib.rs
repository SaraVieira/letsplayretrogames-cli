use std::fmt;

use comfy_table::Table;
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: i32,
    igdb_id: i32,
    first_release_date: i32,
    total_rating: Option<f32>,
    name: String,
    slug: String,
    console: String,
    console_id: String,
}

#[derive(clap::ArgEnum, Clone)]
pub enum Consoles {
    Nes,
    Snes,
    Gb,
    Gbc,
    Gba,
    N64,
    Md,
    Gg,
    Ms,
    Pce,
}

impl fmt::Display for Consoles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Consoles::Nes => write!(f, "nes"),
            Consoles::Snes => write!(f, "snes"),
            Consoles::Gb => write!(f, "gb"),
            Consoles::Gbc => write!(f, "gbc"),
            Consoles::Gba => write!(f, "gba"),
            Consoles::N64 => write!(f, "n64"),
            Consoles::Md => write!(f, "md"),
            Consoles::Gg => write!(f, "gg"),
            Consoles::Ms => write!(f, "ms"),
            Consoles::Pce => write!(f, "pce"),
        }
    }
}

fn create_table() -> Table {
    let mut table = Table::new();
    table.set_header(vec!["Name", "Console", "Rating", "Link"]);
    table
}

fn get_rating(total_rating: Option<f32>) -> String {
    match total_rating {
        Some(i) => format!("{:.2}", i),
        _ => "Unknown".to_string(),
    }
}

#[test]

fn rating_exists() {
    let result = get_rating(Option::Some(78.998));
    assert_eq!(result, "79.00")
}

#[test]

fn rating_not_exists() {
    let result = get_rating(Option::None);
    assert_eq!(result, "Unknown".to_string())
}

fn get_game_url(console_id: String, slug: String) -> String {
    DOMAIN.to_owned() + &console_id + "/" + &slug
}

#[test]

fn game_url() {
    let result = get_game_url("nes".to_string(), "one-two".to_string());
    assert_eq!(
        result,
        "https://letsplayretro.games/nes/one-two".to_string()
    )
}

#[tokio::main]
pub async fn get_searched_games(query: &str) -> Result<(), anyhow::Error> {
    let mut table = create_table();
    let mut sp = Spinner::new(Spinners::Dots9, "Fetching your games".into());
    let result = reqwest::get("https://letsplayretro.games/api/search?query=".to_owned() + query)
        .await?
        .json::<Vec<Game>>()
        .await?;

    for game in result {
        table.add_row(vec![
            &game.name,
            &game.console.to_owned(),
            &get_rating(game.total_rating),
            &get_game_url(game.console_id, game.slug),
        ]);
    }
    sp.stop_and_persist("✔", "Found them!".into());
    println!("{table}");
    Ok(())
}

const DOMAIN: &str = "https://letsplayretro.games/";

#[tokio::main]
pub async fn get_random_game(console: &Option<Consoles>) -> Result<(), anyhow::Error> {
    let mut table = create_table();
    let mut sp = Spinner::new(Spinners::Dots9, "Choosing a random game for you".into());
    let url = match console {
        Some(i) => DOMAIN.to_owned() + "api/" + &i.to_string() + "/random",
        _ => DOMAIN.to_owned() + "api/" + "/random",
    };

    let game = reqwest::get(url).await?.json::<Game>().await?;

    table.add_row(vec![
        &game.name,
        &game.console.to_owned(),
        &get_rating(game.total_rating),
        &get_game_url(game.console_id, game.slug),
    ]);
    sp.stop_and_persist("✔", "Found it!".into());
    println!("{table}");

    Ok(())
}
