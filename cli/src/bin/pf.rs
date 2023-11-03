use passionfruitdev_cli::cli::*;

fn main() {
    let cli = Cli::parse();
    execute(cli)
}
