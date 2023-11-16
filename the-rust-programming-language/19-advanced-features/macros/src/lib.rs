// declarative macros
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        // When we call this macro with vec![1, 2, 3];,
        // the $x pattern matches three times with the three expressions 1, 2, and 3
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                // temp_vec.push(1);
                // temp_vec.push(2);
                // temp_vec.push(3);
            )*
            temp_vec
        }
    };
}

// procedural macros
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}