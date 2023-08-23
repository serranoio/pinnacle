use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PinnacleArgs {
    #[clap(subcommand)]
    pub commands: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Create a new pinnacle project
    New(NewProject),
    /// Compile pinnacle
    Build(NewProject),
    /// Download necessary components
    Download,
}

#[derive(Debug, Args)]
pub struct NewProject {
    /// Name of the new project
    pub name: String,

    /// spread conents into current directory
    #[arg(short, long, default_value_t = false)]
    pub new_directory: bool,
}

#[derive(Debug, Args)]
pub struct Build {
    /// spread conents into current directory
    #[arg(short, long, default_value_t = false)]
    pub random_flag: bool,
}
