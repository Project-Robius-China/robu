use anyhow::Result;
use robu::cli::{Cli, Commands};
use robu::args::args_parser;
use clap::Parser;



fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { ui, terminal, name }) => {
            args_parser(ui, terminal, name)?;
        }
        None => {
            println!("No command provided");
        }
    }
    
    Ok(())
}
