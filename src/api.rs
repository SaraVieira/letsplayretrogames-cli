use super::utils::constants::*;
use super::utils::get_game_url::*;
use super::utils::get_rating::*;
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

#[derive(clap::ArgEnum, Clone, Debug)]
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

fn create_table() -> Table {
    let mut table = Table::new();
    table.set_header(vec!["Name", "Console", "Rating", "Link"]);
    table
}

pub async fn get_searched_games(query: &str) -> Result<(), anyhow::Error> {
    let mut table = create_table();
    let mut sp = Spinner::new(Spinners::Dots9, FETCHING_GAMES.to_string());
    let result = reqwest::get(format!("{DOMAIN}/api/search?query={query}"))
        .await?
        .json::<Vec<Game>>()
        .await?;

    for game in result {
        table.add_row(vec![
            &game.name,
            &game.console.to_owned(),
            &get_rating(game.total_rating),
            &get_game_url(&game.console_id, &game.slug),
        ]);
    }
    sp.stop_and_persist(SUCCESS_SYMBOL, GAMES_FOUND.to_string());
    println!("{table}");
    Ok(())
}

pub async fn get_random_game(console: &Option<Consoles>) -> Result<(), anyhow::Error> {
    let mut table = create_table();
    let mut sp = Spinner::new(Spinners::Dots9, CHOOSING_RANDOM.to_string());
    let url = match console {
        Some(i) => {
            let console_as_string = format!("{:?}", i).to_lowercase();
            format!("{DOMAIN}/api/{console_as_string}/random")
        }
        None => format!("{DOMAIN}/api/random"),
    };
    let game = reqwest::get(url).await?.json::<Game>().await?;

    table.add_row(vec![
        &game.name,
        &game.console.to_owned(),
        &get_rating(game.total_rating),
        &get_game_url(&game.console_id, &game.slug),
    ]);
    sp.stop_and_persist(SUCCESS_SYMBOL, GAME_FOUND.to_string());
    println!("{table}");

    Ok(())
}
