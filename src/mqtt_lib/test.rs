use crate::mqtt_lib::mqtt_init::{
    init, publish_message,
};
use crate::protocol::register::{
    register_func
};
use std::{
    thread,
    time::Duration,
    process
};

static TEST_STR:&str = "FUCKYOU";

fn test_func(_topic: &str, payload: &str) {
    if payload == TEST_STR {
        println!("mqtt test success");
    } else {
        println!("mqtt test failed");
    }
    process::exit(0);
}

pub fn test() {
    register_func("+/get/request/database/version", &test_func);
    init();
    thread::sleep(Duration::from_millis(1000));      
    thread::sleep(Duration::from_millis(1000));     
    publish_message("test/get/request/database/version", &TEST_STR);
    thread::sleep(Duration::from_millis(1000));
    println!("did not recv message!");
}