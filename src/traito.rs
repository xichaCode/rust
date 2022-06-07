trait Run {}

struct Human{

}

impl Run for Human{

}

struct Cat {

}

impl Run for Cat{}

fn demo<T>(x: Vec<Box<T>>) where T: Run{ 

}

fn main() {
    let mut v = vec![];
    v.push(Box::new(human{}));
    v.push(Box::new(Cat{}));
    demo(v);
}