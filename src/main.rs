use clap::Parser;
use std::{fs, io, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of the file to copy
    // #[arg(short, long)]
    file: String,
}

fn main() {
    let home_dir = std::env::var("HOME").expect("Home directory not found!");
    let mut path = path::PathBuf::from(home_dir);
    path.push("test");

    let cli = Args::parse();
    let path_to_file = path::Path::new(&cli.file);

    if !path_to_file.exists() {
        println!("Lol!");
    }

    println!("{}", path_to_file.display());
}

fn get_config_dir() -> path::PathBuf {
    let home_dir = std::env::var("HOME").expect("Home directory not found!");
    let mut path = path::PathBuf::from(home_dir);
    path.push(".config");
    return path;
}
