use clap::Parser;

use letsplayretrogames::{get_random_game, get_searched_game};

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

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Search(command) => get_searched_game(&command.query),
        Command::Random(command) => get_random_game(&command.console),
    };
}

// - letsplay random nes
