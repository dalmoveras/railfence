use text_io;
pub mod railfence;

use railfence::Railfence;
use clap::Parser;

#[derive(Parser)]
struct Args {
    plaintext: String,
    depth: usize,
    repeat: i32,
}

fn main(){
    println!("[*] Railfence Cipher - Cryptography Assignment 2 [*]");

    println!("Enter the plaintext:");
    let plaintext: String = text_io::read!("{}\n"); 
    println!("Enter the key(d,r) [integers and comma separated]:"); 
    let key: String = text_io::read!("{}\n"); 

    let r = Railfence::new();
    let ciphertext = r.encrypt(&plaintext).unwrap();
    println!("{} ---> {}", plaintext, ciphertext);
}
