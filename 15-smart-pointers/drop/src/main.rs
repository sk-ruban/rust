// drop customises what happens when a value is about to go out of scope

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop(); cannot call drop explicitly
    drop(c);
    println!("CustomSmartPointers created.");
}
// drop happens here
//variables are dropped in reverse order - d then c