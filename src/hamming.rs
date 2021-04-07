pub fn encode(block: &mut u64) -> u64 {
    let len_power = 6;
    let len = 64;

    let mut code = 0u64;

    for i in 1..len {
        // Check if `i` is not a power of 2
        if (i & (i - 1)) != 0 {
            code |= (0b1 << i - 1) & *block as u64;
        } else {
            *block <<= 1;
        }
    }

    for i in 0..len_power {
        // If the parity check is odd, set the bit to 1 otherwise move on.
        if !parity(&code, i) {
            code |= 0b1 << (2usize.pow(i) - 1);
        }
    }

    code
}

pub fn decode(code: &mut u64) -> u64 {
    let len_power = (2..).find(|&r| 2u32.pow(r) - r - 1 >= 32).unwrap();
    let len = 2usize.pow(len_power);

    let mut check = 0b0;
    for i in 0..len_power {
        if !parity(&code, i) {
            check |= 0b1 << i;
        }
    }

    // We have an error
    if check > 0b0 {
        *code ^= check;
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

pub fn parity(code: &u64, i: u32) -> bool {
    let mut parity = true;
    let mut j = (0b1 << i) - 1;

    while j < 64 {
        if (code & 0b1 << j) != 0b0 {
            parity = !parity;
        }

        j += j + 2 * (i + 1);
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

            println!("orig: {:064b}", orig);
            let mut raw: u64 = orig;

            // Tamper with a 66.67% probability
            if rng.gen_bool(2.0 / 3.0) {
                let invalid_bit = rng.gen_range(0..63);
                let mask: u64 = 0b1 << invalid_bit;

                // Toggle that specific bit
                raw ^= mask;
            }

            let mut code = encode(&mut raw);
            let block = decode(&mut code);
            println!("deco: {:064b}", block);

            assert_eq!(orig, block);
        }
    }
}
