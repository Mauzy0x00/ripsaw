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
    #[arg(short, long)]
    cyphertext_path: std::path::PathBuf,    // Path to cypher text File

    #[arg(short, long)]
    wordlist_path: std::path::PathBuf,      // Path to the wordlist

    #[arg(short, long, required = false, help = "Meow (Optional)")]
    algorithm: String,
}


fn main() -> Result<()> {
    env_logger::init();
    info!("Starting log...");
    warn!("Ayeee a warning!");

    let args = Args::parse();
    let cyphertext = std::fs::read_to_string(&args.cyphertext_path).with_context(|| format!("File is unreadable! File: `{}`", args.cyphertext_path.display()))?;
    let wordlist = args.wordlist_path;
    // ^ TODO: Don't read entire file into memory; Use something like 'Bufreader' instead of read_to_string()
    let algorithm = args.algorithm;

    
    // Hash each item in the wordlist (Save local hashed lists for common wordlists (check if passed file is common by cheching the file hash itself))
    // In loop, check if the hashed phrase in the word list matches the cyphertext
//  if algo == algo_in_list
//      thread( for *word* in *word_list* {
//              if ( cyphertext == matched_algorithm(word) ) {
//                      MATCH;
//      }); 
//  else { printf("algo not implemented :("); }

    if let Some(hash_algorithm) = get_algorithm(&algorithm) {
        
        // Open passed wordlist file
        // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open(&wordlist) {
            Err(why) => panic!("couldn't open {}: {}", wordlist.display(), why),
            Ok(file) => file,
        };

    // Iterating over each line is not performant. 
    // Also can't read whole file into a String if the file is too big.
    // Can check available RAM and size_of file to make a decision on which to do. 
    // Perhaps try reading in parts of the file to memory then iterate over that 
    // before moving to the next portion of the file. Do this in chunks until EOF

        // Create a reading buffer to the file pointer
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = 
            let hashed_word = hash_algorithm(&line.unwrap());

            if cyphertext == hashed_word {
                println!("Match Found!\nPassword: {}", &line.clone().unwrap());
                break;
            }
        }
    } else {
        eprintln!("Sorry! Passed hashing algorithm ({algorithm}) has not been implemented")
    }

    Ok(())
} // end main