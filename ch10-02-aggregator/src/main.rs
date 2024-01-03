use ch10_02_aggregator::{NewsArticle, Summary, Tweet, notify};

fn main() {
    let today_news = NewsArticle {
        headline: String::from("HHHHHHHH"),
        location: String::from("Guangzhou"),
        author: String::from("Shawn"),
        content: String::from("CCCCCCCCCCCCCC"),
    };
    println!("Tody New: 《{}》", today_news.summarize());
    notify(&today_news);

    let tweet = Tweet {
        username: String::from("Shawn"),
        content: String::from("CCCCCCCCCCCCCC"),
        reply: false,
        retweet: true
    };

    println!("Hot tweet: 《{}》", tweet.summarize());
    notify(&tweet);
}
