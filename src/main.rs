use clap::Parser;
use encoding::{all::UTF_8, DecoderTrap, Encoding};
use simdutf8::basic::from_utf8;
use std::collections::HashMap;
use std::{error::Error, fs::File, io::prelude::*, result};
use walkdir::WalkDir;

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,
}

fn count_lines(file_path: String) -> Option<usize> {
    let mut file = File::open(file_path).unwrap();
    let mut buff = Vec::new();
    file.read_to_end(&mut buff).unwrap();
    // check if it is a valid utf-8 file
    if from_utf8(&buff).is_err() {
        return None;
    }
    let res = UTF_8.decode(&buff, DecoderTrap::Ignore);
    Some(res.unwrap().lines().count())
}

fn exec(path: String) -> Result<()> {
    println!("Counting...\n");
    let mut res: HashMap<String, usize> = HashMap::new();
    let entries = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect::<Vec<_>>();

    let len_entries = entries.len();

    for (idx, entry) in entries.into_iter().enumerate() {
        let path = entry.path().display().to_string();
        if let Some(file_type) = entry.path().extension() {
            if let Some(lines) = count_lines(path) {
                print!("\r{} of {} files", idx + 1, len_entries);
                res.entry(file_type.to_str().unwrap().to_string())
                    .and_modify(|qty| *qty += lines)
                    .or_insert(lines);
            }
        }
    }

    let total = res.values().sum::<usize>();
    let mut v: Vec<_> = res.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\n\nResults:");
    for (ext, lines) in v {
        println!(
            "{}: {} {:.4}%",
            ext,
            lines,
            (lines as f64 / total as f64) * 100.0
        );
    }
    println!("\nTotal raw LOC: {}\n", total);

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    exec(cli.path)?;
    Ok(())
}
