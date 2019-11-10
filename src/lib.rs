const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const SEPARATOR: &str = " ";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut count = 0;
    plain.chars().fold(String::new(), |mut encoded, c| {
        match ALPHABET.iter().position(|&a| a == c.to_ascii_lowercase()) {
            Some(pos) => {
                encoded.push_str(&format!("{}{}", separator(&mut count), ALPHABET[25 - pos]));
                encoded
            }
            None => match c {
                c if !c.is_ascii() || c.is_ascii_punctuation() || c.is_ascii_whitespace() => {
                    encoded
                }
                _ => {
                    encoded.push_str(&format!("{}{}", separator(&mut count), c));
                    encoded
                }
            },
        }
    })
}

fn separator(count: &mut usize) -> &'static str {
    *count += 1;
    if *count == 6 {
        *count = 1;
        SEPARATOR
    } else {
        ""
    }
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match ALPHABET.iter().position(|&a| a == c) {
            Some(pos) => ALPHABET[25 - pos],
            None => c,
        })
        .collect::<String>()
}
