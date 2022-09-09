use anyhow::{anyhow, Context, Result};

use clap::Parser;
use grrs::{find_matches, ERROR_PATTERN_EMPTY};

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let writer = &mut std::io::stdout();
    if &args.pattern == "" {
        return Err(anyhow!(ERROR_PATTERN_EMPTY));
    }

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?;

    return find_matches(&args.pattern, &content, writer);
}
