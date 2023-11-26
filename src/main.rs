#![feature(bigint_helper_methods)]

use std::ops::Add;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

mod block;

use block::Block;

struct Elegabalus;

impl Distribution<Block> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Block {
        rng.gen::<u64>().into()
    }
}

fn convert(msg: Vec<u8>) -> Vec<Block> {
    // Pad the message with zeros to make it a multiple of 8 bytes
    let msg = pad(msg);
    msg.chunks_exact(8).map(|c| {
        let mut b = [0u8; 8];
        b.copy_from_slice(c);
        Block(u64::from_be_bytes(b))
    }).collect()
}

fn pad(msg: Vec<u8>) -> Vec<u8> {
    let mut msg = msg;
    let len = msg.len();
    let rem = len % 8;
    if rem != 0 {
        msg.append(&mut vec![0u8; 8 - rem]);
    }
    msg
}

fn convert_block(msg: Vec<Block>) -> Vec<u8> {
    msg.iter().flat_map(|b| {
        let mut bytes = b.0.to_be_bytes().to_vec();
        bytes.truncate(8);
        bytes
    }).collect()
}

const KEY_SIZE: usize = 16;

type KEY = [Block; KEY_SIZE];

impl Elegabalus {
    fn polynomial(slice: &[Block; KEY_SIZE]) -> Block {
        slice.iter().enumerate().map(|(i, b)| {
            let index: u32 = i as u32;  //Iterate 1..=16
            b.pow(index + 1)
        }).reduce(Add::add).unwrap()
    }

    fn encrypt(key: KEY, cleartext: Vec<Block>) -> Vec<Block> {
        let mut encryption_vector: Vec<Block> = key.to_vec();
        for (i, block) in cleartext.iter().enumerate() {
            let key_stream = Self::polynomial(encryption_vector[i..i + KEY_SIZE].try_into().unwrap());
            let cipher_block = *block ^ key_stream;
            encryption_vector.push(cipher_block);
        }
        encryption_vector[16..].to_vec()
    }

    fn decrypt(key: KEY, ciphertext: Vec<Block>) -> Vec<Block> {
        let mut encryption_vector: Vec<Block> = key.to_vec();
        encryption_vector.append(&mut ciphertext.to_vec());

        let mut decryption_vector: Vec<Block> = Vec::new();

        for (i, block) in ciphertext.iter().enumerate() {
            let key_stream = Self::polynomial(encryption_vector[i..i + KEY_SIZE].try_into().unwrap());
            let cleartext_block = *block ^ key_stream;
            decryption_vector.push(cleartext_block);
        }
        decryption_vector
    }
}

fn main() {
    let key: KEY = rand::random();
    let msg = "Hello World!".as_bytes().to_vec();
    let msg = convert(msg);
    println!("Key: {:?}", &key);
    println!("Plaintext: {:?}", &msg);
    let cipher = Elegabalus::encrypt(key, msg);
    println!("Ciphertext: {:?}", &cipher);
    let plain = Elegabalus::decrypt(key, cipher);
    println!("Plaintext: {:?}", &plain);
    let msg: String = String::from_utf8(convert_block(plain)).unwrap();
    println!("{}", msg);
}
