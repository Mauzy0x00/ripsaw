use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self},
    time::{Duration, Instant},
};

use crate::library::{Config, ThreadPool};

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

/// Password generator with work distribution
struct Generator {
    charset: Vec<char>,
    current: Vec<usize>,
    length: usize,
    done: bool,
    thread_id: usize,
    thread_count: usize,
    counter: usize,
    skip_interval: usize,
}

#[allow(clippy::too_many_arguments)]
impl Generator {
    fn new(
        charset: &CharacterSet,
        use_lower: bool,
        use_upper: bool,
        use_numbers: bool,
        use_symbols: bool,
        length: usize,
        thread_id: usize,
        thread_count: usize,
    ) -> Self {
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

        let charset: Vec<char> = chars.chars().collect();
        let skip_interval = thread_count;

        // Start at our thread's specific offset in the sequence
        let mut current = vec![0; length];

        // Advance to our starting position based on thread_id
        for _ in 0..thread_id {
            Self::advance_indices(&mut current, &charset);
        }

        Self {
            charset,
            current,
            length,
            done: false,
            thread_id,
            thread_count,
            counter: 0,
            skip_interval,
        }
    }

    // Helper method to advance indices
    fn advance_indices(indices: &mut [usize], charset: &[char]) {
        let charset_len = charset.len();

        for i in (0..indices.len()).rev() {
            if indices[i] + 1 < charset_len {
                indices[i] += 1;
                break;
            } else {
                indices[i] = 0;
                // Continue to next position if we wrapped around
            }
        }
    }

    // Generate current password
    fn current_password(&self) -> String {
        self.current.iter().map(|&i| self.charset[i]).collect()
    }
}

impl Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Get current password
        let result = self.current_password();

        // Advance counter
        self.counter += 1;

        // Advance indices skip_interval times (to skip other threads' work)
        for _ in 0..self.skip_interval {
            let charset_len = self.charset.len();
            let mut carry = true;

            for i in (0..self.length).rev() {
                if carry {
                    if self.current[i] + 1 < charset_len {
                        self.current[i] += 1;
                        carry = false;
                    } else {
                        self.current[i] = 0;
                        // carry remains true for next position
                    }
                }
            }

            // If we carried beyond the most significant position, we're done
            if carry {
                self.done = true;
                break;
            }
        }

        Some(result)
    }
}

// Main brute force function with improved implementation
// Divide and conquer
pub fn bruteforce(
    salt: String,
    cyphertext: String,
    length: usize,
    thread_count: u8,
    hash_algorithm: fn(&str) -> String,
    config: Config,
) -> Option<String> {
    let start_time = Instant::now();
    let cracked = Arc::new(AtomicBool::new(false));
    let password_found = Arc::new(Mutex::new(None::<String>));

    // Create a thread pool
    let pool = ThreadPool::new(thread_count as usize);

    // Character set generation
    let charset = CharacterSet::default();

    // Launch worker threads
    for thread_id in 0..thread_count {
        let cracked_clone = Arc::clone(&cracked);
        let password_found_clone = Arc::clone(&password_found);
        let cyphertext_clone = cyphertext.clone();

        pool.execute(move || {
            // Create a generator for this thread's portion of the work
            let generator = Generator::new(
                &CharacterSet::default(),
                true,  // use lowercase
                true,  // use uppercase
                true,  // use numbers
                false, // skip symbols initially for speed
                length,
                thread_id as usize,
                thread_count as usize,
            );

            let mut attempts: u32 = 0;

            // Try passwords until we find a match or another thread signals success
            for attempt in generator {
                attempts += 1;

                // Check if another thread found the password
                if cracked_clone.load(Ordering::Relaxed) {
                    break;
                }

                // Calculate hash and compare
                println!("Generated {}", attempt);
                let attempt_hash = hash_algorithm(&attempt);

                if attempt_hash == cyphertext_clone {
                    // Save the found password
                    let mut found = password_found_clone.lock().unwrap();
                    *found = Some(attempt.clone());

                    // Signal other threads to stop
                    cracked_clone.store(true, Ordering::Relaxed);

                    if config.verbose {
                        println!("Password found by thread {}: {}", thread_id, attempt);
                        println!("Attempts by this thread: {}", attempts);
                    }

                    break;
                }

                // Print progress occasionally
                if config.verbose && attempts % 1_000_000 == 0 {
                    println!("Thread {} has tried {} passwords", thread_id, attempts);
                }
            }
        });
    }

    // Wait for completion or a timeout
    let timeout = Duration::from_secs(3600); // 1 hour timeout
    let mut elapsed = Duration::new(0, 0);
    let sleep_interval = Duration::from_millis(100);

    while !cracked.load(Ordering::Relaxed) && elapsed < timeout {
        thread::sleep(sleep_interval);
        elapsed += sleep_interval;
    }

    // Shutdown the pool (will wait for all threads to finish)
    pool.shutdown();

    // Return the found password if any
    let result = password_found.lock().unwrap().clone();

    if config.verbose {
        if let Some(ref pwd) = result {
            println!("Password cracked: {}", pwd);
        } else {
            println!("Password not found within timeout.");
        }
        println!("Total time: {:.2}s", start_time.elapsed().as_secs_f64());
    }

    result
}
