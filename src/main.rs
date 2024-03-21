use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process;
use walkdir::WalkDir;

mod extensions;
use extensions::FileExtension;

struct Results {
    file: FileExtension,
    code_lines: u64,
    blank_lines: u64,
}

fn count_lines(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut count = 0;
    let mut content = String::new();

    println!("Counting lines of code in: {}", file_path);

    reader.read_to_string(&mut content).unwrap();

    for line in content.lines() {
        if line.trim().len() > 0 {
            count += 1;
        }
    }
    count
}

fn locof(file_path: String) {
    let mut total_lines: u64 = 0;
    let mut total_files: u64 = 0;
    let mut lines_of_code = HashMap::new();

    for entry in WalkDir::new(file_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let mut ext = "".to_string();

            if let Some(extension) = entry.path().extension() {
                ext = extension.to_str().unwrap().to_string();
            }

            if let Some(_) = extensions::get_extensions()
                .iter()
                .find(|e| e.extensions.contains(&ext))
            {
                let count = count_lines(entry.path().to_str().unwrap());
                total_lines += count;
                total_files += 1;
                let counter = lines_of_code.entry(ext.to_string()).or_insert(0);
                *counter += count;
            }
        }
    }

    println!("Total files: {}", total_files);
    println!("Total lines of code: {}", total_lines);
    println!("Lines of code by extension: {:?}", lines_of_code);
}

/// locof aka lines of code of:
/// a tool that takes a project directory and returns the summary of the
/// lines of code of each extension in the project
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: locof <project_dir>");
        process::exit(1);
    }

    let project_dir = &args[1];

    locof(project_dir.to_string());
    println!("Project directory: {}", project_dir);
    process::exit(0);
}
