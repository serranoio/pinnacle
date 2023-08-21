use clap:: {
    Args,
    Parser,
    Subcommand
};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PinnacleArgs{
    #[clap(subcommand)]
    pub commands: Subcommands,
}


#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Create a new pinnacle project
    New(NewProject),
}

#[derive(Debug, Args)]
pub struct NewProject {
    /// Name of the new project
    pub name: String,

     /// spread conents into current directory
     #[arg(short, long, default_value_t=false)]
    pub new_directory: bool,
}



