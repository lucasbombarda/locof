use clap::Parser;
use encoding::{all::UTF_8, DecoderTrap, Encoding};
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

fn count_lines(file_path: String) -> usize {
    let mut file = File::open(file_path).unwrap();
    let mut buff = Vec::new();
    file.read_to_end(&mut buff).unwrap();
    let res = UTF_8.decode(&buff, DecoderTrap::Ignore);
    res.unwrap().lines().count()
}

fn exec(path: String) -> Result<()> {
    println!("Counting...\n");
    let mut res: HashMap<String, usize> = HashMap::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path = entry.path().display().to_string();
        if let Some(file_type) = entry.path().extension() {
            let lines = count_lines(path);
            res.entry(file_type.to_str().unwrap().to_string())
                .and_modify(|qty| *qty += lines)
                .or_insert(lines);
        }
    }

    println!("Total raw LOC: {}\n", res.values().sum::<usize>());
    res.iter().for_each(|(k, v)| {
        println!("{k}: {v}");
    });

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    exec(cli.path)?;
    Ok(())
}
