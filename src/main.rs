mod fibloop;
mod scap;

fn main() {
    let url = "https://wwww.rust-lang.org/";
    let _output = "rust.md";
    println!("Fetching url: {}" ,url);
    // let bodys = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown....");
    // let md = html2md::parse_html(&bodys);
    // fs::write(output, md.as_bytes()).unwrap();
    // println!("Converted markdown has been saved in {}" , output);

    apply(2, square);
    apply(2,cube);

    let alice = User {id : UserId(1), name: "Alice".into(),gender: Gender::Female};
    let bob = User {id : UserId(2), name: "Bob".into(), gender: Gender::Male};
    let topic = TopIc {id :TopicId(1), name: "rust".into(), onwner : UserId(1)};
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id,topic.id));
    let event3 = Event::Message((alice.id,topic.id, "Hello World !".into()));
    println!("event1: {:?}, event2: {:?}, event3: {:?}",event1, event2,event3);

    let mut a = 2;
    let mut b = 1;

    let other = a;
    a = b;

    println!("a 中的 值 为 ：{}" ,a); //会编译错误，原因是a变量指向的地址引用的所有权，已经转移给了b,a的内存地址被 释放


    let cc = 1;
    let bsb = cc;
    let _xybs = bsb;
    println!("打印bb的值，；{ },{ }" ,cc,bsb);

    let s1 = String::from("Hellow");
    let s2 = s1;

    let ss1 = 3;
    let ss2 = ss1;
    println!("{}" , ss1);
    // println!("{}",s1);

    let data = vec![1,2,3,4];
    let data1 = data;
    println!("sum of data1: {}" ,sum(data1));
    // println!("data1 : {:?}",data1);
    // println!("sum of data : {}",sum(data));

}
fn sum(data: Vec<u32>) -> u32{
    data.iter().fold(0, |acc, x | acc + x)
}

fn apply(value: i32, f: fn(i32) ->i32) -> i32{
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug,Copy,Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}
#[derive(Debug)]
struct TopIc {
    id : TopicId,
    name : String,
    onwner : UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId,TopicId)),
    Leave((UserId,TopicId)),
    Message((UserId,TopicId,String)),
}

#[cfg(test)]
mod state {
    #[test]
    fn it_works(){
        assert_eq!(2 + 2,4)
    }
}

