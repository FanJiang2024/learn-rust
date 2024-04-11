pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (idx, item) in bytes.into_iter().enumerate() {
        if item == &b' ' {
            return &s[0..idx];
        }
    }
    return &s[..];
}

pub fn print_str(s: &str) {
    println!("{s}");
}

pub fn print_string(s: String) {
    println!("{s}");
}