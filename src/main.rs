use std::collections::HashMap;
use std::{env, path, io};
use std::path::{Path, PathBuf};
use std::{fs, string};
use walkdir::WalkDir;

extern crate regex;
use regex::Regex;
extern crate reqwest;

#[derive(Debug, Clone)]
enum WebsiteOutcome {
    Ok,
    HttpError(reqwest::StatusCode),
    NetworkError(String),
}

fn main() {
    let mut url_history = HashMap::new();

    let mut paths = vec![];
    let mut contents = vec![];
    let mut urls = vec![];
    let re = Regex::new(r#"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)"#).unwrap();

    //Attempting to collect to use as root
    //let home = fs::read_dir("./").unwrap();
    //println!("{:?}", home);

    for entry in WalkDir::new(".").into_iter().filter_map(|file| file.ok()) {
        let a = entry.path().display().to_string();
        paths.push(a);
    }

    for entry in paths {
        match fs::read_to_string(entry) {
            Ok(file_as_string) => contents.push(file_as_string),
            Err(_) => {}
        }
    }

    for entry in contents {
        let _words = entry.split_whitespace();
        for cap in re.captures_iter(&entry) {
            let a = cap[0].to_string(); //println!("{:?}", &cap[0]);
            urls.push(a);
        }
    }

    for entry in urls {
        if !url_history.contains_key(&entry) {
            let outcome = match reqwest::blocking::get(&entry) {
                Ok(resp) => {
                    if resp.status().is_success() {
                        WebsiteOutcome::Ok
                    } else {
                        WebsiteOutcome::HttpError(resp.status())
                    }
                }
                Err(err) => WebsiteOutcome::NetworkError(err.to_string()),
            };

            match outcome.clone() {
                WebsiteOutcome::Ok => {}
                WebsiteOutcome::HttpError(status) => {
                    eprintln!("Server responded with: {}, {}", status, entry)
                }
                WebsiteOutcome::NetworkError(err) => {
                    eprintln!("Failed to connect: {}, {}", err, entry)
                }
            }
            url_history.insert(entry, outcome);
        }
    }
}


    // for entry in fs::read_dir("./").unwrap() {
    //     let entry = entry.unwrap().path();
    //     paths.push(file.path());
    // }