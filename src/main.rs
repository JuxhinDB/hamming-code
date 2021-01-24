fn main() {
    let mut raw: u64 = 0b1100101101010011010101111010001;
    println!("    Raw: {:064b}", raw);

    let mut code = encode(&mut raw);
    //let block = decode(&mut code);

    println!("Encoded: {:064b}", code);
    //println!("Decoded: {:064b}", block);
}

#[inline(always)]
fn encode(block: &mut u64) -> u64 {
    let len_power = (2..).find(|&r| 2u32.pow(r) - r - 1 >= 32).unwrap();
    let len = 2usize.pow(len_power) - 1;

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
        if parity(&code, i) {
            code |= 1u64 << (2usize.pow(i) - 1);
        } else {
            let mask = 1u64 << (2usize.pow(i) - 1);
            code &= !mask;
        }
    }

    code
}

//#[inline(always)]
//fn decode(code: &mut u64) -> u64 {
//    let len_power = ((64 + 1) as f64).sqrt().round() as u64;
//    let mut flipped_bit = -1i32;
//
//    while (0..len_power).any(|i| parity(&code, i as u32) == 1u64) {
//        if flipped_bit != -1 {
//            *code ^= 1u64 << flipped_bit;
//        }
//
//        flipped_bit += 1;
//        *code ^= 1u64 << flipped_bit;
//    }
//
//    *code
//}

// TODO(jdb): Optimize this, remove internal bools and flags
fn parity(code: &u64, i: u32) -> bool {
    let bi = 2u32.pow(i) - 1;
    let (mut parity, mut ignore, mut counter) = (false, false, 0);
    for j in bi..64 {
        let mask = 0b1 << j;
        if !ignore && (code & mask) == mask {
            parity = !parity;
        }
        counter += 1;
        if counter >= 2u32.pow(i) {
            ignore = !ignore;
            counter = 0;
        }
    }
    parity
}
