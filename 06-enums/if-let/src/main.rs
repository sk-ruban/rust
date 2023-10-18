fn main() {
    // if the function only has an effect in one condition, an if let is most idiomatic
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}