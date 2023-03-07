fn main() {
    let s = String::from("hello world!");
    let hello = first_word(&s);
    let world = last_word(&s);

    println!("{hello} {world}");
}

fn first_word(s: &str) -> &str {
    let bs = s.as_bytes();

    for (i, &b) in bs.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn last_word(s: &str) -> &str {
    let bs = s.as_bytes();
    for (i, &b) in bs.iter().enumerate().rev() {
        if b == b' ' {
            return &s[(i + 1)..];
        }
    }

    return &s[..];
}
