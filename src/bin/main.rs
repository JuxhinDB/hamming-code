use hamming_code_simd::hamming::{decode, encode};

fn main() {
    //let mut raw: u64 = 0b10101011100100001110100011110001;
    let mut raw: u64 = 0b1001;
    println!("      Raw  (2): {:064b}", raw);

    let mut code = encode(&mut raw);
    println!("Encoded:\t{:064b}", code);

    let block = decode(&mut code);
    println!("Decoded:\t{:064b}", block);
}
