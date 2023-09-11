use clap::Parser;
use std::env;
use std::fs;

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
    dir: String,
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

fn ls(dir: &str) -> () {
    let dir = env::current_dir().unwrap();
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
    } else if func == "cat" {
        let args = LsCli::parse();
        let dir = args.dir;
        ls(&dir);
    }
}
