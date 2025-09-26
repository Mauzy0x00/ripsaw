use crate::library::crack_vector;

use std::{
    fs::File,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Function to test how many threads are optimal for the users computer
pub fn thread_benchmark(wordlist_path: &PathBuf, hash_algorithm: fn(&str) -> String) {
    let default_hash = hash_algorithm("R1psaw$$$");

    println!("[+] Getting CPU count.");
    let num_cpus = num_cpus::get();
    let mut thread_count = num_cpus;
    let thread_count_max = num_cpus * 2; // Twice the amount of CPUs
    let mut agregated_execution_time: Vec<u64> = Vec::new();

    // Open passed wordlist file
    // Open the path in read-only mode, returns `io::Result<File>`
    let wordlist_file = match File::open(wordlist_path) {
        Err(why) => panic!("couldn't open {}: {}", wordlist_path.display(), why),
        Ok(file) => file,
    };
    let file_size = wordlist_path.metadata().unwrap().len(); // Get the size of the input wordlist
    let mutex_wordlist_file = Arc::new(Mutex::new(wordlist_file)); // Wrap the Mutex in Arc for mutual excusion of the file and an atomic reference across threads
    let cracked = Arc::new(AtomicBool::new(false)); // Mutex wrap the cracked bool so we can broadcast to each thread if another thread has found a match

    let start_total = Instant::now(); // Start the timer for total execution
    let mut test_times: Vec<u64> = Vec::new(); // A vector to hold the list of times

    //  Increment the thread count for every test
    for thread_count in 1..=thread_count_max {
        let partition_size = file_size / thread_count as u64; // Get the  size of each thread partition
        let start_test = Instant::now(); // Start the timer for test execution
        let mut handles: Vec<JoinHandle<()>> = Vec::new(); // A vector of thread handles

        // Generate threads to do work
        for thread_id in 0..thread_count {
            let wordlist_file = Arc::clone(&mutex_wordlist_file); // Create a clone of the mutex_worldist_file: Arc<Mutex><File>> for each thread
            let default_hash = default_hash.to_string(); // Allocate the cyphertext data in scope for each thread
            let cracked = Arc::clone(&cracked); // Clone of cracked for each thread

            let handle = thread::spawn(move || {
                // zoomy thing goes here !
                thread_work(
                    thread_id,
                    default_hash,
                    partition_size,
                    thread_count,
                    file_size,
                    wordlist_file,
                    hash_algorithm,
                    cracked,
                );
            });

            handles.push(handle); // Push the handles out of the for loop context so they may be joined
        }

        // Iterate ove the vector of handles and join them to conclude cracking
        for handle in handles {
            handle.join().expect("Thread Panicked :(");
        }

        let test_execution_time = start_test.elapsed().as_secs(); // Capture the test execution time
                                                                  //   Record tests to a vector for analyzation after the benchmark
        test_times.push(test_execution_time);
        println!("Thread Count: {}", thread_count);
    }

    // Save this time here!
    let execution_time = start_total.elapsed().as_secs();

    // Run analysis here!

    // Output to user best number of threads :3
}

#[allow(clippy::too_many_arguments)]
fn thread_work(
    thread_id: usize,
    cyphertext: String,
    partition_size: u64,
    thread_count: usize,
    file_size: u64,
    wordlist_file: Arc<Mutex<File>>,
    hash_algorithm: fn(&str) -> String,
    cracked: Arc<AtomicBool>,
) {
    // Calculate current thread's assigned memory space (assigned partition)
    let start = thread_id as u64 * partition_size;

    // If the current thread is the first thread, start at the beginning of the file
    let end = if thread_id == thread_count - 1 {
        file_size
    } else {
        (thread_id as u64 + 1) * partition_size
    };

    // Request and lock the file
    let mut wordlist_file = wordlist_file.lock().unwrap();

    // Count how many lines are in this current partition
    let line_count: usize = match count_lines_in_partition(&mut wordlist_file, start, end) {
        Err(why) => panic!(
            "[X] Error counting lines on thread {} because {}",
            thread_id, why
        ),
        Ok(line_count) => line_count,
    };

    let mut lines: Vec<String> = Vec::with_capacity(line_count); // Allocate a vector of that size (more efficient to pre-allocate and not allocate each entry)

    // Read lines of partition into the vector
    wordlist_file
        .seek(SeekFrom::Start(start))
        .expect("Failed to seek to partition start."); // Move the position of the file read

    let mut buf_reader = BufReader::new(&*wordlist_file); // Create a reading buffer to the file pointer

    let mut current_position = start;
    while current_position < end {
        let mut line = String::new();
        let bytes_read = buf_reader
            .read_line(&mut line)
            .expect("Failed to read line");

        if bytes_read == 0 {
            break;
        }

        lines.push(line.trim().to_string());

        current_position += bytes_read as u64;

        if current_position >= end {
            break;
        }
    }
    // Unlock the file and iterate over vector
    drop(wordlist_file); // Drop is now the owner and its scope has ended. So Is this not neccessary and the lock is freed after the seek and read?

    crack_vector(lines, cyphertext, hash_algorithm, &cracked);

    println!("\n[-] Cleaning up remaining threads...");
}

use std::io::{self, BufRead, BufReader, Seek, SeekFrom};

/// Count how many lines are in the portion of the file that was partitioned to each thread
// Refactored function to increase readability of the large wordlist crack function
fn count_lines_in_partition(file: &mut File, start: u64, end: u64) -> io::Result<usize> {
    file.seek(SeekFrom::Start(start))?;
    let mut buf_reader = BufReader::new(file);
    let mut line_count: usize = 0;
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
