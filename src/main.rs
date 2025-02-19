/*
*   Purpose: A learning project to write my own John The Ripper in Rust!
*               Pass a hash value and a word list to crack hashed passwords!
*               Can also be used to quickly generate hashes of a wordlist (not implemented yet)
*
*   Author: Mauzy0x00
*   Date:   12.11.2024
*
*/

/* 
TODO: 
1.  Add a function to measure time and try to deterine the amount of threads that would be most
    efficient for the file size. Is mutlithreading more effiecent? Fuck I hope so it took a long time. 
2. Bruteforcing
    Implement an iterator for Bruteforce
3. Functionality to determine which algorithm was used to generate the given hash 
4. FASTER FASTER FASTER
    - GPU Compute
    - Pre-allocated buffers
    - SIMD-friendly operations
*/


// IO
use std::fs::File;

// use std::time::Duration;

// CLI
use anyhow::{Context, Result};
use log::{info, warn};
use clap::Parser;

/// Local Includes
// Register local modules
mod library;
mod hashing;
mod bruteforce;
mod dictionary_attack;

// import functions from local modules
use hashing::get_algorithm;
use bruteforce::bruteforce;
use dictionary_attack::{crack_small_wordlist, crack_big_wordlist};


/// Struct to parse Command Line Interface arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long = "cyphertext", required = true, help = "Path to the encrypted text.")]
    cyphertext_path: std::path::PathBuf,    // Path to cypher text File

    #[arg(short = 'w', long = "wordlist", required = false, help = "meowmeowmeowmeowmeow!")]
    wordlist_path: std::path::PathBuf,      // Path to the wordlist

    #[arg(short = 'a', long = "algorithm", required = true, help = "Meow (Optional)")]
    algorithm: String,

    #[arg(short = 't', long = "threads", required = true, help = "Number of threads used to parse wordlist and crack passwords")]
    thread_count: u8,

    #[arg(short = 'b', long = "bruteforce", required = false, help = "Will bruteforce the hash. Be ready to wait")]
    bruteforce: bool,
}


fn main() -> Result<()> {
    initialize();

    // Parse user arguments
    let args = Args::parse();
    let mut cyphertext = std::fs::read_to_string(&args.cyphertext_path).with_context(|| format!("File is unreadable! File: `{}`", args.cyphertext_path.display()))?;
    cyphertext = cyphertext.to_lowercase(); // Needs to be lowercase to correctly match hash

    let wordlist_path = args.wordlist_path;
    let thread_count = args.thread_count;

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

        if args.bruteforce {
            bruteforce(cyphertext, 1, thread_count, hash_algorithm);
        }
        // If the wordlist is larger than 2GB

        else if !args.bruteforce && file_size >= 2_000_000 {
            let cracked = crack_big_wordlist(cyphertext, wordlist_file, file_size, thread_count, hash_algorithm);

            if cracked {
                println!("Password match was FOUND in the wordlist {}", wordlist_path.display());
            } else {
                println!("Password match was NOT FOUND in the wordlist {}", wordlist_path.display());
            }

        } else if !args.bruteforce {

            let cracked = match crack_small_wordlist(&cyphertext, &wordlist_path, args.cyphertext_path, hash_algorithm) {
                Err(why) => panic!("Error cracking wordlist {}: {}", wordlist_path.display(), why),
                Ok(cracked) => cracked,
            };

            if cracked {
                println!("Successfully cracked the hash!");
            } else {
                println!("Successfully processed the hash but no match was found :(");
            }
        }
    } else {
        eprintln!("Sorry! Passed hashing algorithm ({algorithm}) has not been implemented")
    }

    Ok(())
} // end main

fn initialize() {
    env_logger::init();
    info!("Starting log...");
    warn!("Ayeee a warning!");

    let banner = r#"
         _______ ___________  _____  ___  _    _ _ 
        | | ___ \_   _| ___ \/  ___|/ _ \| |  | | |
        | | |_/ / | | | |_/ /\ `--./ /_\ \ |  | | |
        | |    /  | | |  __/  `--. \  _  | |/\| | |
        | | |\ \ _| |_| |    /\__/ / | | \  /\  / |
        | \_| \_|\___/\_|    \____/\_| |_/\/  \/| |
        | |                                     | |
        |_|                                     |_|
        \|/                                     \|/ 
"#;
    let options = r#"
                ex.  ripsaw -w [path] -c [path] -a sha256 -t 5
    Options:
    -c, --cyphertext <PATH>    Path to the encrypted text. (required)
    -w, --wordlist <PATH>      Path to the wordlist. (required)
    -a, --algorithm <NAME>     Hashing algorithm to use. (required)
    -t, --threads <NUMBER>     Number of threads used to parse wordlist and crack passwords. (required)
    -b, --bruteforce           Will bruteforce the hash. Be ready to wait. (optional)
    "#;
    
    println!("{banner}\n{options}");
}
