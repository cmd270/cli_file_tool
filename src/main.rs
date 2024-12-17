use std::{self, fs::read_to_string};
use std::fs;
use std::io::{self, Write};
use filetime::FileTime;
use clap::Parser;
use anyhow::{Context, Result};

mod dateconveter;

macro_rules! format_array {
    ($arr:expr, $fmt:expr) => {
        $arr.iter()
            .map(|x| format!($fmt, x))
            .collect::<Vec<_>>()
            .join(", ")
    };
}

#[derive(Parser)]
struct Cli{
    path: std::path::PathBuf,
    pattern: String,
}

fn get_file_info(path: &std::path::PathBuf){
    let metadata = fs::metadata(path).unwrap();

    let ctime = dateconveter::from_unix_to_datetime(
        FileTime::from_creation_time(&metadata)
        .unwrap()
        .unix_seconds()
    );
    
    let mtime = dateconveter::from_unix_to_datetime(
        FileTime::from_last_modification_time(&metadata)
        .unix_seconds()
    );

    let atime = dateconveter::from_unix_to_datetime(
        FileTime::from_last_access_time(&metadata)
        .unix_seconds()
    );

    let file_size = metadata.len();

    println!("File name: {} 🍣", path.file_name()
        .unwrap()
        .to_string_lossy()
    );
    println!("Size: {} bytes", file_size);
    println!("Creation time: {}", ctime);
    println!("Last modification time: {}", mtime);
    println!("Last access time: {}", atime);
}

fn find_positions(line: &str, pattern: &str) -> Vec<usize>{
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(index) = line[start..].find(pattern){
        let actual_index = start + index;
        positions.push(actual_index);
        start = actual_index + pattern.len();
    }

    positions
}

fn find_matches(content: &str, pattern: &str){
    let mut total_lines = 0;
    for (index, line) in content.lines().enumerate(){
        if !line.contains(pattern){
            continue;
        }
        total_lines += 1;
        let positions = find_positions(&line, &pattern);
        println!("ln: {}, col: {} | `{}`", index + 1, format_array!(&positions, "{}"), line);
    }

    println!("Pattern `{}` found in {} lines.", pattern, total_lines);
    
}

fn main() -> Result<()> {
    let args = Cli::parse(); 
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", args.path))?;

    get_file_info(&args.path);
    find_matches(&content, &args.pattern);

    Ok(())
}