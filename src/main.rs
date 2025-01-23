use clap::{Parser, Subcommand};
pub mod new;
pub mod purge;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    New,
    Purge,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::New => new::run(),
        Commands::Purge => purge::run(),
    }
}
