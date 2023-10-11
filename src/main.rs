use figlet_rs::FIGfont;
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

fn encryption() {
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

fn decryption() {
    println!("Enter the ciphertext:");
    let mut ciphertext: String = text_io::read!("{}\n");
    println!("Enter the key(d,r) [integers and comma separated]:");
    let key: String = text_io::read!("{}\n");
    let mut iter = key.splitn(2, ',');
    let depth = iter.next().unwrap().parse::<usize>().unwrap();
    let rounds = iter.next().unwrap().parse::<i32>().unwrap();
    
    let r = Railfence::new(depth);
    let mut i: i32 = 0;
    let mut plaintext = String::new();
    ciphertext = r.decrypt(&ciphertext).unwrap();
    
    while i < rounds{
        plaintext = r.decrypt(&ciphertext).unwrap();
        ciphertext = plaintext.to_string();
        i+=1;
    }
    println!("{}", plaintext);

}

fn main(){
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Railfence Cipher");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    println!("[*] Dalmo Veras, Bruno Hideki, and Aydin Lamei - Cryptography Assignment 2 [*]");
    encryption();
    decryption();
   }
