use clap::Parser;
use fsio;
use std::{
    collections::HashMap,
    env, error, fs,
    io::{self, BufRead},
    path,
};
use walkdir::{self, WalkDir};

#[derive(Parser)]
struct EchoCli {
    function: String,
    message: String,
}

#[derive(Parser)]
struct CatCli {
    function: String,
    file1: String,
    file2: String,
}

#[derive(Parser)]
struct LsCli {
    function: String,
    dir: Option<String>,
}

#[derive(Parser)]
struct FindCli {
    function: String,
    pattern: String,
    dir: Option<String>,
}

#[derive(Parser)]
struct GrepCli {
    function: String,
    pattern: String,
    dir: Option<String>,
}

fn echo(message: &str) -> () {
    println!("{message}");
}

fn cat(file1: &str, file2: &str) -> String {
    let mut file1_contents = fs::read_to_string(file1).unwrap();
    let file2_contents = fs::read_to_string(file2).unwrap();

    file1_contents.push_str(&file2_contents);

    file1_contents
}

fn ls(dir: &Option<String>) -> Result<(), Box<dyn error::Error>> {
    let current_dir = env::current_dir().unwrap();
    let path = match dir {
        Some(dir) => path::Path::new(dir),
        None => current_dir.as_path(),
    };

    let entries = fs::read_dir(path)?;

    let _ = entries
        .into_iter()
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            let basename;
            let name = if path.is_file() {
                path.file_name()?.to_str()
            } else {
                basename = fsio::path::get_basename(&path);
                basename.as_deref()
            };
            println!("{}", name?);
            Some(name.unwrap().to_owned())
        })
        .collect::<Vec<_>>();
    Ok(())
}

fn find(pattern: &str, dir: &Option<String>) -> Result<Vec<String>, Box<dyn error::Error>> {
    let current_dir = env::current_dir()?;
    let path = match dir {
        Some(dir) => path::Path::new(dir),
        None => current_dir.as_path(),
    };
    let entries = walkdir::WalkDir::new(path);

    let discovered_files = entries
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let result = if path.is_file() && path.file_name()?.to_str()?.contains(pattern) {
                Some(path.to_str()?.to_owned())
            } else {
                None
            };
            result
        })
        .collect::<Vec<String>>();

    for file in &discovered_files {
        println!("{file}");
    }

    Ok(discovered_files)
}

fn grep(pattern: &str, dir: &Option<String>) -> Result<Vec<String>, Box<dyn error::Error>> {
    let current_dir = env::current_dir()?;
    let path = match dir {
        Some(dir) => path::Path::new(dir),
        None => current_dir.as_path(),
    };

    let mut file_lines_mapping: HashMap<String, HashMap<usize, String>> = HashMap::new();
    let mut lines_mapping: HashMap<usize, String> = HashMap::new();

    let entries = WalkDir::new(path);

    let discovered_files = entries
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let result = if path.is_file() {
                let file = fs::File::open(path).unwrap();
                let reader = io::BufReader::new(file);
                let mut line_count: usize = 0;
                lines_mapping = HashMap::new();
                for line in reader.lines() {
                    line_count += 1;
                    let line = if let Ok(l) = line { l } else { break };

                    if line.contains(pattern) {
                        lines_mapping.insert(line_count, line);
                    }
                }
                let result = if lines_mapping.is_empty() {
                    None
                } else {
                    file_lines_mapping.insert(
                        path.file_name()?.to_str()?.to_owned(),
                        lines_mapping.to_owned(),
                    );
                    Some(path.file_name()?.to_str()?.to_owned())
                };
                result
            } else {
                None
            };
            result
        })
        .collect::<Vec<String>>();
    for (file_name, lines_mapping) in file_lines_mapping {
        println!("{file_name}");
        for (line_number, content) in lines_mapping {
            println!("\t{line_number}: {content}");
        }
    }
    Ok(discovered_files)
}

fn main() {
    let mut args = env::args();

    let func = args.nth(1).unwrap();

    if func == "echo" {
        let args = EchoCli::parse();
        let message = args.message;
        echo(&message);
    } else if func == "cat" {
        let args = CatCli::parse();
        let file1 = args.file1;
        let file2 = args.file2;
        let result = cat(&file1, &file2);
        println!("{result}");
    } else if func == "ls" {
        let args = LsCli::parse();
        let dir = args.dir;
        let _ = ls(&dir);
    } else if func == "find" {
        let args = FindCli::parse();
        let pattern = args.pattern;
        let dir = args.dir;
        let _result = find(&pattern, &dir).unwrap();
    } else if func == "grep" {
        let args = GrepCli::parse();
        let pattern = args.pattern;
        let dir = args.dir;
        let _result = grep(&pattern, &dir);
    }
}
