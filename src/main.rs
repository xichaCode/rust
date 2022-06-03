use std::collections::BTreeMap;
fn main() {
    let mut map = BTreeMap::new();
    map.insert("hello", "world");
    print!("map: {:?}", map);

    iter_fnu();
}

fn iter_fnu(){
    let numbers = vec![1,2,3,4,5,6,7,8,9];

    let even_number: Vec<_> = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect();
    println!("{:?}", even_number);
}