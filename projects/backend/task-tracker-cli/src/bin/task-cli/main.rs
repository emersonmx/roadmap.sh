use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    match Cli::parse() {
        Cli::Add { .. } => {
            println!("Task added successfully (ID: 1)");
        }
        Cli::List { .. } => {
            println!("TODO");
            println!("IN PROGRESS");
            println!("DONE");
        }
    }
}
