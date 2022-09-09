use comfy_table::Table;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: i32,
    igdb_id: i32,
    first_release_date: i32,
    total_rating: Option<f32>,
    name: String,
    slug: String,
    console: String,
}

#[tokio::main]
pub async fn get_searched_game(query: &str) -> Result<(), anyhow::Error> {
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
pub async fn get_random_game(console: &str) -> Result<(), anyhow::Error> {
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
