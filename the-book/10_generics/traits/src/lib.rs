use std::fmt::Display;
use std::fmt::Debug;

pub trait Summarizable {

    // default behavior, if not implemented directly
    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }

    fn author_summary(&self) -> String;

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

// We're using a generic type here (T), but trait bounds make it impossible to call this function
// unless the type implements the Summarizable trait.
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

// multiple trait bounds
pub fn some_function<V: Display + Clone, U: Clone + Debug>(v: V, u: U) -> i32 {}

// we could also write this with a where clause, which is neater
pub fn another_function<V, U>(v: V, u: U) -> i32
    where V: Display + Clone,
          U: Clone + Debug
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
