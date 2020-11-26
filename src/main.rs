use json;
use database::protocol::test::test;
use database::common::common::{
    package_message
};

fn main() {
    let data = json::object::Object::new();
    
    println!("{}", package_message("123", data));
    test();
}