use std::fs;

mod fibloop;
mod scap;
mod question;

fn main() {
    let url = "https://www.rust-lang.org";
    let _output ="rust.md";

    println!("Fetching url : {}" ,url);

    let _body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");

    let md = html2md::parse_html(&_body);
    fs::write(_output,md.as_bytes()).unwrap();

    println!("Converted markdown has been saved in {}." ,_output);

    
}

