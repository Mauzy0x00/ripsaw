/*  Ripsaw
*
*   This file contains available hashing algorthms.
*   Include or implement new algorithms here. Make sure to add the hash function to the 'get_algorithm' match and
*   the Algorithms constant array. Might need to come up with a new way to implement get_algorithm to fit other
*   algorithm implementations; else re-write those implementations to fit this design.
*   The algorithms already implemented are from the RustCrypto crate.
*
*/

use ascon_hash::AsconHash;
use belt_hash::BeltHash;
use blake2::{Blake2b512, Blake2s256};
use fsb::{Fsb160, Fsb224, Fsb256};
use gost94::{Gost94CryptoPro, Digest};
use groestl::{Groestl224, Groestl256};
use jh::{Jh224, Jh256, Jh384, Jh512};
// use k12::KangarooTwelve; -- missing need input integer 
use md2::Md2;
use md4::Md4;
use md5::compute;
use ripemd::{Ripemd160, Ripemd256, Ripemd320};
use sha1::Sha1;
// use sha1_checked::Sha1; -- missing
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use shabal::Shabal256;
// use skein::{Skein256, Skein512}; -- missing; need u32 input (is checking every u32 input/word viable?)
use sm3::Sm3;
use streebog::{Streebog256, Streebog512};
use tiger::Tiger;
use whirlpool::Whirlpool;
use yescrypt::yescrypt;

pub const ALGORITHMS: [&str; 31] = [
    "ascon",
    "belt",
    "fsb160",
    "fsb224",
    "fsb256",
    "sha256",
    "sha512",
    "md2",
    "md4",
    "md5",
    "ripemd160",
    "ripemd256",
    "ripemd320",
    "sha1",
    "sha3_256",
    "sha3_512",
    "blake2b512",
    "blake2s256",
    "gost94",
    "groestl224",
    "groestl256",
    "jh224",
    "jh256",
    "jh384",
    "jh512",
    "shabal256",
    "sm3",
    "streebog256",
    "streebog512",
    "tiger",
    "whirlpool",
];

/// Match user supplied algorithm to hashing funciton
pub fn get_algorithm(algorithm: &str) -> Option<fn(&str) -> String> {
    match algorithm {
        "ascon" => Some(hash_ascon),
        "belt" => Some(hash_belt),
        "fsb160" => Some(hash_fsb160),
        "fsb224" => Some(hash_fsb224),
        "fsb256" => Some(hash_fsb256),
        "sha256" => Some(hash_sha256),
        "sha512" => Some(hash_sha512),
        "md2" => Some(hash_md2),
        "md4" => Some(hash_md4),
        "md5" => Some(hash_md5),
        "ripemd160" => Some(hash_ripemd160),
        "ripemd256" => Some(hash_ripemd256),
        "ripemd320" => Some(hash_ripemd320),
        "sha1" => Some(hash_sha1),
        "sha3_256" => Some(hash_sha3_256),
        "sha3_512" => Some(hash_sha3_512),
        "blake2b512" => Some(hash_blake2b512),
        "blake2s256" => Some(hash_blake2s256),
        "gost94" => Some(hash_gost94),
        "groestl224" => Some(hash_groestl224),
        "groestl256" => Some(hash_groestl256),
        "jh224" => Some(hash_jh224),
        "jh256" => Some(hash_jh256),
        "jh384" => Some(hash_jh384),
        "jh512" => Some(hash_jh512),
        "shabal256" => Some(hash_shabal256),
        "sm3" => Some(hash_sm3),
        "streebog256" => Some(hash_streebog256),
        "streebog512" => Some(hash_streebog512),
        "tiger" => Some(hash_tiger),
        "whirlpool" => Some(hash_whirlpool),
        _ => None,
    }
}

fn hash_ascon(input: &str) -> String {
    let mut hasher = AsconHash::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_belt(input: &str) -> String {
    let mut hasher = BeltHash::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_blake2b512(input: &str) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_blake2s256(input: &str) -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_fsb160(input: &str) -> String {
    let mut hasher = Fsb160::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_fsb224(input: &str) -> String {
    let mut hasher = Fsb224::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_fsb256(input: &str) -> String {
    let mut hasher = Fsb256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_gost94(input: &str) -> String {
    let mut hasher = Gost94CryptoPro::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_groestl224(input: &str) -> String {
    let mut hasher = Groestl224::default();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_groestl256(input: &str) -> String {
    let mut hasher = Groestl256::default();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_jh224(input: &str) -> String {
    let mut hasher = Jh224::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_jh256(input: &str) -> String {
    let mut hasher = Jh256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_jh384(input: &str) -> String {
    let mut hasher = Jh384::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_jh512(input: &str) -> String {
    let mut hasher = Jh512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// fn hash_k12(input: &str) -> String {
//     let mut hasher = KangarooTwelve::default();
//     hasher.update(input.as_bytes());
//     format!("{:x}", hasher.finalize())
// }

fn hash_md2(input: &str) -> String {
    let mut hasher = Md2::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_md4(input: &str) -> String {
    let mut hasher = Md4::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_md5(input: &str) -> String {
    let hashed = compute(input);
    format!("{:x}", hashed)
}

fn hash_ripemd160(input: &str) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_ripemd256(input: &str) -> String {
    let mut hasher = Ripemd256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_ripemd320(input: &str) -> String {
    let mut hasher = Ripemd320::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}


fn hash_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_sha3_256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_sha3_512(input: &str) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_shabal256(input: &str) -> String {
    let mut hasher = Shabal256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// fn hash_skein256(input: &str) -> String {
//     let mut hasher = Skein256::new();
//     hasher.update(input.as_bytes());
//     format!("{:x}", hasher.finalize())
// }

fn hash_sm3(input: &str) -> String {
    let mut hasher = Sm3::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_streebog256(input: &str) -> String {
    let mut hasher = Streebog256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_streebog512(input: &str) -> String {
    let mut hasher = Streebog512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_tiger(input: &str) -> String {
    let mut hasher = Tiger::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_whirlpool(input: &str) -> String {
    let mut hasher = Whirlpool::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// fn hash_yescrypt(input: &str) -> String {
//     // Input will be the next word in the wordlist.
//     // -- We need to get the parameters and salt. 
//     // Remember... simpliest solution is likely the greatest solution
//     let salt = b"salt";
//     let params = b"params";
//     yescrypt::yescrypt_kdf(input, salt)?;
//     Ok(())
// }