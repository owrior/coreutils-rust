use clap::Parser;
use fsio;
use std::{env, error, fs, path};

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
    }
}
