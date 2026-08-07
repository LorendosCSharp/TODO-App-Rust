pub mod sort;

use clap::{Parser, Subcommand};

use crate::cli::sort::SortType;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///Adds a new task
    Add {
        ///Name for the task
        name: String,

        ///Description of the task
        description: String,

        ///Date due task needs to be done
        till: String,
    },

    ///Removes one task
    Rem {
        ///ID of the task
        id: String,
    },
    ///Makes a task done
    Done {
        ///ID of the task
        id: String,
    },

    ///List task following the sorting type
    List {
        ///How to sort: all, done or undone
        sort: Option<SortType>,
    },
}
