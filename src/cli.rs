use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file
    Add { task: String },
    /// Remove an entry from the journal file by position
    Done { position: usize },
    /// List all tasks in the journal file
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line todo app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
