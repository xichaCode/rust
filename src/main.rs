mod fibloop;
// mod scap;
struct MyStruct {
    s: u32
}
fn main() {
    let mut x = MyStruct{
        s: 5u32
    };
    let _y = &x;
    x.s = 6;
    println!("{}" ,x.s);
    // types_not_impl_copy_trait();
    // types_not_impl_copy_trait();

}

fn is_copy<T: Copy>(){

}

fn type_impl_copy_trait(){
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait(){
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();

    is_copy::<&mut String>();
    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}
