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
        #[arg(short, long, value_enum, value_enum, default_value_t = Architecture::Aarch64AppleDarwin)]
        architecture: Architecture,
        #[arg(long, value_enum, value_enum, default_value_t = OperatingSystem::MacOS)]
        os: OperatingSystem
    },
    New {
        name: String
    }
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Architecture {
    Aarch64AppleDarwin
}

#[derive(ValueEnum, Debug, Clone)]
enum OperatingSystem {
    MacOS
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Install { architecture, os } => {
            match (architecture,os) {
                (Architecture::Aarch64AppleDarwin, OperatingSystem::MacOS) => { println!("Installing Aarch64AppleDarwin on MacOS dependencies"); }
            }
        }
        Command::New { name } => {
            println!("Creating project: {}",name)
        }
    }
}