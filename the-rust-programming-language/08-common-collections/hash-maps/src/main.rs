// HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function
// data is stored in the heap like vectors
// all keys must have the same type, all vectors must have the same type

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // create hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // access values in a hash map with .get()
    // .get() returns an Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // we can also iterate using a for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
    // this will print {"Blue": 25} since 10 has been overwritten

    // adding a key and value only if a key isn't present - use entry()
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // {"Yellow": 50, "Blue": 10}

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
