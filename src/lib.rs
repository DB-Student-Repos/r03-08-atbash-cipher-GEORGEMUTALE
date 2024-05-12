fn atbash_cipher(text: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let reversed_alphabet: String = alphabet.chars().rev().collect();
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let index = alphabet.find(c.to_ascii_lowercase()).unwrap();
            let cipher_char = reversed_alphabet.chars().nth(index).unwrap();
            result.push(cipher_char);
        } else if c.is_ascii_digit() {
            // For digits, simply append without transformation
            result.push(c);
        }
    }

    result
}

pub fn encode(plain: &str) -> String {
    let mut encoded = String::new();
    let mut count = 0;

    for c in atbash_cipher(plain).chars() {
        encoded.push(c);
        count += 1;

        // Add space every 5 characters, excluding the last one
        if count % 5 == 0 && count < atbash_cipher(plain).len() {
            encoded.push(' ');
        }
    }

    encoded
}

pub fn decode(cipher: &str) -> String {
    atbash_cipher(cipher)
}
