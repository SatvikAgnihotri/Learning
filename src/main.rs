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

enum StatusCode {
    Notfound,
    Found,
}

impl StatusCode {
    fn to_code(&self) -> i32 {
        match self {
            NotFound => 404,
            Found => 200,
        }
    }
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

    //let _failed = StatusCode::NotFound(i32::from(404));
    //let _unregistered = StatusCode::Found(i32::from(302));

    //Split string into lines based on whitespace
    for word in file_as_string.split_whitespace() {
        // let a = reqwest::blocking::get(word).unwrap().text().unwrap();
        // let a = reqwest::blocking::get(word).unwrap().status().is_success();
        //Test for url_indicator
        if word.contains(url_indicator) {
            fn code_as_i32(code: Statuscode) -> i32 {
                match code {
                    StatusCode::NotFound => 404,
                    StatusCode::Found => 302,
                }
            }
            // let success = match reqwest::blocking::get(word){
            //     Ok(resp) => resp.status().is_success(),
            //     Err(err) => false
            // };
            // if! success {
            //     println!("{} with code", word);
            // }
        }
    }
}
//cargo check
/* Snippits of Code that may be useful:

if file_as_string.contains(url_indicator) {
    do_something();
}

file_as_string.push_str("This is how you append to a string!");

for url_indicator in file_as_string.chars(){
    println!("Number: {}", url_indicator);} */
