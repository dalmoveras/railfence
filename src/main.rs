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
    let args = Args::parse();
    let plaintext = args.plaintext;
    let key = (args.depth, args.repeat);
    
    let r = Railfence::new(args.depth);
    let ciphertext = r.encrypt(&plaintext).unwrap();
    println!("{} ---> {}", plaintext, ciphertext);
}
