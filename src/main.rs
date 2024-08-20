
mod test {

    #[test]
    fn validate() {
        use crate::from_hex_to_base64;

        let strings: [(&str, &str); 4] = [
            ("000", "AA"),
            ("0",   "A="),
            ("FC",  "/A=="),
            ("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        ];
        
        for s in &strings {
            assert_eq!(from_hex_to_base64(s.0.to_string()), s.1.to_string());
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() < 1 {
        usages();
        return;
    }

    for hex_string in args {
        if is_hex(&hex_string) {
            println!("{:?}", from_hex_to_base64(hex_string));
        } else {
            println!("{:?}", hex_string);
        }
    }
}

struct SixBits(u8);

enum StartBit {
    Zero,
    Two,
}

impl SixBits {
    fn get(&self) -> usize {
        self.0 as usize
    }

    fn from_hex(hex_raw: u8, start_bit: StartBit) -> SixBits {
        SixBits(
            match start_bit {
                // [h h h h][l l _ _]
                StartBit::Zero => (hex_raw & 0b11111100) >> 2,
                // [_ _ h h][l l l l]
                StartBit::Two  => (hex_raw & 0b00111111) >> 0,
        })
    }
}

fn from_hex_to_base64(hex: String) -> String {
    let mut base64 = String::new();
    let bits_to_represent = hex.len() * 4;
    let b64_words = (bits_to_represent + 5) / 6;
    let chars: Vec<char> = hex.chars().collect();

    for w in 0..b64_words {
        // extract bit stream
        let bits = get_bits_from_hex(&chars, w);
        base64.push(bits_to_base64(bits));
    }

    let padding_bits = b64_words * 6 - bits_to_represent;
    // must be true, sanity check for validity of code above
    assert!(padding_bits % 2 == 0);
    
    let padding = padding_bits / 2;
    for _ in 0..padding {
        base64.push('=');
    }


    base64
}

fn get_bits_from_hex(hex: &Vec<char>, index: usize) -> SixBits {
    let bit_offset = index * 6;
    let hex_token_start = bit_offset / 4;
    let hex_token_count = if hex.len() - hex_token_start > 1 { 2 } else { 1 };
    let hex_value = if hex_token_count == 1 {
        hex_to_raw(hex[hex_token_start]) << 4
    } else {
        (hex_to_raw(hex[hex_token_start]) << 4) | hex_to_raw(hex[hex_token_start+1])
    };
    let start_bit = if bit_offset % 4 == 0 { StartBit::Zero } else { StartBit::Two };

    SixBits::from_hex(hex_value, start_bit)
}

fn bits_to_base64(bits: SixBits) -> char {
    let mut map: Vec<char> = Vec::new();
    
    for c in 'A'..='Z' {
        map.push(c);
    }

    for c in 'a'..='z' {
        map.push(c);
    }

    for c in '0'..='9' {
        map.push(c);
    }

    map.push('+');
    map.push('/');


    map[bits.get()]
}

fn hex_to_raw(h: char) -> u8 {
    match h {
        '0'..='9' => h as u8 - '0' as u8,
        'a'..='f' => h as u8 - 'a' as u8 + 10,
        'A'..='F' => h as u8 - 'A' as u8 + 10,
        _ => panic!("Invalid Hex Token!")
    }
    
}

fn is_hex(to_check: &String) -> bool {
    let lower = 'a'..='f';
    let upper = 'A'..='F';
    let numbers = '0'..='9';

    for c in to_check.chars() {
        if !lower.contains(&c) && !upper.contains(&c) && !numbers.contains(&c) {
            return false;
        }
    }

    true
}

fn usages() {
    println!("USAGE:");
    println!("hex2base64 [hex string 1] [...hex string N]");
}