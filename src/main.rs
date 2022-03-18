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
use std::process::Command;
use std::env;

fn main() {

    //Identify Filename
    let filename = "info.txt";
    println!("In file {}...", filename);

    //Read file to string
    let file_as_string = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    //Boolean test for URLs
    let url_indicator = "https:";
    println!("Does the file contain a URL? {}", file_as_string.contains(url_indicator)); 

    //Establish features to talk to Console
    let mut cmd = Command::new("ping");

    //Split string into lines based on whitespace
    for word in file_as_string.split_whitespace() {
        //Test for url_indicator 
        if word.contains(url_indicator){
            cmd.arg(word);
            match cmd.output() {
                Ok(o) => {
                    let args: Vec<String> = env::args().collect();
                    println!("{:?}", args);
                    // let url_output = fs::read_to_string(o)
                    //     .expect("Something went wrong reading the file");
                    // if url_output.contains("bytes"){
                    //     println!(" {} works!", word);
                    // } else {
                    //    println!("{} is broken with error: {}", word, o);
                    //}   
                },
                    Err(e) => {
                        println!("Something went wrong: ");pin
                }
            }
        }
    }
}

/* Snippits of Code that may be useful:

    if file_as_string.contains(url_indicator) {
        do_something();
    }

    file_as_string.push_str("This is how you append to a string!");

    for url_indicator in file_as_string.chars(){
        println!("Number: {}", url_indicator);} */

