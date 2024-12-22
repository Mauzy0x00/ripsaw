/*
*   Purpose: A learning project to write my own John The Ripper in Rust!
*               Pass a hash value and a word list to crack hashed passwords!
*   
*   Author: Mauzy0x00
*   Date:   12.11.2024
*
*/


// IO
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// System info
use std::path::PathBuf;

// Paralization 
use std::thread;
use std::time::Duration;

// CLI
use anyhow::{Context, Result};
use log::{info, warn};
use clap::Parser;

// Includes
mod hashing;
use hashing::*;


/// Struct to parse Command Line Interface arguments 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long, required = true, help = "Path to the encrypted text.")]
    cyphertext_path: std::path::PathBuf,    // Path to cypher text File

    #[arg(short = 'w', long, required = false, help = "meowmeowmeowmeowmeow!")]
    wordlist_path: std::path::PathBuf,      // Path to the wordlist

    #[arg(short = 'a', long, required = false, help = "Meow (Optional)")]
    algorithm: String,

    #[arg(short = 't', long, required = true, help = "Number of threads used to parse wordlist and crack passwords")]
    thread_count: u8,
}


fn main() -> Result<()> {
    env_logger::init();
    info!("Starting log...");
    warn!("Ayeee a warning!");

    let args = Args::parse();
    let cyphertext = std::fs::read_to_string(&args.cyphertext_path).with_context(|| format!("File is unreadable! File: `{}`", args.cyphertext_path.display()))?;
    let wordlist_path = args.wordlist_path;
    let thread_count = args.thread_count;
    // ^ TODO: Don't read entire file into memory; Use something like 'Bufreader' instead of read_to_string()
    
    let algorithm = args.algorithm;

    if let Some(hash_algorithm) = get_algorithm(&algorithm) {
        
        // Open passed wordlist file
        // Open the path in read-only mode, returns `io::Result<File>`
        let wordlist_file = match File::open(&wordlist_path) {
            Err(why) => panic!("couldn't open {}: {}", wordlist_path.display(), why),
            Ok(file) => file,
        };

        // get the size of the input wordlist
        let file_size = wordlist_path.metadata().unwrap().len();
        println!("File size: {file_size}");


        
        // If the wordlist is larger than 2GB
        if file_size <= 2000000000 { 

            crack_big_wordlist(cyphertext, &wordlist_file, hash_algorithm);

        } else {

            // Read wordlist into a vec (This goes in the if/else checking filesize dumby)
            // Create a reading buffer to the file pointer
            //let reader = BufReader::new(wordlist_file);
            let string_wordlist = std::fs::read_to_string(wordlist_path).with_context(|| format!("File is unreadable! File: `{}`", args.cyphertext_path.display()))?;
            for line in string_wordlist.lines() {
                let line = match line {
                    Err(why) => panic!("Could not read the next line of the wordlist String...\n {}", why),
                    Ok(line) => line,
                };

                let hashed_word = hash_algorithm(&line);

                if cyphertext == hashed_word {
                    println!("Match Found!\nPassword: {}", &line.clone());
                    break;
                }
            }


        }
        
    } else {
        eprintln!("Sorry! Passed hashing algorithm ({algorithm}) has not been implemented")
    }

    Ok(())
} // end main


// Function to crack a hashed password given a large wordlist as the input (Larger than 2GB)
// Optomize file parsing with its larger size in mind. Cannot just read the entire file into memory (unless user specifies otherwise)
fn crack_big_wordlist(cyphertext:String, wordlist_file:&File, hash_algorithm:fn(&str)->String) { // -> vec?  {

        // Create a reading buffer to the file pointer
        let reader = BufReader::new(wordlist_file);

        // Create threads for parseing file and cracking passwords! meoow
        thread::spawn(move || {

        });

        for line in reader.lines() {
            let line = match line {
                Err(why) => panic!("Couldn't read the next line in the file! Why: {}", why), 
                Ok(line) => line,
            };

            let hashed_word = hash_algorithm(&line);

            if cyphertext == hashed_word {
                println!("Match Found!\nPassword: {}", &line.clone());
                break;
            }
        }

} // end get_file_size