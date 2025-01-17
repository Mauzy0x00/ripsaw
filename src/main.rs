/*
*   Purpose: A learning project to write my own John The Ripper in Rust!
*               Pass a hash value and a word list to crack hashed passwords!
*               Can also be used to quicly generate hashes of a wordlist (not implemented yet)
*
*   Author: Mauzy0x00
*   Date:   12.11.2024
*
*/


// IO
use std::io::{self, BufReader, Seek, SeekFrom, BufRead};
use std::fs::File;

// System info

use std::path::PathBuf;
// Paralization
use std::thread::{self};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
// use std::time::Duration;

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
    #[arg(short = 'c', long = "cyphertext", required = true, help = "Path to the encrypted text.")]
    cyphertext_path: std::path::PathBuf,    // Path to cypher text File

    #[arg(short = 'w', long = "wordlist", required = false, help = "meowmeowmeowmeowmeow!")]
    wordlist_path: std::path::PathBuf,      // Path to the wordlist

    #[arg(short = 'a', long = "algorithm", required = false, help = "Meow (Optional)")]
    algorithm: String,

    #[arg(short = 't', long = "threads", required = true, help = "Number of threads used to parse wordlist and crack passwords")]
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
        let file_size = wordlist_path.metadata().unwrap().len();
        println!("File size: {file_size}");

        // If the wordlist is larger than 2GB
        if file_size <= 2_000_000_000 {
            let cracked = crack_big_wordlist(cyphertext, wordlist_file, file_size, thread_count, hash_algorithm);

            if cracked {
                println!("Password match was FOUND in the wordlist {}", wordlist_path.display());
            } else {
                println!("Password match was NOT FOUND in the wordlist {}", wordlist_path.display());
            }

        } else {

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


/// Crack a hashed password given a large wordlist as the input (Larger than 2GB)
// Optomize file parsing with its larger size in mind. Cannot just read the entire file into memory (unless user specifies otherwise)
    // Idea:
    //      Each thread is assigned a partition of the file;
    //      Each thread uses a mutex to read a portion of its partition into memory.
    //          The sum of data read into memory from each thread should not exceed 2GB.
    // Create # of threads specified by user for cracking the password list
fn crack_big_wordlist(cyphertext:String, wordlist_file:File, file_size:u64, thread_count:u8, hash_algorithm:fn(&str)->String) -> bool {

    let partition_size = file_size / thread_count as u64; // Get the  size of each thread partition

    println!("File size: {file_size}");
    println!("Partition size per thread: {partition_size}");
    println!("Building threads...");

    let cracked = Arc::new(Mutex::new(false)); // Mutex wrap the cracked bool so we can broadcast to each thread if another thread has found a match

    let mutex_wordlist_file = Arc::new(Mutex::new(wordlist_file)); // Wrap the Mutex in Arc for mutual excusion of the file and an atomic reference across threads
    
    let mut handles: Vec<JoinHandle<()>> = vec![]; // A vector of thread handles

    for thread_id in 0..thread_count {
        let wordlist_file = Arc::clone(&mutex_wordlist_file);   // Create a clone of the mutex_worldist_file: Arc<Mutex><File>> for each thread
        let cracked_bool = Arc::clone(&cracked);          // Create a clone of mutex_cracked for each thread
        
        let cyphertext = cyphertext.to_string();                               // Allocate the cyphertext data in scope for each thread

        let handle = thread::spawn(move || {
            // Calculate current thread's assigned memory space (assigned partition)
            let start = thread_id as u64 * partition_size;

            // If the current thread is the first thread, start at the beginning of the file
            let end = if thread_id == thread_count - 1 {
                                file_size
                            } else {
                                (thread_id as u64 + 1) * partition_size
                            };

        // Request and lock the file
            println!("Thread {thread_id} is now reading from wordlist");
            let mut wordlist_file = wordlist_file.lock().unwrap();

            // Count how many lines are in this current partition
            let line_count:usize = match count_lines_in_partition(&mut wordlist_file, start, end) {
                Err(why) => panic!("Error counting lines on thread {} because {}", thread_id, why),
                Ok(line_count) => line_count,
            };

            let mut lines:Vec<String> = Vec::with_capacity(line_count);  // Allocate a vector of that size (more efficient to pre-allocate and not allocate each entry)

            // Read lines of partition into the vector
            wordlist_file.seek(SeekFrom::Start(start)).expect("Failed to seek to partition start.");    // Move the position of the file read

            let mut buf_reader = BufReader::new(&*wordlist_file); // Create a reading buffer to the file pointer

            let mut current_position = start;
            while current_position < end {
                let mut line = String::new();
                let bytes_read = buf_reader.read_line(&mut line).expect("Failed to read line");

                if bytes_read == 0 {
                    break;
                }

                lines.push(line.trim().to_string());

                current_position += bytes_read as u64;

                if current_position >= end {
                    break;
                }
            }

            println!("Thread {thread_id} finished reading {} lines.", lines.len());

        // Unlock the file and iterate over vector
            drop(wordlist_file); // Drop is now the owner and its scope has ended. So Is this not neccessary and the lock is freed after the seek and read?


            if crack_vector(lines, cyphertext, hash_algorithm, &cracked_bool) {
                println!("cracked!");
            } else {
                println!("Not cracked :(");
            }

        }); // End of thread

        handles.push(handle);   // Push the handles out of the for loop context so they may be joined
    }

    // Iterate ove the vector of handles and join them to conclude cracking
    for handle in handles {
        handle.join().expect("Thread Panicked :(");
    }

    let cracked = cracked.lock().unwrap();
    
    *cracked
} // end get_file_size


/// Iterate over the passed vector and hash the string at that index before checking to see if it matches the
/// passed cypher text. If it does, return that a match has been found (cracked)
// Refactored function to increase readability of the large wordlist crack function
fn crack_vector(lines: Vec<String>, cyphertext:String, hash_algorithm:fn(&str)->String, cracked:&Arc<Mutex<bool>>) -> bool {
    let mut match_found = false;

    for string in lines.iter() {

        let mut this_cracked = cracked.lock().unwrap(); // Get the value cracked flag from the main thread
        let hashed_word = hash_algorithm(string);

        if cyphertext == hashed_word {
            println!("Match Found!\nPassword: {}", string);
            match_found = true;
            *this_cracked = true;   // Change the global value for cracked 
            break;
        
        // If another thread has cracked the hash then break out of cracking loop
        } else if *this_cracked {
            break;
        }
    }

    match_found
} // end crack_vector

/// Count how many lines are in the portion of the file that was partitioned to each thread
// Refactored function to increase readability of the large wordlist crack function
fn count_lines_in_partition(file: &mut File, start: u64, end: u64) -> io::Result<usize> {
    file.seek(SeekFrom::Start(start))?;
    let mut buf_reader = BufReader::new(file);
    let mut line_count = 0;
    let mut current_position = start;

    while current_position < end {
        let mut line = String::new();
        let bytes_read = buf_reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break; // EOF reached
        }
        line_count += 1;
        current_position += bytes_read as u64;
        if current_position >= end {
            break;
        }
    }
    Ok(line_count)
} // end count_lines_in_partition


/// Function to crack a hash with a wordlist that is smaller than 2GB
fn crack_small_wordlist(cyphertext:&String, wordlist_path: &PathBuf, cyphertext_path: PathBuf, hash_algorithm:fn(&str)->String) -> Result<bool> {
    let mut cracked = false;
    // Create a reading buffer to the file pointer
    //let reader = BufReader::new(wordlist_file);
    let string_wordlist = std::fs::read_to_string(wordlist_path).with_context(|| format!("File is unreadable! File: `{}`", cyphertext_path.display()))?;
    for line in string_wordlist.lines() {

        let hashed_word = hash_algorithm(line);

        if *cyphertext == hashed_word {
            println!("Match Found!\nPassword: {}", &line);
            cracked = true;
            break;
        }
    }
    Ok(cracked)
}