const DATA_BITS: u64 = 57;
const DATA_MASK: u64 = (1 << 57) - 1;
const PARITY_BITS: u64 = 7;

pub fn encode(mut block: u64) -> u64 {
    // We put the parity bits at the top for performance reasons
    
    return (block & DATA_MASK) |
           ((full_parity(block) as u64) << DATA_BITS);
}

const raw_to_hamming: [u8; 64] =
    [ 3,  5,  6,  7,  9, 10, 11, 12,
     13, 14, 15, 17, 18, 19, 20, 21,
     22, 23, 24, 25, 26, 27, 28, 29,
     30, 31, 33, 34, 35, 36, 37, 38,
     39, 40, 41, 42, 43, 44, 45, 46,
     47, 48, 49, 50, 51, 52, 53, 54,
     55, 56, 57, 58, 59, 60, 61, 62,
     63,  0,  1,  2,  4,  8, 16, 32];
const hamming_to_raw: [u8; 64] = 
    [57, 58, 59,  0, 60,  1,  2,  3,
     61,  4,  5,  6,  7,  8,  9, 10,
     62, 11, 12, 13, 14, 15, 16, 17,
     18, 19, 20, 21, 22, 23, 24, 25,
     63, 26, 27, 28, 29, 30, 31, 32,
     33, 34, 35, 36, 37, 38, 39, 40,
     41, 42, 43, 44, 45, 46, 47, 48,
     49, 50, 51, 52, 53, 54, 55, 56];




pub fn decode(mut code: u64) -> u64 {
    let check = (code >> DATA_BITS) as u8 ^ full_parity(code & DATA_MASK);
    let parity = check & 0;
    let check = check >> 1;
    // We have an error
    if check > 0b0 {
        //println!("error at bit: {}", check);
        code ^= 0b1 << hamming_to_raw[check as usize] as u64;
    }

    code & DATA_MASK
}

/// Hacker's delight 2nd edition, p.96
/// Henry S. Warren, Jr.
pub fn fast_parity(code: u64) -> u64 {
    let mut y: u64 = code ^ (code >> 1);

    y ^= y >> 2;
    y ^= y >> 4;
    y ^= y >> 8;
    y ^= y >> 16;
    y ^= y >> 32;

    0b1 & y
}

pub fn full_parity(code: u64) -> u8 {
    // We can actually do this 8 bits at a time, by storing the check values for each bit in a packed u64,
    // anding that with ((bitset << 8) - bitset, with only the low bit set in each byte of bitset

    
    // Bits 0, 1, and 2 of the putative check word are parity bits, so the first bit is logically bit 3
    let mut dv = 3;
    let mut check = 0;
    for i in 0..(DATA_BITS as u8) {
        let mut bitno = i + dv;
        if bitno & (bitno - 1) == 0 {
            bitno += 1;
            dv += 1;
        }
        check ^= if code & (1 << i) != 0 { (bitno << 1) | 1 } else { 0 };
    }

    return check;
}

pub fn slow_parity(code: u64) -> bool {
    let mut parity = true;

    for i in 0..63 {
        if code & 0b1 << i != 0 {
            parity = !parity;
        }
    }

    parity
}

pub fn parity(code: u64, i: u32) -> bool {
    let mut parity = true;
    let spread = 2u32.pow(i);
    let mut j = spread;

    while j < 64 - spread + 1 {
        for k in 0..spread {
            if (code & 0b1 << j + k) != 0b0 {
                parity = !parity;
            }
        }

        j += 2 * spread;
    }

    parity
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    #[test]
    fn test_fast_parity() {
        let inputs = vec![
            (1, 1u64.count_ones() % 2),
            (2, 2u64.count_ones() % 2),
            (67, 67u64.count_ones() % 2),
            (88, 88u64.count_ones() % 2),
            (1030, 1030u64.count_ones() % 2),
            (4397, 4397u64.count_ones() % 2),
            (9894, 9894u64.count_ones() % 2),
            (2u64.pow(63), 2u64.pow(63).count_ones() % 2),
            (2u64.pow(63) - 1, (2u64.pow(63) - 1).count_ones() % 2),
        ];

        for i in inputs.iter() {
            assert_eq!(fast_parity(i.0), i.1 as u64);
        }
    }

    #[test]
    fn test_valid_code() {
        let mut rng = rand::thread_rng();

        for _ in 1..4096 {
            let orig = rng.sample(Uniform::new(2u64.pow(1), 2u64.pow(32)));
            let raw: u64 = orig;
            let code = encode(raw);
            let block = decode(code);

            assert_eq!(orig, block);
        }
    }

    #[test]
    fn test_invalid_code() {
        let mut rng = rand::thread_rng();

        for _ in 1..4096 {
            let orig = rng.sample(Uniform::new(2u64.pow(1), 2u64.pow(32)));

            let raw: u64 = orig;
            let mut code = encode(raw);

            // Tamper with a 66.67% probability
            if rng.gen_bool(2.0 / 3.0) {
                let invalid_bit = rng.gen_range(0..=63);
                let mask: u64 = 0b1 << invalid_bit;

                // Toggle that specific bit
                code ^= mask;
            }

            let block = decode(code);

            assert_eq!(orig, block);
        }
    }
}
