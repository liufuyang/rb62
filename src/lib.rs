#![no_std]
use lazy_static::lazy_static;
use specialized_div_rem::u128_div_rem_delegate; // for fast u128 dividing

lazy_static! {
    static ref MAX_VAL_ARRAY: [u8; 22] = {
        let mut max_val_array = [0u8; 22];
        let max_chars = "7N42dgm5tFLK9N8MT7fHC7".as_bytes(); // This will set all bits of a u128 as 1
        for i in 0..22 {
            max_val_array[i] = base62_val(&max_chars[i]).unwrap();
        }
        max_val_array
    };
}


pub fn get_integer(base62: &str) -> Option<u128> {
    let mut b62_val_array: [u8; 22] = [0u8; 22];
    let mut bi = 0u128;

    let base62 = base62.as_bytes();
    if base62.len() != 22 {
        return None;
    }


    for i in 0..22 {
        b62_val_array[i] = base62_val(&base62[i])?;
    }

    // check input value size is no bigger than max value - "7N42dgm5tFLK9N8MT7fHC7"
    for (val, max_val) in b62_val_array.iter().zip(MAX_VAL_ARRAY.iter()) {
        if val > max_val {
            return None;
        } else if val < max_val {
            break;
        } // and if they are equal, continue loop to compare next val
    }

    for v in b62_val_array.iter() {
        bi *= 62;
        bi += *v as u128;
    }
    Some(bi)
}

pub fn get_b62(hex: &str) -> Option<[u8; 22]> {
    let mut b62_val_array: [u8; 22] = [0u8; 22];

    for i in 0..22 {
        b62_val_array[i] = 48u8;
    }
    let mut hex_as_u128 = u128::from_str_radix(hex, 16).ok()?;
    let mut index = 22; // start with the last digit of 22 char b62
    while hex_as_u128 > 0 {
        // faster integer dividing, the code below is the same as:
        // let result = hex_as_u128 / 62;
        // let remainder = hex_as_u128 % 62;
        let (result, remainder) = u128_div_rem_delegate(hex_as_u128, 62);
        hex_as_u128 = result;
        b62_val_array[index - 1] = base62_char(remainder as u8)?;
        index -= 1;
    }
    Some(b62_val_array.clone())
}


// Returns 0-61
fn base62_val(value_char: &u8) -> Option<u8> {
    match value_char {
        b'0'..=b'9' => Some(*value_char as u8 - b'0'),
        b'a'..=b'z' => Some(*value_char as u8 - b'a' + 10),
        b'A'..=b'Z' => Some(*value_char as u8 - b'A' + 36),
        _ => None,
    }
}

// Return a char within "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
fn base62_char(value: u8) -> Option<u8> {
    match value {
        0..=9 => Some(b'0' + value),
        10..=35 => Some(b'a' + value - 10),
        36..=61 => Some(b'A' + value - 36),
        _ => None,
    }
}


#[cfg(test)]
mod tests {

}
