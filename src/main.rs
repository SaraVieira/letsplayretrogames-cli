use std::fmt::format;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use comfy_table::Table;
use serde::{Deserialize, Serialize};

enum CONSOLES {
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
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    id: i32,
    igdb_id: i32,
    first_release_date: i32,
    total_rating: Option<f32>,
    name: String,
    slug: String,
    console: String,
}

#[derive(Parser)]
struct SearchParams {
    query: String,
}

#[derive(Parser)]
struct RandomParams {
    console: String,
    query: String,
}

#[derive(clap::Subcommand)]
enum Command {
    Search(SearchParams),
    Random(RandomParams),
}
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn get_searched_game(query: &str) -> Result<(), anyhow::Error> {
    let mut table = Table::new();
    table.set_header(vec!["Name", "Console", "Rating", "Link"]);
    let result = reqwest::get(
        "https://letsplayretro.games/api/search?query=".to_owned() + &query.to_owned(),
    )
    .await?
    .json::<Vec<Data>>()
    .await?;

    for game in result {
        let url = "https://letsplayretro.games/".to_owned()
            + &game.console.to_owned()
            + "/"
            + &game.slug.to_owned();

        let rating = match game.total_rating {
            Some(i) => format!("{:.2}", i.to_string()),
            _ => "Unknown".to_string(),
        };
        table.add_row(vec![&game.name, &game.console, &rating.to_string(), &url]);
    }
    println!("{table}");
    Ok(())
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Search(command) => get_searched_game(&command.query),
    };
}

// - letsplay search mario
// - letsplay random nes
