use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    pub command: ProjectSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubCommand {
    /// create project
    Create(CreateProject),
}

#[derive(Debug, Args)]
pub struct CreateProject {
    /// The name of the project
    pub name: String,
}

// create virtualenv and project