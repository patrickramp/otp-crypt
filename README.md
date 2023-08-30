# otp-crypt
Command line tool for encrypting .txt files using One-Time Pad (OTP) cipher and printable characters. 

## Usage
This command line program looks for specifically named .txt files in the 
same directory. Please make sure your files are named correctly. 
*key.txt, plaintext.txt, ciphertext.txt.**

**key.txt and plaintext.txt** are required for **encryption** and will produce ciphertext.txt.
**key.txt and ciphertext.txt** are required for **decryption** and will produce decrypt.txt

This application will also generate a new key for you with the same length as your plaintext, if required.


## WARNING 
This tool has not been audited. Although OTP encryption should be secure, my application probably is not. 
DO NOT USE FOR SENSITIVE OR CRITICAL ENCRYPTION USE CASES. 