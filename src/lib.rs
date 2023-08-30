use crypto_hash::{Algorithm, hex_digest};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::io;
use rand::rngs::OsRng;
use rand::Rng;

// Function that returns a vector of printable characters.
pub fn char_vec() -> Vec<char> {
    // Set printable characters as a vector and paths for the plaintext, key, ciphertext, and decrypted files
    let mut printable_chars: Vec<char> = (32..=126).map(|c| c as u8 as char).collect();
    printable_chars.push(10 as char); // Add newline char.
    printable_chars.push(13 as char); // Add carriage return char.
    printable_chars
}
// Decrypt the ciphertext using the one-time pad
pub fn decrypt(ciphertext: &str, key: &str, printable_chars: &[char]) -> String {
    let decrypted: String = ciphertext
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| {
            // Find the index of the character in the ciphertext and key in the printable_chars vector
            let ciphertext_index = find_char_index(c, &printable_chars);
            let key_index = find_char_index(k, &printable_chars);

            // Check if both indices are valid
            match (ciphertext_index, key_index) {
                (Some(c_index), Some(k_index)) => {
                    // Perform the decryption operation: (ciphertext_index + printable_chars.len() - key_index) % printable_chars.len()
                    let decrypted_index =
                        (c_index + printable_chars.len() - k_index) % printable_chars.len();
                    // Retrieve the decrypted character from the printable_chars vector
                    printable_chars[decrypted_index]
                }
                _ => panic!("Invalid character in ciphertext or key"),
            }
        })
        .collect();
    decrypted
}

// Encrypt the plaintext using the one-time pad
pub fn encrypt(plaintext: &str, key: &str, printable_chars: &[char]) -> String {
    let ciphertext: String = plaintext
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| {
            // Find the index of the character in the plaintext and key in the printable_chars vector.
            let plaintext_index = find_char_index(c, &printable_chars);
            let key_index = find_char_index(k, &printable_chars);

            // Check if both indices are valid
            match (plaintext_index, key_index) {
                (Some(p_index), Some(k_index)) => {
                    // Perform the encryption operation: (plaintext_index + key_index) % printable_chars.len()
                    let encrypted_index = (p_index + k_index) % printable_chars.len();
                    // Retrieve the encrypted character from the printable_chars vector
                    printable_chars[encrypted_index]
                }
                _ => panic!("Invalid character in plaintext or key"),
            }
        })
        .collect();
    ciphertext
}

// Function to find the index of a character in a vector.
pub fn find_char_index(c: char, chars: &[char]) -> Option<usize> {
    // Use the `position` method to find the index of the character in the vector
    chars.iter().position(|&x| x == c)
}

// Create a 32 character SHA256 hash fingerprint to identify key-text pair.
pub fn fingerprint(key: &str, text: &str) -> String {
    // Get the SHA256 hash of the key-text.
    let full_pair = key.to_owned() + text;
    let algorithm = Algorithm::SHA256;
    let hash = hex_digest(algorithm, full_pair.as_bytes());
    // Return the first 8 characters of the hash as a string.
    hash[..32].to_string()
}

// Generate a one-time pad key with length equal to the plaintext.
pub fn keygen(plaintext: String) -> io::Result<()> {
    let key: String = (0..plaintext.len())
        .map(|_| {
            let rand_char = OsRng.gen_range::<u8, _>(30..=126);
            // Add character mapping for new line 10 and carriage return 13.
            if rand_char == 30 {
                (rand_char - 20) as char
            } else if rand_char == 31 {
                (rand_char - 18) as char
            } else {
                rand_char as char
            }
        })
        .collect();

    // Write key to key.txt
    let bind_name = "key.txt";
    let new_key_path = Path::new(&bind_name);
    match write(&new_key_path, &key) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e)
        }
    }
    Ok(())
}

// Function to read to a string from a file.
pub fn read(file_path: &Path) -> io::Result<String> {
    let text_file = File::open(file_path)?;
    let mut text_reader = BufReader::new(text_file);
    let mut text_string = String::new();
    text_reader.read_to_string(&mut text_string)?;
    Ok(text_string)
}

// Function to write a string to a file.
pub fn write(file_path: &Path, text: &str) -> io::Result<()> {
    let mut text_file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?,
    );
    text_file.write_all(text.as_bytes())?;
    Ok(())
}


