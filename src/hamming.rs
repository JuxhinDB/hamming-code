pub fn encode(block: &mut u64) -> u64 {
    let len_power = (2..).find(|&r| 2u32.pow(r) - r - 1 >= 32).unwrap();
    let len = 2usize.pow(len_power);

    let mut code = 0u64;

    for i in 1..len {
        // Check if `i` is not a power of 2
        if (i & (i - 1)) != 0 {
            code |= (0b1 << i - 1) & *block as u64;
        } else {
            *block <<= 1;
        }
    }

    let mut encoded = code;
    for i in 0..len_power {
        // If the parity check is odd, set the bit to 1 otherwise move on.
        if !parity(&code, i) {
            encoded |= 0b1 << (2usize.pow(i) - 1);
        }
    }

    encoded
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

fn parity(code: &u64, i: u32) -> bool {
    let bi = (0b1 << i) - 1;
    let (mut parity, mut ignore, mut counter) = (true, false, 0);
    for j in bi..64 {
        if !ignore && (code & 0b1 << j) != 0b0 {
            parity = !parity;
        }

        counter += 1;
        if counter >= 0b1 << i {
            ignore = !ignore;
            counter = 0;
        }
    }

    // true if even
    // false if odd
    parity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_code() {
        let orig: u64 = 0b1001;
        let mut raw: u64 = orig;
        let mut code = encode(&mut raw);
        let block = decode(&mut code);

        assert_eq!(orig, block);
        pub fn encode(block: &mut u64) -> u64 {
            let len_power = (2..).find(|&r| 2u32.pow(r) - r - 1 >= 32).unwrap();
            let len = 2usize.pow(len_power);

            let mut code = 0u64;

            for i in 1..len {
                // Check if `i` is not a power of 2
                if (i & (i - 1)) != 0 {
                    code |= (0b1 << i - 1) & *block as u64;
                } else {
                    *block <<= 1;
                }
            }

            let mut encoded = code;
            for i in 0..len_power {
                // If the parity check is odd, set the bit to 1 otherwise move on.
                if !parity(&code, i) {
                    encoded |= 0b1 << (2usize.pow(i) - 1);
                }
            }

            encoded
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

        fn parity(code: &u64, i: u32) -> bool {
            let bi = (0b1 << i) - 1;
            let (mut parity, mut ignore, mut counter) = (true, false, 0);
            for j in bi..64 {
                if !ignore && (code & 0b1 << j) != 0b0 {
                    parity = !parity;
                }

                counter += 1;
                if counter >= 0b1 << i {
                    ignore = !ignore;
                    counter = 0;
                }
            }

            // true if even
            // false if odd
            parity
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_basic_code() {
                let orig: u64 = 0b1001;
                let mut raw: u64 = orig;
                let mut code = encode(&mut raw);
                let block = decode(&mut code);

                assert_eq!(orig, block);
            }
        }
    }
}
