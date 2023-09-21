extern crate sodiumoxide;
extern crate base32;

use std::env;
use std::fs::File;
use std::io::Write;
use sodiumoxide::crypto::box_::gen_keypair;
use base32::Alphabet;
use base32::encode;

fn key_str(key: &[u8]) -> String {
    // Encode the key to base32 with uppercase letters
    let key_b32 = encode(Alphabet::RFC4648 { padding: false }, key);
    key_b32
}

fn main() {
    sodiumoxide::init().expect("Sodiumoxide initialization failed");

    // Get the name and onion address from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <name> <onion_address>", args[0]);
        std::process::exit(1);
    }
    let name = &args[1];
    let mut onion_address = args[2].clone(); // Clone to convert to String

    // Strip the .onion suffix if it's present
    if onion_address.ends_with(".onion") {
        onion_address = onion_address[..onion_address.len() - 6].to_string();
    }

    // Generate a keypair
    let (public_key, secret_key) = gen_keypair();

    // Convert keys to base32 strings
    let public_key_str = key_str(public_key.as_ref());
    let secret_key_str = key_str(secret_key.as_ref());

    // Prepend the keys with the specified prefixes
    let public_key_with_prefix = format!("descriptor:x25519:{}", public_key_str);
    let private_key_with_prefix = format!(
        "{}:descriptor:x25519:{}",
        onion_address, secret_key_str
    );

    // Create and write to the public key file
    let public_key_filename = format!("{}.auth", name);
    let mut public_key_file = File::create(&public_key_filename)
        .expect("Failed to create public key file");
    public_key_file
        .write_all(public_key_with_prefix.as_bytes())
        .expect("Failed to write to public key file");

    // Create and write to the private key file
    let private_key_filename = format!("{}_onion.auth_private", name);
    let mut private_key_file = File::create(&private_key_filename)
        .expect("Failed to create private key file");
    private_key_file
        .write_all(private_key_with_prefix.as_bytes())
        .expect("Failed to write to private key file");
        
    println!("{} public key:  {}", name, public_key_str);
    println!("{} private key: {}", name, secret_key_str);
    println!("Public key saved to {}", public_key_filename);
    println!("Private key saved to {}", private_key_filename);
}

