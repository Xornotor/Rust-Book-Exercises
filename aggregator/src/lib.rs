// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) {
    //Syntax Sugar:
    //pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}
