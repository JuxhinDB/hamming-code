fn main() {
    let mut raw: u64 = 0b10011010;
    println!("    Raw: {:064b}", raw);

    let mut code = encode(&mut raw);
    let block = decode(&mut code);

    println!("Encoded: {:064b}", code);
    println!("Decoded: {:064b}", block);
}

fn encode(block: &mut u64) -> u64 {
    let len_power = (2..).find(|&r| 2u32.pow(r) - r - 1 >= 32).unwrap();
    let len = 2usize.pow(len_power) - 1;

    let mut code = 0u64;

    for i in 1..len + 1 {
        // Check if `i` is not a power of 2
        if (i & (i - 1)) != 0 {
            code |= (0b1 << i) & *block as u64;
        } else {
            *block <<= 1;
        }
    }

    for i in 0..len_power {
        if parity(&code, i) {
            code |= 0b1 << (2usize.pow(i) - 1);
        } else {
            let mut mask = u64::MAX;
            mask ^= 0b1 << (2usize.pow(i) - 1);
            println!("code: {:064b}, {}", code, 2usize.pow(i) - 1);
            code &= mask;
            println!("code: {:064b}\n\n", code);
        }
    }

    code
}

fn decode(code: &mut u64) -> u64 {
    let len_power = 7;
    let mut flipped_bit = -1i32;

    let mut decoded = 0u64;

    for i in 0..len_power {
        if parity(&code, i) {
            if flipped_bit != -1 {
                *code ^= 0b1 << flipped_bit;
            }

            flipped_bit += 1;
            *code ^= 0b1 << flipped_bit;
        }
    }

    for i in 0..64 {
        if (i & (i + 1)) != 0 {
            decoded |= (0b1 << i) & *code;
        }
    }

    decoded
}

// TODO(jdb): Optimize this, remove internal bools and flags
fn parity(code: &u64, i: u32) -> bool {
    let bi = (0b1 << i) - 1;
    let (mut parity, mut ignore, mut counter) = (false, false, 0);
    for j in bi..64 {
        if !ignore && (code & 0b1 << j) != 0b0 {
            parity = !parity;
        }

        counter += 1;
        if counter > 0b1 << i {
            ignore = !ignore;
            counter = 0;
        }
    }

    parity
}
