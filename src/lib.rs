const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const CHUNK_SIZE: u8 = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut encoded = String::new();
    let mut chunk_char_count = 0;
    for c in plain.chars() {
        match ALPHABET.iter().position(|&a| a == c.to_ascii_lowercase()) {
            Some(pos) => {
                append_char(
                    &mut encoded,
                    ALPHABET[25 - pos].to_ascii_lowercase(),
                    &mut chunk_char_count,
                );
            }
            None => {
                if c.is_ascii() && !c.is_ascii_whitespace() && !c.is_ascii_punctuation() {
                    append_char(&mut encoded, c, &mut chunk_char_count);
                }
            }
        }
    }
    encoded
}

fn append_char(encoded: &mut String, c: char, chunk_char_count: &mut u8) {
    if *chunk_char_count == CHUNK_SIZE {
        encoded.push(' ');
        *chunk_char_count = 0 as u8;
    }
    encoded.push(c);
    *chunk_char_count += 1;
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut decoded = String::new();
    for c in cipher.chars() {
        match ALPHABET.iter().position(|&a| a == c) {
            Some(pos) => decoded.push(ALPHABET[25 - pos]),
            None => {
                if !c.is_ascii_whitespace() {
                    decoded.push(c);
                }
            }
        }
    }
    decoded
}
