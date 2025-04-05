




use crate::library::crack_vector;

use anyhow::{Context, Error};
use std::result::Result::Ok;
use std::path::PathBuf;
use std::thread::{self};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

/// Function to crack a hash with a wordlist that is smaller than 2GB
pub fn crack_small_wordlist(cyphertext:&String, wordlist_path: &PathBuf, hash_algorithm:fn(&str)->String) -> Result<bool, Error> {
    let mut cracked = false;
    // Create a reading buffer to the file pointer
    //let reader = BufReader::new(wordlist_file);
    println!("[+] Loading wordlist into memory");
    let string_wordlist = std::fs::read_to_string(wordlist_path).with_context(|| format!("File is unreadable! File: `{}`", wordlist_path.display()))?;
    
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


/// Crack a hashed password given a large wordlist as the input (Larger than 2GB)
// Optomize file parsing with its larger size in mind. Cannot just read the entire file into memory (unless user specifies otherwise)
    // Idea:
    //      Each thread is assigned a partition of the file;
    //      Each thread uses a mutex to read a portion of its partition into memory.
    //          The sum of data read into memory from each thread should not exceed 2GB.
    // Create # of threads specified by user for cracking the password list
pub fn crack_big_wordlist(cyphertext:String, wordlist_file:File, file_size:u64, thread_count:u8, hash_algorithm:fn(&str)->String, verbose:bool) -> bool {

    let partition_size = file_size / thread_count as u64; // Get the  size of each thread partition

    println!("File size: {file_size}");
    println!("Partition size per thread: {partition_size}");
    println!("[+] Building threads...");

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
            if verbose { println!("[+] Thread {thread_id} is now reading from wordlist"); }
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

            if verbose { println!("Thread {thread_id} finished reading {} lines.", lines.len()); }
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


use std::fs::File;
use std::io::{self, BufReader, Seek, SeekFrom, BufRead};
/// Count how many lines are in the portion of the file that was partitioned to each thread
// Refactored function to increase readability of the large wordlist crack function
fn count_lines_in_partition(file: &mut File, start: u64, end: u64) -> io::Result<usize> {
    file.seek(SeekFrom::Start(start))?;
    let mut buf_reader = BufReader::new(file);
    let mut line_count:usize = 0;
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