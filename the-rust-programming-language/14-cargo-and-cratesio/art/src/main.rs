// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::mix; // re-exported to top level using pub use in lib.rs
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
