fn main() {
    let tweet = Tweet {
        content: String::from("hellooooo"),
        username: String::from("victor"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
    notify(&tweet);

    notify_with_title(&tweet)
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking news {}", item.summarize())
}

fn notify_with_title<T: Summary + Title>(item: &T) {
    println!("Title headline: {}", item.get_title())
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Title {
    fn get_title(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Title for NewsArticle {
    fn get_title(&self) -> String {
        format!("{}", self.headline)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Title for Tweet {
    fn get_title(&self) -> String {
        format!("{}", self.username)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
