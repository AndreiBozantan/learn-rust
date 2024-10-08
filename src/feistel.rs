use std::io::Write;

use anyhow::{ensure, Result};
use sha2::{Sha256, Digest};


fn main() -> Result<()> {
    const NUMBER_OF_ROUNDS : i32 = 19;

    let key = "ThisIsASecretKey";
    let plain_text = "Hello amazing world! It's a beautiful day!";

    let cipher_bytes = feistel(plain_text.as_bytes(), key.as_bytes(), NUMBER_OF_ROUNDS)?;
    let cipher_text = cipher_bytes.iter().map(|b| format!("{:02X}", b)).collect::<String>();

    let decrypted_bytes = feistel(cipher_bytes.as_slice(), key.as_bytes(), NUMBER_OF_ROUNDS)?;
    let decrypted_text = String::from_utf8(decrypted_bytes)?;

    println!("Plaintext: {plain_text}");
    println!("Cipher text: {cipher_text}");
    println!("Decrypted string: {decrypted_text}");

    assert_eq!(plain_text, decrypted_text);

    Ok(())
}

// The Feistel cipher is designed to be reversible, even when using a non-reversible one-way hash function.
fn feistel(input: &[u8], key: &[u8], rounds: i32) -> Result<Vec<u8>> {
    ensure!(input.len() % 2 == 0, "Input length must be even.");

    let half_len = input.len() / 2;
    let mut left = input[..half_len].to_vec();
    let mut right = input[half_len..].to_vec();
    let mut new_right;

    for _ in 0..rounds {
        // generate a hash of the right side and truncate it to half the length
        let hash = hash(&right, &key);
        let truncated_hash = hash[..half_len].to_vec();

        // xor the left side with the truncated hash
        new_right = xor(&left, &truncated_hash);

        // swap the left and right sides
        left = right;
        right = new_right;
    }

    // swap the left and right sides one last time
    new_right = right;
    right = left;
    left = new_right;

    // concatenate the left and right sides
    left.extend(right);
    Ok(left)
}

fn hash(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut h = Sha256::new();
    h.write_all(input).unwrap();
    h.write_all(key).unwrap();
    h.finalize().to_vec()
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}