// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
// Slices contain a pointer and length (additional), therefore it takes more me than a String

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("the first word is: {}", word);
}

// Slices are of type str if generated from String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice() {
    let s = String::from("hello world");

    // s loses write and own permissions
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s;

    // these 2 values are equal
    let slice = &s[0..2];
    let slice = &s[..2];

    // slice of entire string
    let slice = &s[0..len];
    let slice = &s[..];
}