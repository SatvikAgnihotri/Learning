/*! Given a directory, read all the files, and find anything that looks like a url. Test the url, and see if its error 404.
Then return a file with all non-valid urls.

Basic Outline
1. For file in folder
2. Read file to a string.
3. Turn each term in a string into a vector (dividing string into vector by spaces ” “)
4. Search vectors for URL link (looking for “https:“)
5. For each vector that has “https:“, plug the vector into a website test.
6. Initialize an empty string called Broken_Links
6. If link is broken, append the vector to the broken links string
7. When done counting each term in the file, print file name followed by all broken links in that file
8. Repeat for all files in the folder.
*/

// :: is a path seperator. std is crate. Fs is module.

use std::fs;
extern crate reqwest;

#[derive(Debug)]
enum WebsiteOutcome {
    Ok,
    HttpError(reqwest::StatusCode),
    NetworkError(reqwest::Error),
}

fn main() {
    //Identify Filename
    let filename = "info.txt";
    println!("In file {}...", filename);

    //Read file to string
    let file_as_string =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    //Boolean test for URLs
    let url_indicator = "https:";
    println!(
        "Does the file contain a URL? {}",
        file_as_string.contains(url_indicator)
    );

    //Split string into lines based on whitespace
    for word in file_as_string.split_whitespace() {
        //Test for url_indicator
        if word.contains(url_indicator) {
            let outcome = match reqwest::blocking::get(word) {
                Ok(resp) => {
                    if resp.status().is_success() {
                        WebsiteOutcome::Ok
                    } else {
                        WebsiteOutcome::HttpError(resp.status())
                    }
                }
                Err(err) => WebsiteOutcome::NetworkError(err),
            };

            match outcome {
                WebsiteOutcome::Ok => {}
                WebsiteOutcome::HttpError(status) => eprintln!("Server responded with: {}", status),
                WebsiteOutcome::NetworkError(err) => eprintln!("Failed to connect: {}", err),
            }
        }
    }
}
