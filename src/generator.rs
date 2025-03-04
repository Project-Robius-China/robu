use crate::cli::{Framework, Terminal};
use anyhow::Result;
use cargo_generate::{generate, GenerateArgs, TemplatePath, Vcs};
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;
use crate::utils::print::{print_completion_banner, print_clickable_link};

pub struct CreationOptions {
    pub ui: Framework,
    pub terminal: Terminal,
    pub name: String,
}

pub fn generator(option: CreationOptions) -> Result<()> {
    match option.ui {
        Framework::Makepad => {
            match option.terminal {
                Terminal::Client => {
                    generate(GenerateArgs {
                        name: Some(option.name.clone()),
                        vcs: Some(Vcs::Git),
                        template_path: TemplatePath {
                            git: Some("https://github.com/Guocork/template".to_string()),
                            ..TemplatePath::default()
                        },
                        ..GenerateArgs::default()
                    })?;
                },
                Terminal::Mobile => {
                    println!("Mobile terminal is not supported for makepad framework");
                },
            }
        }
    }

    // 不同的线程是最好的 要不然用户用完还得等
    let pb = ProgressBar::new(100);
    
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(30));
        pb.inc(1);
    }
    
    pb.finish_and_clear();

    print_completion_banner(&option.name)?;

    print_clickable_link(&option.name)?;

    Ok(())
}