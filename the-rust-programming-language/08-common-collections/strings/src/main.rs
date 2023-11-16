fn main() {
    // creates a new empty string where we can load data into
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // String::from and to_string() do the same thing
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // appending to strings
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str() takes a string slice
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push() adds a single character
    let mut s = String::from("lo");
    s.push('l');

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // complicated string combination
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format!() returns a String with the contents

    // access characters by indexing is INVALID
    let s1 = String::from("hello");
    let h = s1[0]; // INVALID

    // use .chars() to access each element
    for c in "Зд".chars() {
        println!("{c}");
    }

    // returns each raw byte
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
