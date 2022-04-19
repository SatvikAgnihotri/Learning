use std::env;
use std::thread::current;
use std::{fs, string};
use std::collections::HashMap;

extern crate regex; use regex::Regex;
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
    
    for entry in fs::read_dir("./").unwrap() {
        let entry = entry.unwrap().path();
        paths.push(entry);
    }

    for entry in paths {
        match fs::read_to_string(entry) {
            Ok(file_as_string) => {contents.push(file_as_string)},
            Err(_) => {},
        }
    }

    for entry in contents {
        let words = entry.split_whitespace();
        for cap in re.captures_iter(&entry) {
                let a = cap[0].to_string(); //println!("{:?}", &cap[0]);
                urls.push(a);
            }
        }

    for entry in urls {
        if !url_history.contains_key(&entry){
        
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
                WebsiteOutcome::HttpError(status) => 
                    eprintln!("Server responded with: {}, {}", status, entry),
                WebsiteOutcome::NetworkError(err) => 
                    eprintln!("Failed to connect: {}, {}", err, entry),
            }
            url_history.insert(
                entry,
                outcome,
            );
        }
    }
}
