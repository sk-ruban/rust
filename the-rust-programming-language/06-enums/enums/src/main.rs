// enums give you a way of saying a value is one of a possible set of values

fn concise_enum() {
    // V4() is a function call that takes in a String and returns an instance of IpAddr
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

fn main() {

    // Option<T> and T are different types so you cannot add them together
    // code must handle Some(T) and None
    enum Option<T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    // we need to declare the type for None as the compiler cannot infer
    let absent_number: Option<i32> = None;

    enum IpAddrKind {
        V4,
        V6,
    }

    // struct with type IpAddrKind
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // both values are of the same type: IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

