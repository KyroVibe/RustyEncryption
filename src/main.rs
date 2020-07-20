use std::num::Wrapping;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if (&args).len() == 0 {
        println!("Needs arguments");
        return;
    }
    if (&args)[0] == "--help" {
        print_help();
        return;
    }
    if (&args).len() != 3 {
        println!("Incorrect # of params: {}", args.len());
        return;
    }

    let file_path = args[0].to_owned();
    let mut buf: Vec<u8> = Vec::new();
    let key = args[1].to_owned();
    let encrypt: bool = args[2] == "true";

    let mut f = File::open(&file_path).unwrap();
    f.read_to_end(&mut buf).expect("Whoop Idk what happened");

    fuck_wit_byte(&mut buf, key.as_bytes(), encrypt);

    let mut encrypted_file = File::create(&file_path).unwrap();
    encrypted_file.write(buf.as_slice()).unwrap();
    encrypted_file.flush().unwrap();
    
    // println!("{}", result);
}

fn fuck_wit_byte(b: &mut Vec<u8>, key: &[u8], encrypt: bool) {
    for i in 0..b.len() {
        for x in 0..key.len() {
            if encrypt {
                b[i] = (Wrapping(b[i]) + (Wrapping(key[x]) * Wrapping(key[(x + 1) % key.len()]))).0;
            } else {
                b[i] = (Wrapping(b[i]) - (Wrapping(key[x]) * Wrapping(key[(x + 1) % key.len()]))).0;
            }
        }
    }
}

fn print_help() {
    println!("Usage:\nrust_encryption [file] [key] [encrypt or decrypt (true/false)]");
}
