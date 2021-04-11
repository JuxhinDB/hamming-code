pub fn encode(block: &mut u64) -> u64 {
    // TODO(jdb): assert length on block to be less the parity bits
    let len_power = 6;
    let len = 64;

    let mut code = 0u64;

    for i in 0..len {
        // Check if `i` is not a power of 2
        if (i != 0) && (i & (i - 1)) != 0 {
            code |= (0b1 << i) & *block as u64;
        } else {
            *block <<= 1;
        }
    }

    for i in 0..len_power {
        // If the parity check is odd, set the bit to 1 otherwise move on.
        if !parity(&code, i) {
            code |= 0b1 << (2usize.pow(i));
        } 
    }

    // Set the global parity
    code |= fast_parity(code);

    code
}

pub fn decode(code: &mut u64) -> u64 {
    let len_power = 6;
    let len = 64;

    let mut check = 0b0;

    for i in 0..len_power {
        let bit_position = 2u32.pow(i);

        // If odd and the parity bit is not 1, then we have an error
        let is_even = parity(&code, i);
        let _code = *code & (0b1 << bit_position);

        if is_even && _code != 0 || !is_even && _code == 0 {
            check |= 0b1 << bit_position;
        }    
    }

    // We have an error
    if check > 0b0 {
        println!("error at bit: {}", check);
        *code ^= 0b1 << check;
    }

    // Drop all parity bits
    let mut offset = 0;
    let mut decoded = 0b0;

    for i in 1..len {
        if (i & (i - 1)) != 0 {
            decoded |= ((0b1 << i - 1) & *code) >> offset;
        } else {
            offset += 1;
        }
    }

    decoded
}

/// Hacker's delight 2nd edition, p.96 
/// Henry S. Warren, Jr.
pub const fn fast_parity(code: u64) -> u64 {
    let mut y: u64 = code ^ (code >> 1);
    
    y ^= y << 2;
    y ^= y << 4;
    y ^= y << 8;
    y ^= y << 16;
    y ^= y << 32;

    0b1 & y
}

pub fn parity(code: &u64, i: u32) -> bool {
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
    fn test_dynamic_valid_code() {
        let mut rng = rand::thread_rng();

        for _ in 1..4096 {
            let orig = rng.sample(Uniform::new(2u64.pow(1), 2u64.pow(32)));
            let mut raw: u64 = orig;
            let mut code = encode(&mut raw);
            let block = decode(&mut code);

            assert_eq!(orig, block);
        }
    }

    #[test]
    fn test_dynamic_single_invalid_code() {
        let mut rng = rand::thread_rng();

        for _ in 1..4096 {
            let orig = rng.sample(Uniform::new(2u64.pow(1), 2u64.pow(32)));

            let mut raw: u64 = orig;
            let mut code = encode(&mut raw);

            // Tamper with a 66.67% probability
            if rng.gen_bool(2.0 / 3.0) {
                let invalid_bit = rng.gen_range(0..=63);
                let mask: u64 = 0b1 << invalid_bit;

                // Toggle that specific bit
                code ^= mask;
            }

            let block = decode(&mut code);

            assert_eq!(orig, block);
        }
    }
}
