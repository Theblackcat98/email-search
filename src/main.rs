use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;
use serde_json::json;

fn search_email_and_strings(file_path: &str) -> io::Result<Vec<String>> {
    // Regular expression to match email addresses and the strings that follow them
    let pattern = Regex::new(r"([A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}):(.+)")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Regex error: {}", e)))?;
    
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut email_and_strings = Vec::new();

    for line in reader.lines() {
        let content = line?;
        
        // Use regex captures to find email addresses and the strings that follow them
        if let Some(captures) = pattern.captures(&content) {
            let email = captures[1].to_string();
            let string_following_email = captures[2].to_string();
            
            // Create a JSON string for each email and its associated string
            let json_pair = json!({
                "email": email,
                "string": string_following_email,
            });

            email_and_strings.push(json_pair.to_string());
        }
    }

    Ok(email_and_strings)
}

fn main() {
    let file_path = "./textfile.txt";
    
    match search_email_and_strings(file_path) {
        Ok(email_and_strings) => {
            println!("Email addresses and their associated strings (JSON format):");
            for json_pair in email_and_strings {
                println!("{}", json_pair);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}


