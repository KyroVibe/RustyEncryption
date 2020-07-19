use std::num::Wrapping;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for x in &args {
        println!("{}", x);
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
    let buf_len = f.read_to_end(&mut buf).unwrap();
    
    for x in 0..buf_len {
        buf[x] = fuck_wit_byte(buf[x], key.as_bytes(), encrypt);
    }

    let mut encrypted_file = File::create(&file_path).unwrap();
    encrypted_file.write(buf.as_slice()).unwrap();
    encrypted_file.flush().unwrap();
    
    // println!("{}", result);
}

fn fuck_wit_byte(b: u8, key: &[u8], encrypt: bool) -> u8 {
    let mut new_byte = Wrapping(b);
    for x in 0..key.len() {
        if encrypt {
            new_byte += Wrapping(key[x]);
        } else {
            new_byte -= Wrapping(key[x]);
        }
    }
    return new_byte.0;
}

fn print_help() {
    println!("Usage:\nrust_encryption [file] [key] [encrypt or decrypt (true/false)]");
}
