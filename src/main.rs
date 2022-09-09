use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use serde::{Deserialize, Serialize};

// enum CONSOLES {
//     Nes,
//     Snes,
//     Gb,
//     Gbc,
//     Gba,
//     N64,
//     Md,
//     Gg,
//     Ms,
//     Pce,
// }
#[derive(Debug, Deserialize, Serialize)]
struct Game {
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
    .json::<Vec<Game>>()
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

#[tokio::main]
async fn get_random_game(console: &str) -> Result<(), anyhow::Error> {
    let url =
        "https://letsplayretro.games/api/".to_owned() + &console.to_owned() + &"/random".to_owned();
    let game = reqwest::get(url).await?.json::<Game>().await?;
    let url = "https://letsplayretro.games/".to_owned()
        + &game.console.to_owned()
        + "/"
        + &game.slug.to_owned();
    println!("{:#?}", game.name);
    println!("{:#?}", url);
    Ok(())
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Search(command) => get_searched_game(&command.query),
        Command::Random(command) => get_random_game(&command.console),
    };
}

// - letsplay random nes
