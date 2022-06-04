#[allow(dead_code)]
fn main() {
    let tweet = Tweet {
        usernames: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already have"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet {}" ,tweet.summary());
}
pub trait Summary {
    fn summary(&self) -> String;
}

struct NesAritics {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,

}

struct Tweet {
    pub usernames: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}



impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}, by {}, ({}),{}", self.usernames
        ,self.content, self.reply,self.retweet)
    }
    
}

impl Summary for NesAritics{
    fn summary(&self) -> String {
        format!("{}, by {},is {} is {}",self.author,  self.content, self.headline, self.location)
    }
}