use anyhow::Result;
use dialoguer::{console::Term, theme::ColorfulTheme, Select, Input};
use console::Style;
use crate::cli::{Framework, Terminal};
use crate::generator::{CreationOptions, generator};


pub fn args_parser(
    ui: Option<Framework>,
    terminal: Option<Terminal>,
    project_name: Option<String>,
) -> Result<()> {

    let theme = ColorfulTheme {
        prompt_style: Style::new().green().bold(),
        ..ColorfulTheme::default()
    };
    let ui = match ui {
        Some(ui) => ui,
        None => {
            let item = vec!["makepad"];
            let selection = Select::with_theme(&theme)
                .with_prompt("Select a framework, use up/down arrow keys to select a framework")
                .items(&item)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(0) => Framework::Makepad,
                _ => Framework::Makepad,
                
            }
        },
        
    };

    let terminal = match terminal {
        Some(terminal) => terminal,
        None => {
            let item = vec!["client", "mobile"];
            let selection = Select::with_theme(&theme)
                .with_prompt("Select a terminal to display, use up/down arrow keys to select a terminal")
                .items(&item)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(0) => Terminal::Client,
                Some(1) => Terminal::Mobile,
                _ => Terminal::Client,
            }
        },
    };

    let project_name = match project_name {
        Some(project_name) => project_name,
        None => {
            let project_name = Input::with_theme(&theme)
                .with_prompt("project name")
                .interact_text()?;
            project_name
        },
    };

    generator(CreationOptions {
        ui,
        terminal,
        name: project_name,
    })?;

    Ok(())
}