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
    let mut iter = key.splitn(2, ',');
    let depth = iter.next().unwrap().parse::<usize>().unwrap();
    let rounds = iter.next().unwrap().parse::<i32>().unwrap();
    
    let r = Railfence::new(depth);
    
    let mut ciphertext =r.encrypt(&plaintext).unwrap();

    let mut i: i32 = 0;
    let mut current: String;
    
    while i < rounds {
        current = r.encrypt(&ciphertext).unwrap();
        ciphertext = current.to_string();
        i+=1;
    }
    println!("{}", ciphertext);
}
