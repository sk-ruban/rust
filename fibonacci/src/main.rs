fn main() {
    let mut n = 10;
    let mut f0 = 0;
    let mut f1 = 1;
    let mut holder = 0;

    println!("{f0}");
    println!("{f1}");

    while n > 2 {
        holder = f1;
        f1 = f0 + f1;
        f0 = holder;
        n -= 1;
        println!("{f1}")
    }

}