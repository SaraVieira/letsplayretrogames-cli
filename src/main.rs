pub mod api;
pub mod utils;

use clap::Parser;

use api::{get_random_game, get_searched_games, Consoles};

#[derive(Parser)]
struct SearchParams {
    query: String,
}

#[derive(Parser)]
struct RandomParams {
    #[clap(arg_enum)]
    console: Option<Consoles>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Search for a game passing a query
    Search(SearchParams),
    /// Get a random game, pass a console for games in that console or leave empty and be surprised
    Random(RandomParams),
}
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    match args.command {
        Command::Search(command) => get_searched_games(&command.query).await,
        Command::Random(command) => get_random_game(&command.console).await,
    }
}
