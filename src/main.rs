use clap::{Arg, Command, Parser};
use sha2::{Sha512, Digest, Sha256};
use md5;
use base64;
use std::str;
use md5::Md5;
use rand::distr::Alphanumeric;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    create: bool,
    #[arg(short, long, default_value_t = 8)]
    length: usize,
    #[arg(long)]
    md5: bool,
    #[arg(long)]
    sha256: bool,
    #[arg(long)]
    sha512: bool,
    #[arg(long)]
    xor: bool,
    #[arg(long)]
    ceaser: bool,
}

fn help() {
    println!();
    println!("# SHA256: ./kei -c --sha256");
    println!("# SHA512: ./kei -c --sha512");
    println!("# MD5: ./kei -c --md5");
    println!();
}

fn main() {
    let args = Arguments::parse();
    if args.create {
        let password = generate_password(args.length);
        println!("Generate password:\n   {}", password);
        if args.sha512 {
            let mut hasher = Sha512::new();
            hasher.update(password.as_bytes());
            let hash = hasher.finalize();
            println!("Generated SHA 512:\n   {}", hex::encode(hash));
        } else if args.sha256 {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let hash = hasher.finalize();
            println!("Generated SHA 256:\n   {}", hex::encode(hash));
        } else if args.md5 {
            let mut hasher = Md5::new();
            hasher.update(password.as_bytes());
            let hash = hasher.finalize();
            println!("Generated MD5:\n   {}", hex::encode(hash));
        } else {
            help()
        }
    } else {
        help()
    }
}


fn generate_password(length: usize) -> String {
    use rand::{thread_rng, Rng};
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn xor_encrypt(input: &str, key: &str) -> String {
    input.chars()
        .zip(key.chars().cycle())
        .map(|(c1, c2)| ((c1 as u8) ^ (c2 as u8)) as char)
        .collect()
}

fn caesar_cipher(input: &str, shift: i32) -> String {
    input.chars()
        .map(|c| {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            ((c as u8 - base + shift as u8) % 26 + base) as char
        })
        .collect()
}
