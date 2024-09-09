use clap::Parser;

mod cli;
use cli::Cli;

fn main() {
    match Cli::parse().commands.execute() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
