use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{IoSlice, Write};

use rand::Rng;

use hamming_code_simd::hamming::{decode, encode};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1])?;
    let mut o = File::create(format!("{}.bak", &args[1]))?;
    let mut n = File::create(format!("{}.dmg", &args[1]))?;

    let mut output_bytes: Vec<[u8; 8]> = vec![];
    let mut noisy_bytes: Vec<[u8; 8]> = vec![];

    let mut buffer = [0; 8];
    let mut rng = rand::thread_rng();

    // For each 8 bytes (64 bits) we can only consume
    // up to 7 bytes (56 bits) to give way for parity
    // bits.
    while let Ok(b) = f.read(&mut buffer[..7]) {
        // Not sure why f.read doesn't stop?
        //
        // This is definitely a bug and why we are only
        // processing 7168 bytes.
        if b == 0 {
            break;
        }

        let original = u64::from_le_bytes(buffer);
        let mut block = original;
        println!("Block:   {:064b}", block);

        let mut encoded = encode(&mut block);
        println!("Encoded: {:064b}", encoded);

        if rng.gen_bool(2.0 / 3.0) {
            let invalid_bit = rng.gen_range(0..56);
            let mask: u64 = 0b1 << invalid_bit;
            println!("flipping bit: {}", invalid_bit);

            // Toggle that specific bit
            encoded ^= mask;
            noisy_bytes.push(encoded.to_le_bytes());
        } else {
            noisy_bytes.push(encoded.to_le_bytes());
        }

        let decoded = decode(&mut encoded);
        println!("Decoded: {:064b}", decoded);

        println!("Match?   {}\n", original == decoded);
        output_bytes.push(decoded.to_le_bytes());

        // Clear buffer for next block
        buffer = [0; 8];
    }

    o.write_vectored(
        &output_bytes
            .iter()
            .map(|b| IoSlice::new(&b[..b.len() - 1]))
            .collect::<Vec<IoSlice>>(),
    )?;

    n.write_vectored(
        &noisy_bytes
            .iter()
            .map(|b| IoSlice::new(&b[..b.len() - 1]))
            .collect::<Vec<IoSlice>>(),
    )?;

    Ok(())
}
