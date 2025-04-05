/*
*   
*   A collection of supporting functions
*
*/

use std::{
    sync::{atomic::{AtomicBool, Ordering}, Arc, Condvar, Mutex}, 
    thread::{self}, 
    time::Duration,
};
use std::collections::VecDeque;

/// Iterate over the passed vector and hash the string at that index before checking to see if it matches the
/// passed cypher text. If it does, return that a match has been found (cracked)
// Refactored function to increase readability of the large wordlist crack function
pub fn crack_vector(lines: Vec<String>, cyphertext:String, hash_algorithm:fn(&str)->String, cracked:&Arc<AtomicBool<>>) -> bool {
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


// Thread Pool implementation
type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Arc<(Mutex<VecDeque<Task>>, Condvar)>,
    running: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(num_workers: usize) -> Self {
        let task_queue: VecDeque<Box<dyn FnOnce() + Send>> = VecDeque::new();
        let sender = Arc::new((Mutex::new(task_queue), Condvar::new()));
        let running = Arc::new(AtomicBool::new(true));
        let mut workers = Vec::with_capacity(num_workers);

        for _ in 0..num_workers {
            let sender_clone = sender.clone();
            let running_clone = running.clone();
            
            let worker = thread::spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    let task = {
                        let (lock, condition_variable) = &*sender_clone;
                        let mut queue = lock.lock().unwrap();

                        // Wait for a task or until shutdown
                        while queue.is_empty() && running_clone.load(Ordering::Relaxed) {

                            queue = condition_variable.wait_timeout(queue, Duration::from_millis(100)).unwrap().0;
                            if !running_clone.load(Ordering::Relaxed) && queue.is_empty() {
                                return;
                            }
                        }

                        queue.pop_front()
                    };

                    if let Some(task) = task {
                        task();
                    }
                }
            });
            
            workers.push(worker);
        }

        ThreadPool { workers, sender, running }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        let (lock, cvar) = &*self.sender;
        let mut queue = lock.lock().unwrap();
        queue.push_back(task);
        cvar.notify_one();
    }

    pub fn shutdown(self) {
        // Signal threads to stop
        self.running.store(false, Ordering::Relaxed);
        
        // Wake up all threads
        let (_, condition_variable) = &*self.sender;
        condition_variable.notify_all();
        
        // Wait for all threads to finish
        for worker in self.workers {
            let _ = worker.join();
        }
    }
}