use clap::{ValueEnum, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Install {
        #[arg(long, value_enum, value_enum, default_value_t = Hardware::M2)]
        hardware: Hardware,
    },
    New {
        name: String
    }
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Hardware {
    M2
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Install { hardware } => {
            match hardware {
                Hardware::M2 => { println!("Installing M2 dependencies"); }
            }
        }
        Command::New { name } => {
            println!("Creating project: {}",name)
        }
    }
}