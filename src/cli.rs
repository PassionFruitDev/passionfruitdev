use chrono::{Datelike, Timelike, Utc};
use clap::{Subcommand, ValueEnum};
use std::process::Command as RunCommand;
use std::{fmt, fs};
use toml_edit::{value, Document};

pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Install {
        #[arg(short, long, value_enum, default_value_t = Architecture::Aarch64AppleDarwin)]
        architecture: Architecture,
        #[arg(long, value_enum, value_enum, default_value_t = OperatingSystem::MacOS)]
        os: OperatingSystem,
    },
    New {
        name: String,
    },
    Publish {
        #[arg(short, long, value_enum, default_value_t = Lifecycle::Manual)]
        lifecycle: Lifecycle,
    },
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Architecture {
    Aarch64AppleDarwin,
}

#[derive(ValueEnum, Debug, Clone)]
enum OperatingSystem {
    MacOS,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
enum Lifecycle {
    Manual,
    Pull,
    Merge,
    Nightly,
    Candidate,
    Official,
}

impl fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lifecycle::Manual => write!(f, "manual"),
            Lifecycle::Pull => write!(f, "pull"),
            Lifecycle::Merge => write!(f, "merge"),
            Lifecycle::Nightly => write!(f, "nightly"),
            Lifecycle::Candidate => write!(f, "rc"),
            Lifecycle::Official => write!(f, "official"),
        }
    }
}

pub fn execute(cli: Cli) {
    match &cli.command {
        Command::Install { architecture, os } => match (architecture, os) {
            (Architecture::Aarch64AppleDarwin, OperatingSystem::MacOS) => {
                println!("Installing Aarch64AppleDarwin on MacOS dependencies");
            }
        },
        Command::New { name } => {
            println!("Creating project: {}", name)
        }
        Command::Publish { lifecycle } => publish(lifecycle),
    }
}

fn publish(lifecycle: &Lifecycle) {
    let datetime = Utc::now();
    let datetime = format!(
        "{:02}{:02}{:02}{:02}{:02}{:02}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second()
    );
    let git_hash = RunCommand::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(git_hash.stdout).unwrap();
    let git_hash = git_hash.lines().next().unwrap();

    let dryrun = RunCommand::new("cargo")
        .args(&["publish", "--dry-run"])
        .output()
        .unwrap();
    if dryrun.status.success() {
        let current_directory = std::env::current_dir().unwrap();
        let cargo_toml_path = format!("{}/Cargo.toml", current_directory.display());
        let cargo_toml_backup_path = format!("{}/Cargo.toml.bak", current_directory.display());
        let _ = RunCommand::new("cp")
            .args(&[&cargo_toml_path, &cargo_toml_backup_path])
            .output()
            .unwrap();
        let cargo_toml = fs::read_to_string(&cargo_toml_path).unwrap();
        let mut cargo_toml = cargo_toml.parse::<Document>().unwrap();
        let version = cargo_toml["package"]["version"].as_str().unwrap();

        let version = format!(
            "{}-{}.{}+{}",
            &version,
            lifecycle.to_string(),
            &datetime,
            &git_hash
        );
        cargo_toml["package"]["version"] = value(&version);
        let _ = fs::write(&cargo_toml_path, cargo_toml.to_string()).unwrap();
        let _ = RunCommand::new("cargo")
            .args(&["publish", "--allow-dirty"])
            .output()
            .unwrap();
        let _ = RunCommand::new("mv")
            .args(&[&cargo_toml_backup_path, &cargo_toml_path])
            .output()
            .unwrap();
        if lifecycle.to_string() == Lifecycle::Official.to_string() {
            let _ = RunCommand::new("cargo")
                .args(&["publish"])
                .output()
                .unwrap();
            // TODO: Bump semver after official release.
        }
    } else {
        println!(
            "Publish failed dry run with error:\n{}",
            String::from_utf8_lossy(&dryrun.stderr)
        );
    }
    ()
}
