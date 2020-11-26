use json::{self, object::Object, object};
use json::JsonValue;
use chrono::prelude::*;
#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String
}



struct CacherExample<T>
where
    T: Fn(u32) -> u32,
{
    callback: T,
    value: Option<u32>
}

impl <T> CacherExample<T> 
where
    T: Fn(u32) -> u32
{
    fn new(cacher_example:T) -> CacherExample<T> {
        CacherExample {
            callback: cacher_example,
            value:None,
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.callback)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn gen_topic(topic: &str) -> String {
    let mut v: Vec<&str> = topic.split('/').collect();
    let mut result = String::new();    
    
    if v.len() < 4 {
        return result;
    }

    {
        let appname = v[0].to_string().clone();
        let database = v[3].to_string().clone();
        let request = String::from("response");

        v[0] = &database;
        v[2] = &request;
        v[3] = &appname;
        for i in &v {
            result.push_str(i);
            result.push('/');
        }
        result.pop();
    }

    result
}

pub fn get_appname(topic: &str) -> String {
    let v: Vec<&str> = topic.split('/').collect();
    let result = String::new();    
    
    if v.len() < 4 {
        return result;
    }
    let appname = v[0].to_string().clone();

    appname
}

fn gen_token(token: &str) -> String {
    token.to_string()
}

fn gen_timestamp() -> String {
    let dt = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    dt
}

pub fn package_message(token: &str, mut message: Object) -> String {
    let ts = gen_timestamp();
    let token = gen_token(token);

    message.insert("token", json::JsonValue::String(token));
    message.insert("timestamp", json::JsonValue::String(ts));

    message.dump()
}

pub fn get_token(message: &JsonValue) -> String {
    let mut token: String = String::from("123");

    match message["token"].is_null() {
        false => {
            token = message["token"].as_str().unwrap().to_string();
        },
        _ => {}
    }

    token
}

pub fn get_key_value_str(key: &str, message: &JsonValue) -> Option<String> {
    if message[key].is_null() {
        return None;
    }

    match message[key].is_string() {
        true => {
            match message[key].as_str() {
                None => { None},
                Some(val) => { Some(val.to_string())}
            }
        },
        false => {
            None
        }
    }
}

pub fn get_array_array(key: &str, message: &JsonValue) -> JsonValue {
    if message[key].is_null() {
        return JsonValue::new_array();
    }

    match message[key].is_array() {
        true => {
            message[key].clone()
        },
        false => {
            JsonValue::new_array()
        }
    }
}