use std::fs::File;
use std::io::prelude::*;

extern crate rjson;
// use rjson::*;

fn main() {
    println!("Hello, world!");
    let path: &str = "sample.json";
    println!("file {}", path);
    let mut input: File = File::open(path).expect("No file opened!");
    let mut content: String = String::new();
    input.read_to_string(&mut content);

    // let n :rjson::Sxx;
    

    let obj = rjson::parse(&content);
    println!("{:?}",obj);
    println!("{:?}",obj["FirstName"]);
    println!("{:?}",obj["Age"]);
    println!("{:?}", obj.get("PhoneNumbers"));
    println!();
}