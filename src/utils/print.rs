use console::{style, Term, Style, Emoji};
use anyhow::Result;
use figlet_rs::FIGfont;

pub fn print_completion_banner(project_name: &str) -> Result<()> {

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Makepad");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());


    let term = Term::stdout();
    term.write_line("")?;
    
    let width = project_name.len() + 55;
    let border = "=".repeat(width);
    let thin_border = "-".repeat(width);
    let space_line = format!("||{}||", " ".repeat(width - 4));
    
    // 创建一个居中的文本行
    let create_centered_line = |text: &str, fill: char, edge: &str| -> String {
        let text_len = text.len();
        let fill_len = (width - text_len - 4) / 2;
        let left_fill = fill.to_string().repeat(fill_len);
        let right_fill = if (width - text_len - 4) % 2 == 0 {
            fill.to_string().repeat(fill_len)
        } else {
            fill.to_string().repeat(fill_len + 1)
        };
        format!("{}{}{}{}{}",edge, left_fill, text, right_fill, edge)
    };
    
    // 打印多行艺术文字
    term.write_line(&style(border.clone()).green().to_string())?;
    term.write_line(&style(space_line.clone()).green().to_string())?;
    term.write_line(&style(create_centered_line(" ", '*', "||")).green().to_string())?;
    term.write_line(&style(create_centered_line(&format!("PROJECT {} CREATED", project_name.to_uppercase()), ' ', "||")).green().bold().to_string())?;
    term.write_line(&style(create_centered_line(" ", '*', "||")).green().to_string())?;
    term.write_line(&style(space_line.clone()).green().to_string())?;
    term.write_line(&style(thin_border.clone()).green().to_string())?;
    term.write_line(&style(create_centered_line("Build with Robius CLI", '-', "//")).cyan().to_string())?;
    term.write_line(&style(create_centered_line(&format!("Version: {}", env!("CARGO_PKG_VERSION")), '-', "\\\\")).cyan().to_string())?;
    term.write_line(&style(thin_border.clone()).green().to_string())?;
    term.write_line(&style(space_line.clone()).green().to_string())?;
    term.write_line(&style(create_centered_line("To get started, run:", ' ', "||")).green().to_string())?;
    term.write_line(&style(create_centered_line(&format!("cd {} && cargo run", project_name), ' ', "||")).yellow().bold().to_string())?;
    term.write_line(&style(space_line.clone()).green().to_string())?;
    term.write_line(&style(border.clone()).green().to_string())?;
    
    Ok(())
}


pub fn print_clickable_link(name: &str) -> Result<()> {
    let stdout = Term::stdout();
    let url = "https://book.makepad.rs/";
    
    // 清晰的成功消息
    stdout.write_line("")?;
    stdout.write_line(&format!("{} project '{}' created !", 
                             Emoji("✅", ""),
                             Style::new().green().bold().apply_to(name)))?;
    stdout.write_line("")?;
    
    // 打印彩色链接
    print!("{}  ", Emoji("📖", ""));
    print!("{}", Style::new().green().apply_to("Tutorial doc: "));
    print!("{}", Style::new().green().apply_to(url));
    println!("\u{001B}]8;;\u{001B}\\");

    Ok(())
}