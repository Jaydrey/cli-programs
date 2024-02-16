mod project;
mod django_args;
mod app;
mod env_file;

use clap::Parser;
use django_args::{DjangoArgs, DocumentType};
use crate::project::commands::handle_project_command;

fn main() {
    let args = DjangoArgs::parse();
    println!("{:?}", args);

    match args.document_type{
        DocumentType::Project(project) =>{
            handle_project_command(project);
        } ,
        DocumentType::App(_app) => {},
        DocumentType::EnvFile(_env_file) => {},
    };
}
