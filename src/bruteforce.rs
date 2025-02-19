


// Parallelization
use std::thread::{self};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;



// NEEDS: a.List of characters; b.Distrubtion method across threads; c.method of bruteforcing with a wordlist or phrase; d.common password patterns
/// Divide and conquer
pub fn bruteforce(cyphertext:String, length:usize, thread_count:u8, hash_algorithm:fn(&str)->String) -> bool {

    let cracked = Arc::new(AtomicBool::new(false)); // Mutex wrap the cracked bool so we can broadcast to each thread if another thread has found a match
    let mut handles: Vec<JoinHandle<()>> = vec![]; // A vector of thread handles
    


    // Rainbow table ??
    // Avoid collisions 
    // Create a thread pool
    // Define a job list / number of threads
    // Assign given number of threads with available jobs ; ensure jobs don't have combined values
    // To save time, a wordlist with pre-generated values should be given so the program can focus on hashing
    // Jobs can be assigned a weight to indicate their importance in the list of jobs
        /* JOBS: 
        GENERATION workers: Worker threads tasked with generating a list of word combinations. 
        * Assign Jobs in order of relevance. ie. most common password patterns first
            - Lowercase
            - Uppercase
            - numbers
            - symbols

            - Lowercase w/ uppercase - Split
            - Lowercase w/ numbers - Split
            - Lowercase w/ symbols - Split

            - Uppercase w/ Lowercase - Split
            - Uppercase w/ numbers - Split
            - Uppercase w/ symbols - Split

            - Uppercase w/ Lowercase w/ numbers - Split x2
            - Uppercase w/ Lowercase w/ symbols - Split x2
            - Uppercase w/ Lowercase w/ numbers - Split x2

            - Uppercase w/ numbers - Split
            - Uppercase w/ symbols - Split
        
        
        CRACKING workers
        
        
        
        */ 

        // Alternatively, Calculate all possible combinations in a wordlist and have a thread pool iterate over this list 
        // Then the array 

    // Character set generation
    let charset = CharacterSet::default();
    let mut generator = Generator::new(&charset, true, true, true, false, length);


    for thread_id in 0..thread_count {
        let cracked = Arc::clone(&cracked);     // Clone of cracked for each thread
        let cyphertext = cyphertext.to_string();               // Allocate the cyphertext data in scope for each thread
        let division_of_work = 20;

        let handle = thread::spawn(move || {
            // THREAD'S JOB!
        }); 
        
        if cracked.load(Ordering::Relaxed) {
            handles.push(handle);   // Push the handles out of the for loop context so they may be joined
        }
    }

    // Iterate ove the vector of handles and join them to conclude cracking
    for handle in handles {
        handle.join().expect("Thread Panicked :(");
    }
    
    cracked.load(Ordering::Relaxed)
} // end bruteforce


/// Available character sets for password generation
struct CharacterSet {
    lowercase: &'static str,
    uppercase: &'static str,
    numbers: &'static str,
    symbols: &'static str,
}

// Implementation default for the set of available characters 
impl Default for CharacterSet {
    fn default() -> Self {
        CharacterSet {
            lowercase: "abcdefghijklmnopqrstuvwxyz",
            uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            numbers: "0123456789",
            symbols: "!@#$%^&*()_+-=[]{}|;:,.<>?",
        }
    }
}

struct Generator {
    charset: Vec<char>,
    indices: Vec<usize>,
    done: bool,
}

// Generator implementation 
impl Generator {
    fn new(charset: &CharacterSet, use_lower: bool, use_upper: bool, use_numbers: bool, use_symbols: bool, length: usize) -> Self {
        let mut chars = String::new();
        
        if use_lower {
            chars.push_str(charset.lowercase);

        }
        if use_upper {
            chars.push_str(charset.uppercase);

        }
        if use_numbers {
            chars.push_str(charset.numbers);

        }
        if use_symbols {
            chars.push_str(charset.symbols);

        }   

        let charset = chars.chars().collect();

        Self {
            charset,
            indices: vec![0, length],
            done: false,
        }
    }
}


// Implement an iterator for Generator to iterate over different char combinations
// Iterator is a trait. Traits are implemented by structs, they don't exist on their own. You could also have a reference trait object (&Iterator), 
// a boxed trait object (Box<Iterator>) or an anonymous trait implementation (impl Iterator), all of which have a known sizes.
impl Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result: String = self.indices.iter().map(|&i| self.charset[i]).collect();

        for i in (0..self.indices.len()).rev() {
            if self.indices[i] + 1 < self.charset.len() {
                self.indices[i] += 1;
                break;
            } else {
                self.indices[i] = 0;
                if i == 0 {
                    self.done = true;
                }
            }
        }

        Some(result)
    }
}