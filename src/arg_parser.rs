




use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Bruteforce attack mode
    Bruteforce {
        #[arg(short = 'c', long = "cyphertext", help = "Path to the encrypted text.")]
        cyphertext_path: PathBuf,

        #[arg(short = 't', long = "threads", help = "Number of threads.")]
        thread_count: u8,

        #[arg(short = 'm', long = "min-length", help = "Minimum password length.")]
        min_length: usize,

        #[arg(short = 'a', long = "algorithm", help = "Hashing algorithm to use.")]
        algorithm: String,

        #[arg(short = 'v', long = "verbose", help = "Verbose output.")]
        verbose: bool,
    },

    /// Wordlist attack mode
    Wordlist {
        #[arg(short = 'c', long = "cyphertext", help = "Path to the encrypted text.")]
        cyphertext_path: PathBuf,

        #[arg(short = 'w', long = "wordlist", help = "Path to the wordlist.")]
        wordlist_path: PathBuf,

        #[arg(short = 'a', long = "algorithm", help = "Hashing algorithm to use.")]
        algorithm: String,

        #[arg(short = 't', long = "threads", help = "Number of threads.")]
        thread_count: u8,

        #[arg(short = 'v', long = "verbose", help = "Verbose output.")]
        verbose: bool,
    },
}