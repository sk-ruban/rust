use std::collections::HashMap;

fn main() {
    let mut nums = vec![7, 3, 8, 4, 6, 2, 5, 1, 9, 10, 12, 13, 7, 14, 20, 4, 16, 7, 15, 6];

    println!("MODE: {}", mode(&nums));
    println!("MEDIAN: {}", median(&mut nums));
}

// return the most common value
fn mode(i: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for each in i {
        *map.entry(each).or_insert(0) += 1;
    }

    *map
        .into_iter() // converts the HashMap into an iterator of (key, value) pairs
        .max_by_key(|&(_, count)| count) // finds the entry with the maximum count value. It compares the count part of each (key, count) pair.
        .map(|(val, _)| val) // extracts just the key part from the (key, count) pair
        .expect("Cannot compute the mode of zero numbers") // if map is empty
}

// return the middle value
fn median(i: &mut Vec<i32>) -> i32 {
    i.sort();
    let middle = i.len() / 2;
    i[middle]
}