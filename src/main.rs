/*
*   Purpose: A learning project to write my own John The Ripper in Rust!
*               Pass a hash value and a word list to crack hashed passwords!
*   
*   Author: Mauzy0x00
*   Date:   12.11.2024
*
*/


// IO
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;

// Parallelization 
use std::thread;
use std::sync::{Arc, Mutex};
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
    
    let algorithm = args.algorithm;

    if let Some(hash_algorithm) = get_algorithm(&algorithm) {
        
        // Open passed wordlist file
        // Open the path in read-only mode, returns `io::Result<File>`
        let wordlist_file = match File::open(&wordlist_path) {
            Err(why) => panic!("couldn't open {}: {}", wordlist_path.display(), why),
            Ok(file) => file,
        };

        // get the size of the input wordlist
        let file_size:u64 = wordlist_file.metadata()?.len();
        println!("File size: {file_size}");
        
        // If the wordlist is larger than 2GB
        if file_size >= 2_000_000_000 { 

            crack_big_wordlist(&cyphertext, wordlist_file, file_size, thread_count, hash_algorithm);

        } else {
            // Typical file size (< 2GB)
            let wordlist_content = BufReader::new(wordlist_file)
                                .lines()
                                .collect::<Result<Vec<_>, io::Error>>()?;
            for line in wordlist_content {
                if let Some(found) = process_line(&line, &cyphertext, hash_algorithm) {
                    println!("Match Found!\nPassword: {}", found);
                    return Ok(());
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
fn crack_big_wordlist(cyphertext:&str, wordlist_file:File, file_size:u64, thread_count: u8, hash_algorithm:fn(&str)->String) -> Result<()>{

    let partition_size = file_size / thread_count as u64;

    println!("File size: {file_size}");
    println!("Partition size per thread: {partition_size}");
    
    let wordlist_file_mutex = Arc::new(Mutex::new(wordlist_file)); // Shared file handle for threads

    let mut handles = vec![];

    for thread_id in 0..thread_count {
        let file = Arc::clone(&wordlist_file_mutex);
        let cyphertext = cyphertext.to_string();

        let handle = thread::spawn(move || {
            let start = thread_id as u64 * partition_size;
            let end = if thread_id == thread_count - 1 {
                file_size
            } else {
                (thread_id as u64 + 1) * partition_size
            };

            let mut file = file.lock().unwrap();
            file.seek(SeekFrom::Start(start)).expect("Failed to seek to partition start");

            let reader = BufReader::new(&*file);    // The '*' "unwraps" the file pointer from the mutex (Points to the value of the file pointer, not the mutex)
            for line in reader.lines() {
                let line = line.expect("Failed to read line");
                if file.seek(SeekFrom::Current(0)).unwrap() >= end {
                    break;
                }
                let hashed_word = hash_algorithm(&line);
                if hashed_word == cyphertext {
                    println!("Match found by thread {thread_id}!\nPassword: {line}");
                    return;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())

} // end get_file_size

// Helper function to process a line
fn process_line(line: &str, cyphertext: &str, hash_algorithm: fn(&str) -> String) -> Option<String> {
    let hashed_word = hash_algorithm(line);
    if cyphertext == hashed_word {
        Some(line.to_string())
    } else {
        None
    }
} // end process_line