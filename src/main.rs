#![allow(dead_code, unused_imports)]
use std::process::Output;
use std::ops::Add;


fn main() {
    let tweet = Tweet {
        usernames: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already have"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet {}" ,tweet.summary());

    assert_eq!(sum(1u32, 2u32), 3);
    assert_eq!(sum(1u64,2u64), 3);

    let my_name = "Pascal".to_string();
    geeet(my_name);
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

fn sum<T: Add<T ,Output=T>>(a: T, b: T) -> T {
    a + b
}

fn geeet(name :String)  {
    println!("hello {} ,name: {}",name,name)
}