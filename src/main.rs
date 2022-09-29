use chrono::prelude::*;
use clap::AppSettings;
use clap::Parser;
use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    title: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path = "C:/Users/ryota-kita/Documents/working";
    if let Some(title) = args.title {
        do_mkdir(&format!("{}/{}", path, create_dir_name(&title, &path)));
    } else {
        do_mkdir(&format!("{}/{}", path, create_dir_name("", &path)));
    }
}

fn do_mkdir(file_name: &str) {
    match fs::create_dir(file_name) {
        Err(e) => panic!("{}: {}", file_name, e),
        Ok(_) => {
            println!("Complete. crated {}.", &file_name);
        }
    }
}

fn create_dir_name(title: &str, path: &str) -> String {
    let today: Date<Utc> = Utc::today();
    format!(
        "#{}_{}_{}",
        get_latest_number(&Path::new(path)) + 1,
        today.format("%y%m%d"),
        title
    )
}

fn get_latest_number(path: &Path) -> i64 {
    let re = Regex::new(r"\#\d+").unwrap();
    let re_only_num = Regex::new(r"\d+").unwrap();

    let dir = read_dir(path).expect("read_dir_error");

    let dir_fil: Vec<regex::Captures> = dir
        .iter()
        .filter_map(|dir_name| re.captures(&dir_name))
        .collect();
    println!("{:?}", dir_fil);

    let map: Vec<i64> = dir_fil
        .iter()
        .map(|index| {
            re_only_num
                .captures(&index.get(0).unwrap().as_str())
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap()
        })
        .collect();

    println!("{:?}", map);

    *map.iter().max().or(Some(&0)).unwrap()
}

fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}
