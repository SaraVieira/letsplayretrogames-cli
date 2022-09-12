use clap::Parser;

use letsplayretrogames::{get_random_game, get_searched_game, Consoles};

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

fn main() -> () {
    let args = Cli::parse();
    match args.command {
        Command::Search(command) => get_searched_game(&command.query),
        Command::Random(command) => get_random_game(&command.console),
    };
}
