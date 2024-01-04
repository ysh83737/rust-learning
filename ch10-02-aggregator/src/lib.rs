use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, {} by {}", self.headline, self.location, self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1(item1: &impl Summary, item2: &impl Summary) {
    println!("1. Breaking news! {}", item1.summarize());
    println!("2. Breaking news! {}", item2.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("1. Breaking news! {}", item1.summarize());
    println!("2. Breaking news! {}", item2.summarize());
}

pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
    println!("Display {}", item);
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display {}", item);
}

pub fn func1<T: Clone + Display, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1111
}

pub fn func2<T, U>(t: &T, u: &U) -> i32
where
    T: Clone + Display,
    U: Clone + Debug,
{
    1111
}
