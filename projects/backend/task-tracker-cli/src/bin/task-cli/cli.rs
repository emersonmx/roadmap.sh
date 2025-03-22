use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "task-cli", about = "A simple task tracker")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Add {
        description: String,
    },
    List {
        #[arg(value_parser = ["done", "todo", "in-progress"])]
        status: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
