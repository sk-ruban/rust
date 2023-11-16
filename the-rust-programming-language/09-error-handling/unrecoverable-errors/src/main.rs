// aborting ends the program without cleaning up
// you can add panic = 'abort' to cargo.toml if you want to abort
// backtrace is a list of functions called to get to this point

fn panic() {
    panic!("crash and burn");
}

fn main() {
    let v = vec![1, 2, 3];

    v[99];
}