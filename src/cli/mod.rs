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
    ///Modify existing task
    Modify {
        ///ID of the task
        id: String,

        ///Name for the task
        #[arg(short, long)]
        name: Option<String>,

        ///Description of the task
        #[arg(short, long)]
        description: Option<String>,

        ///Date due task needs to be done
        #[arg(short, long)]
        till: Option<String>,

        ///Status of the task, done or not
        #[arg(short, long)]
        status: Option<String>,
    },

    ///Searches for a task based on the parameters
    Search {
        ///ID of the task
        #[arg(short, long)]
        id: Option<String>,

        ///Name for the task
        #[arg(short, long)]
        name: Option<String>,

        ///Date due task needs to be done
        #[arg(short, long)]
        till: Option<String>,

        #[arg(short, long)]
        status: Option<String>,
    },
}
