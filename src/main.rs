use std::{
    io::{self, Write},
    process::Command,
};
extern crate prettytable;
use prettytable::{color, format::Alignment, Attr, Cell, Row, Table};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'q', long, default_value_t = String::from(""))]
    query: String,
}

fn exec_command() -> String {
    if cfg!(target_os = "windows") {
        panic!("Could Not Operate On Windows")
    }

    let command = Command::new("sh")
        .arg("-c")
        .arg("echo $PATH")
        .output()
        .expect("Failed to Execute");

    String::from_utf8_lossy(&command.stdout).to_string()
}

fn empty_path_error(query: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

    write!(
        &mut stdout,
        "Oops! None of the Environment Variables Contains The Query "
    )?;

    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bg(Some(Color::Red)),
    )?;
    write!(&mut stdout, "{}", query)?;

    stdout.reset()?;
    writeln!(&mut stdout)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let mut search_query: Option<String> = None;
    if !args.query.is_empty() {
        search_query = Some(args.query.to_string());
    }

    let stdout_str = exec_command();
    let mut paths: Vec<_> = stdout_str
        .split(":/")
        .map(|path| {
            let path = path.trim_start_matches('/');
            format!("/{}", path)
        })
        .collect();

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Count").with_style(Attr::Bold),
        Cell::new("Path").with_style(Attr::Bold),
    ]));

    if let Some(q) = search_query {
        paths.retain(|p| p.contains(&q))
    };

    if paths.is_empty() {
        empty_path_error(&args.query).unwrap();
        return;
    }

    for (idx, path) in paths.iter().enumerate() {
        let path_cell = Cell::new(path);
        let count_cell = Cell::new_align(&(idx + 1).to_string(), Alignment::CENTER)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED));
        table.add_row(Row::new(vec![count_cell, path_cell]));
    }

    table.printstd();
}
