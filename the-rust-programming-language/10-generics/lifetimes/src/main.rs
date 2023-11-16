//&i32        // a reference
//&'a i32     // a reference with an explicit lifetime
//&'a mut i32 // a mutable reference with an explicit lifetime

fn compare_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // we want longest to take string slice, rather than strings, because we don't want ownership
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// the lifetime of the return value is the same of smaller of the function arguments
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 3rd lifetime elision rule - if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters
struct ImportantExcerpt<'a> {
    // lifetime declaration when structs hold references
    part: &'a str,
}

// 1st lifetime elision rule - compiler assigns a different lifetime parameter to each lifetime in each input type
// 2nd lifetime elision rule - if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 3rd lifetime elision rule -
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 'static means the data is never deallocated