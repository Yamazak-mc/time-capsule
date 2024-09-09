use clap::Parser;
use std::error::Error;

mod cli;
use cli::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    Cli::parse().commands.execute()
}
