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


/// Hashmap to match user supplied algorithm to hashing funciton
use std::collections::HashMap;
pub fn get_algorithm(algorithm: &str) -> Option<fn(&str) -> String> {
    let mut algorithms: HashMap<&str, fn(&str) -> String> = HashMap::new();

    algorithms.insert("ascon", hash_ascon);
    algorithms.insert("belt", hash_belt);
    algorithms.insert("fsb160", hash_fsb160);
    algorithms.insert("fsb224", hash_fsb224);
    algorithms.insert("fsb256", hash_fsb256);
    algorithms.insert("sha256", hash_sha256);
    algorithms.insert("sha512", hash_sha512);
    algorithms.insert("md2", hash_md2);
    algorithms.insert("md4", hash_md4);
    algorithms.insert("md5", hash_md5);
    algorithms.insert("ripemd160", hash_ripemd160);
    algorithms.insert("ripemd256", hash_ripemd256);
    algorithms.insert("ripemd320", hash_ripemd320);
    algorithms.insert("sha1", hash_sha1);
    algorithms.insert("sha3_256", hash_sha3_256);
    algorithms.insert("sha3_512", hash_sha3_512);
    algorithms.insert("blake2b512", hash_blake2b512);
    algorithms.insert("blake2s256", hash_blake2s256);
    algorithms.insert("gost94", hash_gost94);
    algorithms.insert("groestl224", hash_groestl224);
    algorithms.insert("groestl256", hash_groestl256);
    algorithms.insert("jh224", hash_jh224);
    algorithms.insert("jh256", hash_jh256);
    algorithms.insert("jh384", hash_jh384);
    algorithms.insert("jh512", hash_jh512);
    algorithms.insert("shabal256", hash_shabal256);
    algorithms.insert("sm3", hash_sm3);
    algorithms.insert("streebog256", hash_streebog256);
    algorithms.insert("streebog512", hash_streebog512);
    algorithms.insert("tiger", hash_tiger);
    algorithms.insert("whirlpool", hash_whirlpool);

    algorithms.get(algorithm).copied()
}

pub fn hash_ascon(input: &str) -> String {
    let mut hasher = AsconHash::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_belt(input: &str) -> String {
    let mut hasher = BeltHash::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_blake2b512(input: &str) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_blake2s256(input: &str) -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_fsb160(input: &str) -> String {
    let mut hasher = Fsb160::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_fsb224(input: &str) -> String {
    let mut hasher = Fsb224::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_fsb256(input: &str) -> String {
    let mut hasher = Fsb256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_gost94(input: &str) -> String {
    let mut hasher = Gost94CryptoPro::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_groestl224(input: &str) -> String {
    let mut hasher = Groestl224::default();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_groestl256(input: &str) -> String {
    let mut hasher = Groestl256::default();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_jh224(input: &str) -> String {
    let mut hasher = Jh224::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_jh256(input: &str) -> String {
    let mut hasher = Jh256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_jh384(input: &str) -> String {
    let mut hasher = Jh384::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_jh512(input: &str) -> String {
    let mut hasher = Jh512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// fn hash_k12(input: &str) -> String {
//     let mut hasher = KangarooTwelve::default();
//     hasher.update(input.as_bytes());
//     format!("{:x}", hasher.finalize())
// }

pub fn hash_md2(input: &str) -> String {
    let mut hasher = Md2::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_md4(input: &str) -> String {
    let mut hasher = Md4::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_md5(input: &str) -> String {
    let hashed = compute(input);
    format!("{:x}", hashed)
}

pub fn hash_ripemd160(input: &str) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_ripemd256(input: &str) -> String {
    let mut hasher = Ripemd256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_ripemd320(input: &str) -> String {
    let mut hasher = Ripemd320::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}


pub fn hash_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_sha3_256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_sha3_512(input: &str) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_shabal256(input: &str) -> String {
    let mut hasher = Shabal256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// fn hash_skein256(input: &str) -> String {
//     let mut hasher = Skein256::new();
//     hasher.update(input.as_bytes());
//     format!("{:x}", hasher.finalize())
// }

pub fn hash_sm3(input: &str) -> String {
    let mut hasher = Sm3::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_streebog256(input: &str) -> String {
    let mut hasher = Streebog256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_streebog512(input: &str) -> String {
    let mut hasher = Streebog512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_tiger(input: &str) -> String {
    let mut hasher = Tiger::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_whirlpool(input: &str) -> String {
    let mut hasher = Whirlpool::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}