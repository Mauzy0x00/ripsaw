/*  Ripsaw
*
*   Uses the clap library to parse user CLI input.
*   Args: Single command arguments that produce an output
*   Commands: These are command 'modes' that take many inputs to perform a task and output to the user.
*             Some of the arguments for each mode are optional; Defined with 'default_value = "foobar"'
*/


use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Args {
    // the subcommands for different modules
    #[command(subcommand)]
    pub command: Option<Commands>,

    // global commands
    #[arg(
        short = 'l',
        long = "list",
        help = "List available hashing algorithms."
    )]
    pub list: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Bruteforce attack mode
    Bruteforce {
        #[arg(short = 'c', long = "cyphertext", help = "Path to the encrypted text.")]
        cyphertext_path: PathBuf,

        #[arg(
            short = 't',
            long = "threads",
            default_value = "10",
            help = "Number of threads."
        )]
        thread_count: u8,

        #[arg(short = 'm', long = "min-length", help = "Minimum password length.")]
        min_length: usize,

        #[arg(short = 'a', long = "algorithm", help = "Hashing algorithm to use.")]
        algorithm: String,

        #[arg(
            short = 's',
            long = "salt",
            default_value = "",
            help = "String to prefix each generated word."
        )]
        salt: String,

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

        #[arg(
            short = 's',
            long = "salt",
            default_value = "",
            help = "String to prefix each item in the given wordlist."
        )]
        salt: String,

        #[arg(
            short = 't',
            long = "threads",
            default_value = "10",
            help = "Number of threads."
        )]
        thread_count: u8,

        #[arg(short = 'v', long = "verbose", help = "Verbose output.")]
        verbose: bool,
    },
    /// Generate Mode
    Generate {
        // so.. generate is a mode that will take in a plaintext as well as a cypher and generate a
        // cypher text using the selected cipher.
        // -o will output to <user>.txt
        #[arg(short = 'p', long = "plaintext", help = "Plaintext to encyrpt.")]
        plaintext: String,

        #[arg(short = 'a', long = "algorithm", help = "Hashing algorithm to use.")]
        algorithm: String,

        #[arg(
            short = 'o',
            long = "output",
            default_value = None,
            help = "Name of the file to output cyphertext to."
        )]
        output_path: Option<PathBuf>,
    },
}
