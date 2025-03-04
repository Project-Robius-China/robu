use clap::{Parser, Subcommand, ValueEnum};


#[derive(Parser)]
#[command(
    version,
    about,
    // args_conflicts_with_subcommands = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create {
        #[arg(
            help = "The framework to use",
            ignore_case = true,
        )]
        ui: Option<Framework>,

        #[arg(
            help = "The terminal to use",
            ignore_case = true,
        )]
        terminal: Option<Terminal>,

        #[arg(
            help = "The name of the project to create"
        )]
        name: Option<String>,
    },
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Framework {
    Makepad,
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Terminal {
    Client,
    Mobile,
}

