mod content;

// To call the trait methods on instances, the trait and the types must be brought into scope
use aggregator::Summary;
use content::{NewsArticle, Tweet};

fn main() {
  let tweet = Tweet {
    username: "tomislavcapan".to_string(),
    content: "Hello, world!".to_string(),
    retweet: false,
    reply: false,
  };

  let news_article = NewsArticle {
    headline: "Hello, world from our newspaper!".to_string(),
    author: "Tomislav Capan".to_string(),
    content: "This is my world. I want to enjoy in this world.".to_string(),
    location: "Zagreb, Croatia".to_string(),
  };

  println!("1 new Tweet:\n{}\n", tweet.summarize());
  println!("New article available:\n{}\n", news_article.summarize());

  content::notify_impl(&tweet);
  content::notify_impl(&news_article);

  content::notify_trait_bound(&tweet);
  content::notify_trait_bound(&news_article);
  content::notify_trait_bound(&content::returns_summarizable());
}
