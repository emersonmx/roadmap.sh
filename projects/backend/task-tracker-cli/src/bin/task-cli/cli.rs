use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "task-cli", about = "A simple task tracker")]
pub enum Cli {
    /// Add a new task
    Add { description: String },
    /// List tasks
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
