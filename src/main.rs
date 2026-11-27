#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "linux-mint-ref", about = "SigmaOS Linux Mint reference")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("linux-mint-ref: no entries configured."),
        Commands::Status => println!("linux-mint-ref: ready."),
    }
}
