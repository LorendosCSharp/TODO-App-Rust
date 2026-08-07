mod cli;
mod commands;
mod data;
mod task;

use clap::Parser;
use data::Data;

use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let mut data = Data::load();
    match cli.command {
        Commands::Add {
            name,
            description,
            till,
        } => {
            commands::add::execute(&mut data, name, description, till);
        }
        Commands::Rem { id } => {
            commands::rem::execute(&mut data, id);
        }
        Commands::List { sort } => {
            commands::list::execute(&data, sort);
        }
        Commands::Done { id } => {
            commands::done::execute(&mut data, id);
        }
    }

    Data::save(&data);
}
