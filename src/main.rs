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

2. FASTER FASTER FASTER
*/

// IO
use std::io::{self, BufReader, Seek, SeekFrom, BufRead};
use std::fs::File;

// System info
use std::path::PathBuf;

// Paralization
use std::thread::{self};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
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

    #[arg(short = 'w', long = "wordlist", required = true, help = "meowmeowmeowmeowmeow!")]
    wordlist_path: std::path::PathBuf,      // Path to the wordlist

    #[arg(short = 'a', long = "algorithm", required = true, help = "Meow (Optional)")]
    algorithm: String,

    #[arg(short = 't', long = "threads", required = true, help = "Number of threads used to parse wordlist and crack passwords")]
    thread_count: u8,

    #[arg(short = 'b', long = "bruteforce", required = false, help = "Will bruteforce the hash. Be ready to wait")]
    bruteforce: bool,
}


fn main() -> Result<()> {
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
    Options:
    -c, --cyphertext <PATH>    Path to the encrypted text. (required)
    -w, --wordlist <PATH>      Path to the wordlist. (required)
    -a, --algorithm <NAME>     Hashing algorithm to use. (required)
    -t, --threads <NUMBER>     Number of threads used to parse wordlist and crack passwords. (required)
    -b, --bruteforce           Will bruteforce the hash. Be ready to wait. (optional)
    "#;
    
    println!("{banner}\n{options}");


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

        // If the wordlist is larger than 2GB
        if file_size >= 20_000_000 {
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

    let cracked = Arc::new(AtomicBool::new(false)); // Mutex wrap the cracked bool so we can broadcast to each thread if another thread has found a match

    let mutex_wordlist_file = Arc::new(Mutex::new(wordlist_file)); // Wrap the Mutex in Arc for mutual excusion of the file and an atomic reference across threads
    
    let mut handles: Vec<JoinHandle<()>> = vec![]; // A vector of thread handles

    for thread_id in 0..thread_count {
        let wordlist_file = Arc::clone(&mutex_wordlist_file);   // Create a clone of the mutex_worldist_file: Arc<Mutex><File>> for each thread
        let cracked = Arc::clone(&cracked);                      // Clone of cracked for each thread
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

            println!("Starting to crack on thread {thread_id}");
            if crack_vector(lines, cyphertext, hash_algorithm, &cracked) {
                println!("cracked!");
            } else {
                println!("Not cracked on thread {thread_id} :(");
            }

        }); // End of thread

        handles.push(handle);   // Push the handles out of the for loop context so they may be joined
    }

    // Iterate ove the vector of handles and join them to conclude cracking
    for handle in handles {
        handle.join().expect("Thread Panicked :(");
    }

    cracked.load(Ordering::Relaxed)
} // end get_file_size


/// Iterate over the passed vector and hash the string at that index before checking to see if it matches the
/// passed cypher text. If it does, return that a match has been found (cracked)
// Refactored function to increase readability of the large wordlist crack function
fn crack_vector(lines: Vec<String>, cyphertext:String, hash_algorithm:fn(&str)->String, cracked:&Arc<AtomicBool<>>) -> bool {
    let mut match_found = false;

    for string in lines.iter() {

        let hashed_word = hash_algorithm(string);

        if cyphertext == hashed_word {
            cracked.store(true, Ordering::Relaxed);
            println!("Match Found!\nPassword: {}", string);
            match_found = true;
            break;
        
        // If another thread has cracked the hash then break out of cracking loop
        } else if cracked.load(Ordering::Relaxed) {
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
    println!("Loading wordlist into memory");
    let string_wordlist = std::fs::read_to_string(wordlist_path).with_context(|| format!("File is unreadable! File: `{}`", cyphertext_path.display()))?;
    
    println!("Starting to crack!");
    for line in string_wordlist.lines() {

        let hashed_word = hash_algorithm(line);

        if *cyphertext == hashed_word {
            println!("cyphertext: {}", *cyphertext);
            println!("hashed word: {hashed_word}");
            println!("Match Found!\nPassword: {}", &line);
            cracked = true;
            break;
        }
    }
    Ok(cracked)
}

