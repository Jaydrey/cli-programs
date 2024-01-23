use clap::{
    Parser,
    Subcommand,
};

use crate::{
    env_file::EnvFileCommand,
    app::AppCommand,
    project::project_args::ProjectCommand,
};


#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct DjangoArgs{
    /// type of document to build e.g. project, app, .env,
    #[clap(subcommand)]
    pub document_type: DocumentType,

}

#[derive(Debug, Subcommand)]
pub enum DocumentType {
    /// Project commands
    Project(ProjectCommand),
    // App commands
    App(AppCommand),
    // env files commands
    EnvFile(EnvFileCommand),
}


