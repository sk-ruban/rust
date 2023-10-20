// vectors allow you to store more than one value in a single data structure
// they can only store values of the same type

fn main() {
    // only i32 allowed
    let v1: Vec<i32> = Vec::new();

    // type is inferred
    let v2 = vec![1, 2, 3];

    // push to add elements
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reference a stored value
    let v4 = vec![1, 2, 3, 4, 5];

    // via index
    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    // via get method
    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // iterating over values
    let v5 = vec![100, 32, 57];
    // n_ref does not change v5
    for n_ref in &v5 {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // we can use enums to have vectors hold different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

