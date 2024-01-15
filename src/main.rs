use prettytable::Table;
#[macro_use]
extern crate prettytable;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::{env, fs};

const SIZE_LIMIT: usize = 80;
const COUNT_LIMIT: usize = 10;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let custom_file_name = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    let history_file_path = get_history_file_path(custom_file_name)?;
    let file = fs::File::open(history_file_path)?;
    let reader = io::BufReader::new(file);

    let mut count = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let item = line.split(';').last().unwrap_or_default().to_string();
        let trimmed = trim(&item);
        *count.entry(trimmed).or_insert(0) += 1;
    }

    let mut rows: Vec<_> = count
        .into_iter()
        .filter(|&(_, v)| v > COUNT_LIMIT)
        .collect();

    rows.sort_by(|a, b| a.1.cmp(&b.1));

    render_table(rows);

    Ok(())
}

fn render_table(commands: Vec<(String, usize)>) {
    let mut table = Table::new();

    for (command, count) in commands {
        table.add_row(row![command, count]);
    }
    table.add_row(row!["Command", "Count"]);
    table.printstd();
}

fn trim(item: &str) -> String {
    let item = item.trim();
    match item.len() {
        len if len == SIZE_LIMIT => item.chars().take(SIZE_LIMIT).collect(),
        len if len > SIZE_LIMIT => format!("{}...", &item[..SIZE_LIMIT]),
        _ => item.to_string(),
    }
}

fn get_history_file_path(file_name: Option<String>) -> io::Result<String> {
    match file_name {
        Some(name) => {
            let file_path = PathBuf::from(&name);
            if file_path.exists() {
                Ok(file_path.to_str().unwrap().to_string())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File {:?} does not exist", name),
                ))
            }
        }
        None => {
            let home_dir = env::var("HOME").expect("HOME environment variable not set");
            let zsh_history = PathBuf::from(&home_dir).join(".zsh_history");
            let bash_history = PathBuf::from(&home_dir).join(".bash_history");

            if zsh_history.exists() {
                Ok(zsh_history.to_str().unwrap().to_string())
            } else if bash_history.exists() {
                Ok(bash_history.to_str().unwrap().to_string())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Missing both zsh and bash history files",
                ))
            }
        }
    }
}
