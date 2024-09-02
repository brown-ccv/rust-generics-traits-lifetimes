use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("unfortunately, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet_summary = tweet.summarize();

    println!("1 new tweet: {}", tweet_summary);
}

