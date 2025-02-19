/*
*   
*   A collection of supporting functions
*
*/


use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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