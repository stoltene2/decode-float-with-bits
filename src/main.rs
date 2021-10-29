const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

/**
Extract the bit representation of the sign, exponent, and mantissa
*/
fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits: u32 = n.to_bits();

    let sign = bits >> 31 & 0xff;
    let exponent = bits >> 23 & 0xff;
    let mantissa = bits & 0x7fffff;

    (sign, exponent, mantissa)
}

/**
Take a floating point number and extract the sign, exponent, and mantissa for float representation
*/
fn decode(n: f32) -> (f32, f32, f32) {
    let (sign_bits, exponent_bits, mantissa_bits) = to_parts(n);

    let sign = (-1.0_f32).powf(sign_bits as f32);
    let exponent = (exponent_bits as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = mantissa_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (sign, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn main() {
    // Convert a 4-byte float to a 4-byte unsignted int
    let (sign, exponent, mantissa) = decode(42.42);
    let n = from_parts(sign, exponent, mantissa);
    println!("{}", n);
}
