use crate::cli::*;
pub mod cli;

fn main() {
    let cli = cli::Cli::parse();
    cli::execute(cli)
}
