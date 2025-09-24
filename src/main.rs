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
    Work out issues with generation and threadpooling 
3. Functionality to determine which algorithm was used to generate the given hash 
4. Functionality for Salt and Pepper
5. FASTER FASTER FASTER
    - GPU Compute
    - Pre-allocated buffers
    - SIMD-friendly operations
*/


// IO
use std::fs::File;
use std::path::PathBuf;

use clap::{Command, Parser};
use colored::Colorize;

// use std::time::Duration;

// CLI
use anyhow::Result;
use log::{info, warn};

/// Local Includes
// Register local modules
mod arg_parser;
mod library;
mod hashing;
mod bruteforce;
mod dictionary_attack;
mod ssh;

// import functions from local modules
use arg_parser::{Args, Commands};
use library::Config;
use hashing::get_algorithm;
use bruteforce::bruteforce;
use dictionary_attack::{crack_small_wordlist, crack_big_wordlist};

fn main() -> Result<()> {
    
    initialize();

    let args = Args::parse();

    match args.command {

        Some(Commands::Wordlist {
            cyphertext_path,
            wordlist_path,
            algorithm,
            salt,
            thread_count,
            verbose,
        }) => {

            let config = Config {
                salt_present: !salt.is_empty(), // I think this is right. might be backwards
                verbose,
            };
            
            let cyphertext = std::fs::read_to_string(&cyphertext_path)?
                .to_lowercase();

            if let Some(algorithm_function) = get_algorithm(&algorithm) {
                
                process_wordlist(salt, cyphertext, &wordlist_path, algorithm_function, thread_count, config)?;

            } else {
                eprintln!("Sorry! Passed hashing algorithm ({algorithm}) has not been implemented")
            }
        }

        
        Some(Commands::Bruteforce {
            cyphertext_path,
            thread_count,
            min_length,
            algorithm,
            salt,
            verbose,
        }) => {

            let config = Config {
                salt_present: !salt.is_empty(), // I think this is right. might be backwards
                verbose,
            };

            let cyphertext = std::fs::read_to_string(&cyphertext_path)?
                .to_lowercase();
            if let Some(algorithm_function) = get_algorithm(&algorithm) {

                bruteforce(salt, cyphertext, min_length, thread_count, algorithm_function, config);

            } else {
                eprintln!("Sorry! Passed hashing algorithm ({algorithm}) has not been implemented")
            }
        }

        Some(Commands::Ssh {
            server,
            port,
            wordlist_path,
            user,
            thread_count,
            salt,
            verbose,
        }) => {

            let config = Config {
                salt_present: !salt.is_empty(), // I think this is right. might be backwards
                verbose,
            };

            ssh::ssh_socket(server, port, user, wordlist_path, config)?;
        }

        None => {}
    }

    // list all the hasing algorithms from the list in hashing.rs
    if args.list {
        println!("{}", "Hashing Algorithms Available:".bold().underline());
        for i in hashing::ALGORITHMS.iter() {
            println!("  {}", i);
        }
        println!("\n");
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

        Rip and Tear. . .
"#;

    println!("{banner}\n");
}


fn process_wordlist(
    salt: String, 
    cyphertext: String, 
    wordlist_path: &PathBuf, 
    algorithm: fn(&str) -> String, 
    thread_count: u8, 
    config: Config,
) -> Result<()> {

    // Open passed wordlist file
    // Open the path in read-only mode, returns `io::Result<File>`
    let wordlist_file = match File::open(wordlist_path) {
        Err(why) => panic!("couldn't open {}: {}", wordlist_path.display(), why),
        Ok(file) => file,
    };

    // get the size of the input wordlist
    let file_size = wordlist_path.metadata().unwrap().len();
    println!("File size: {file_size}");

    // If the wordlist is larger than 2GB
    if file_size >= 2_000_000 {
        let cracked = crack_big_wordlist(salt, cyphertext, wordlist_file, file_size, thread_count, algorithm, config);

        if cracked {
            println!("Password match was FOUND in the wordlist {}", wordlist_path.display());
        } else {
            println!("Password match was NOT FOUND in the wordlist {}", wordlist_path.display());
        }

    } else {
        let cracked = match crack_small_wordlist(salt, &cyphertext, wordlist_path, algorithm, config) {
            Err(why) => panic!("Error cracking wordlist {}: {}", wordlist_path.display(), why),
            Ok(cracked) => cracked,
        };

        if cracked {
            println!("Successfully cracked the hash!");
        } else {
            println!("Successfully processed the hash but no match was found :(");
        }
    }

    Ok(())
}