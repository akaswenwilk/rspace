use clap::{Parser, Subcommand};
pub mod new;
pub mod purge;
pub mod config;

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

    let conf = config::load();

    match args.cmd {
        Commands::New => new::run(conf),
        Commands::Purge => purge::run(conf),
    }
}
