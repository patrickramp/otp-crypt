use std::path::Path;
use lib::*;

fn main() {
    println!("=================================");
    println!("    Welcome to the OTP-Crypt!   ");
    println!("=================================");
    println!(" [1] - Encrypt plaintext.txt");
    println!(" [2] - Decrypt ciphertext.txt");
    println!(" [3] - Create new key & Encrypt");
    println!(" [4] - Create new key.txt only.");
    println!(" [5] - Exit");
    println!("=================================");
    println!(" Please select an option above. ");

    loop {
        menu()
    }
}

fn menu() {
    // Get user's choice.
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim().parse::<u8>() {
        Ok(1) => {
            encrypton()
        }
        Ok(2) => {
            decrypton()
        }
        Ok(3) => {
            new_pad();
            encrypton()
        }
        Ok(4) => {
            new_pad()
        }
        Ok(5) => {
            println!("Exiting");
            std::process::exit(0);
        }
        _ => {
            println!("Invalid choice, please try again");
        }
    }
}
// Encrypt plaintext.txt using the one-time pad.
fn encrypton() {
    // Define printable characters.
    let printable_chars = char_vec();
    // Unused bind_name option for future use.
    let bind_name = "key.txt";
    let key_path = Path::new(bind_name);
    let key = read(key_path).unwrap();
    // Unused bind_name option for future use.
    let bind_name = "plaintext.txt";
    let plaintext_path = Path::new(bind_name);
    let plaintext = read(plaintext_path).unwrap();
    let ciphertext = encrypt(&plaintext, &key, &printable_chars);
    // Write ciphertext to ciphertext.txt, unused bind_name option for future use.
    let bind_name = "ciphertxt.txt";
    let ciphertext_path = Path::new(&bind_name);
    match write(ciphertext_path, &ciphertext) {
        Ok(_) => {
            println!("Tex encrypted successfully.");
            // Print key-ciphertext pair fingerprint.
            println!("Fingerprint:{}",fingerprint(&key, &ciphertext))
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

}
// Decrypt ciphertext.txt using the one-time pad.
fn decrypton() {
    // Unused bind_name option for future use.
    let bind_name = "key.txt";
    let key_path = Path::new(bind_name);
    let printable_chars = char_vec();
    let key = read(key_path).unwrap();
    // Unused bind_name option for future use.
    let bind_name = "ciphertxt.txt";
    let ciphertext_path = Path::new(&bind_name);
    let ciphertext = read(ciphertext_path).unwrap();
    let decrypted_text = decrypt(&ciphertext, &key, &printable_chars);
    // Write decrypted text to decrypted.txt, unused bind_name option for future use.
    let bind_name = "decrypted.txt";
    let decrypted_path = Path::new(&bind_name);
    match write(decrypted_path, &decrypted_text) {
        Ok(_) => {
            println!("Text decrypted successfully.");
            println!("Fingerprint:{}",fingerprint(&key, &ciphertext))
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn new_pad(){
    // Generate a one-time key with a length of the plaintext file.
    let plaintext_path = Path::new("plaintext.txt");
    let plaintext = read(plaintext_path).unwrap();
    match keygen(plaintext) {
        Ok(_) => {
            println!("One-Time Pad generated successfully.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}