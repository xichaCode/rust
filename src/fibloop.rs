use crate::Event;

fn fib_loop(n: u8) {
    let mut a  = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}" ,b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1,2,3);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}" ,b);
    }
}

fn fib_for(n: u8) {
    let (mut a ,mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}" ,b);
    }
}

fn process_message(event: &Event) {
    if let Event:: Message((_,_,msg)) = event {
        println!("broadcase: {}" ,msg);
    }
}

fn abs_fun(mut a: i32,mut b: i32,mut i: u8) -> i32{
    let c = a + b;
    a = b;
    b = c;
    i += 1;
    b
}
