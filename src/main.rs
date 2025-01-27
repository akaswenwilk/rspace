use clap::{Parser, Subcommand};
pub mod config;
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

    let conf = config::load();

    let res = match args.cmd {
        Commands::New => new::run(conf),
        Commands::Purge => purge::run(conf),
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
