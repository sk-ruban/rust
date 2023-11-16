// a trait defines functionality a particular type has and can share with other types
// we cannot implement external traits on external types

pub trait Summary { // default implementation
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// traits as parameters
pub fn notify(item: &impl Summary) { // accepts any type that implements the specified trait
    println!("Breaking news! {}", item.summarize());
}

// another way using trait bounds for 2 parameters
// pub fn notify<T: Summary>(item1: &T, item2: &T) { }

// multiple trait bounds
// pub fn notify(item: &(impl Summary + Display)) { }
// pub fn notify<T: Summary + Display>(item: &T) { }

// where clause for better readability
fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    9
}

// return a type that implements traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementation for a particular struct
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// implementation for a particular struct
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}