use crate::mqtt_lib::mqtt_init::publish_message;
use crate::common::common::{
    gen_topic,
    package_message
};
use json::{
    object::Object,
    JsonValue
};

pub fn response_version(topic: &str, payload: &str) {
    let parse_result = json::parse(payload);
    let mut request_message = JsonValue::new_object();

    match parse_result {
        Ok(val) => {
            request_message = val; 
        },
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    {
        let response_topic = gen_topic(topic);
        let version: String = String::from("2.1");
        let compatible: String = String::from("1.0");
        let mut result = Object::new();

        result.insert("version", json::JsonValue::String(version));
        result.insert("Compatible", json::JsonValue::String(compatible));
        let token = request_message["token"].as_str().unwrap_or_else(|| {
            return "";
        });

        if token.len() == 0 {
            return;
        }
        
        let send_message = package_message(token, result);
        println!("publish!");
        publish_message(&response_topic, &send_message);
    }
}

