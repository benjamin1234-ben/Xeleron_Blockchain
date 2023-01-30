extern crate magic_crypt;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::tokens::_tokens::Token;
use crate::constants::MAGIC_KEY;

pub fn encrypt(data : &[u8; 32]) -> String {
    let mc = new_magic_crypt!(MAGIC_KEY, 256);
    
    mc.encrypt_bytes_to_base64(data)
}
    
pub fn decrypt(data : &String) -> Vec<u8> {
    let mc = new_magic_crypt!(MAGIC_KEY, 256);

    mc.decrypt_base64_to_bytes(data).unwrap()
}

pub fn hash<T : Hash>(value : &T) -> u64 {
    let mut hasher = DefaultHasher::new();

    value.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub struct Storage {
    info : Option<String>,
    value : Option<Value>
}

#[derive(Debug)]
struct Value {
    token : Option<Token>,
    amount : Option<u64>
}