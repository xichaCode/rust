#![allow(dead_code, unused_imports)]
mod stdrrdemo;
use std::marker::PhantomData;
use std::process::Output;
use std::ops::Add;
use stdrrdemo::subscribe;


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

pub struct BufReader<R> {
    inner : R,
    buf: Box<u8>,
    pos: usize,
    cap: usize,
}

pub struct BuffReader<R> {
    inner : R,
    buf: Box<u8>,
    pos: usize,
    cap: usize,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identfier<T> {
    inner: u64,
    _tag: PhantomData<T>,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identfier<Self>
}
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product  {
    id: Identfier<Self>
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn id_should_be_the_same(){
        let user = User::default();
        let product = Product::default();
        // assert_eq!(user.id,product.id);
        assert_eq!(user.id.inner, product.id.inner);
    }
}

