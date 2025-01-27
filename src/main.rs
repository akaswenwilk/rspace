use clap::{Parser, Subcommand};
pub mod clone;
pub mod config;
pub mod error;
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

    match res {
        Ok(s) => {
            println!("{}", s);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
