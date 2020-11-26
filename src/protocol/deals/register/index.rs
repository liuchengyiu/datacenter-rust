use crate::common::common::{
    get_appname,
    KeyValue,
    package_message,
    gen_topic,
    get_token,
    get_key_value_str,
    get_array_array
};
use json::{
    object::Object,
    JsonValue,
};
use crate::mqtt_lib::mqtt_init::publish_message;
use std::{
    process,
    hash::Hasher,
    collections::hash_map::DefaultHasher,
    collections::HashMap
};
use crate::sql::operate::DATABASE;

static DEVICE_TABLE: &str = "device";

pub fn response_device_register(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let app_name = get_appname(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();
    let devices = get_array_array("body", &request_message);
    let mut kvs: Vec<Vec<KeyValue>> = Vec::new();
    let get_values = Vec::from(["model".to_string(), "port".to_string(), "addr".to_string(),
                                    "desc".to_string(), "manuID".to_string(), "isreport".to_string()]);

    response_data.insert("status", JsonValue::String("FAILURE".to_string()));
    for dev in devices.members() {
        let deal = || {
            return "".to_string();
        };
        let mut hasher = DefaultHasher::new();
        let mut kv: Vec<KeyValue> = Vec::new();
        for param in &get_values {
            let p = get_key_value_str(param, dev).unwrap_or_else(deal);

            if p.len() == 0 {
                publish_message(&response_topic, &package_message(&token, response_data));
                return;
            }
            if param == "addr" || param == "model" || param == "port" {
                hasher.write(p.as_bytes());
            }
            kv.push(KeyValue {
                key: param.to_string(),
                value: p.to_string()
            });
        }
        let guid = hasher.finish().to_string();
        kv.push(KeyValue {
            key: "appname".to_string(),
            value: app_name.to_string()
        });
        kv.push(KeyValue {
            key: "guid".to_string(),
            value: guid
        });

        kvs.push(kv);
    }
    DATABASE.with(move |lock| {
        let tmp = lock.lock().unwrap_or_else(move|e|{
            println!("{}", e);
            process::exit(0);
        });
        let database = &mut *tmp.borrow_mut();
        let db_deal = |e: sqlite::Error| {
            println!("db model set error {}", e);
            return false;
        };

        for kv in kvs {
            let id = Vec::from([kv[0].clone(), kv[1].clone(), kv[2].clone()]);

            match database.exists_data(DEVICE_TABLE, &id) {
                Ok(val) => {
                    let mut flag = false;
                    if val {
                        flag = database.update_data(DEVICE_TABLE, &id, &kv)
                        .unwrap_or_else(db_deal);
                    } else {
                        flag = database.insert_one_data(DEVICE_TABLE,&kv)
                        .unwrap_or_else(db_deal);
                    }
                    if flag == false {
                        publish_message(&response_topic, &package_message(&token, response_data));
                        return;
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    publish_message(&response_topic, &package_message(&token, response_data));
                    return;
                }
            }
        }
        response_data["status"] = JsonValue::String("OK".to_string());
        publish_message(&response_topic, &package_message(&token, response_data));
    });
}

pub fn response_device_register_get(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut data: Vec<Vec<String>> = Vec::new();
    let mut response_data = Object::new();
    let get_values = Vec::from([
            "model".to_string(), "port".to_string(), 
            "addr".to_string(), "desc".to_string(),
            "guid".to_string(), "appname".to_string(),
            "manuID".to_string(), "isreport".to_string()
    ]);

    response_data.insert("body", JsonValue::Array(Vec::new()));
    {
        DATABASE.with(|lock| {
            let tmp = lock.lock().unwrap_or_else(move|e|{
                println!("{}", e);
                process::exit(0);
            });
            let database = &mut *tmp.borrow_mut();

            data = database.select_all(DEVICE_TABLE).unwrap_or_else(|e| {
                println!("{}", e);
                return Vec::new();
            });
        });
    }
    if data.len() == 0 {
        publish_message(&response_topic, &package_message(&token, response_data));
        return;
    }
    let mut tmp: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for d in data {
        let model = d[0].clone();
        let port = d[1].clone();
        let mut id = String::new();
        id.push_str(&model);
        id.push('/');
        id.push_str(&port);

        match tmp.get_mut(&id) {
            None => {
                tmp.insert(id, Vec::from([d]));
            },
            Some(array) => {
                array.push(d);
            }
        }
    }

    for (key, value) in tmp {
        let sps: Vec<&str> = key.split('/').collect();
        let model = sps[0].to_string();
        let port = sps[1].to_string();
        let mut item = Object::new();

        item.insert("model", JsonValue::String(model));
        item.insert("port", JsonValue::String(port));
        item.insert("body", JsonValue::new_array());

        for dev in value {
            let len = dev.len();
            let mut son = Object::new();

            for i in 2..len {
                son.insert(&get_values[i], JsonValue::String(dev[i].clone()));
            }
            item["body"].push(son).unwrap_or_else(|e| {
                println!("{}", e);
            });
        }
        println!("{}", item.dump());
        response_data["body"].push(item).unwrap_or_else(|e| {
            println!("{}", e);
        });
    }
    publish_message(&response_topic, &package_message(&token, response_data));
}

pub fn response_device_guid_get(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();
    let mut kvs: Vec<Vec<KeyValue>> = Vec::new();
    let devices = get_array_array("body", &request_message);
    let keys: Vec<String> = Vec::from([
        String::from("model"), String::from("port"), String::from("addr")
    ]);
    
    response_data.insert("body", JsonValue::Array(Vec::new()));
    for dev in devices.members() {
        let mut kv: Vec<KeyValue> = Vec::new();

        for i in &keys {
            let param = get_key_value_str(&i, dev).unwrap_or_else(|| {
                return String::new();
            });

            if param.len() == 0 {
                publish_message(&response_topic, &package_message(&token, response_data));
                return;
            }
            kv.push(KeyValue {
                key: i.clone(),
                value: param
            });
        }
        kvs.push(kv);
    }
    {
        DATABASE.with(move |lock| {
            let tmp = lock.lock().unwrap_or_else(move|e|{
                println!("{}", e);
                process::exit(0);
            });
            let database = &mut *tmp.borrow_mut();
            let db_deal = |e: sqlite::Error| {
                println!("db model set error {}", e);
                return false;
            };
            let keys = Vec::from(["guid".to_string()]);

            for kv in kvs {
                let result = database.select_by_colunm(DEVICE_TABLE, keys.clone() , &kv).unwrap_or_else(|e| {
                    println!("{}", e);
                    return Vec::new();
                });

                if result.len() == 0 {
                    continue;
                }
                if result[0].len() == 0 {
                    continue;
                }
                let mut item = Object::new();
                println!("{:?}", kv);
                let guid = result[0][0].clone();
                let mut dev = kv[0].value.clone();
                dev.push('_');
                dev.push_str(&guid);
                
                item.insert("guid", JsonValue::String(guid));
                item.insert("dev", JsonValue::String(dev));
                response_data["body"].push(item).unwrap_or_else(|e| {
                    println!("{}", e);
                    return;
                });
            }
            publish_message(&response_topic, &package_message(&token, response_data));
        });
    }
}

pub fn response_device_cancel_register(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();
    let mut kvs: Vec<Vec<KeyValue>> = Vec::new();
    let devices = get_array_array("body", &request_message);
    let keys: Vec<String> = Vec::from([
        String::from("model"), String::from("port"), String::from("addr")
    ]);
    
    response_data.insert("status", JsonValue::String(String::from("FAILURE")));
    for dev in devices.members() {
        let mut kv: Vec<KeyValue> = Vec::new();

        for i in &keys {
            let param = get_key_value_str(&i, dev).unwrap_or_else(|| {
                return String::new();
            });

            if param.len() == 0 {
                publish_message(&response_topic, &package_message(&token, response_data));
                return;
            }
            kv.push(KeyValue {
                key: i.clone(),
                value: param
            });
        }
        kvs.push(kv);
    }
    DATABASE.with(move |lock| {
        let tmp = lock.lock().unwrap_or_else(move|e|{
            println!("{}", e);
            process::exit(0);
        });
        let database = &mut *tmp.borrow_mut();
        let db_deal = |e: sqlite::Error| {
            println!("db model set error {}", e);
            return false;
        };

        for kv in kvs {
            let flag = database.delete_data(DEVICE_TABLE, &kv).unwrap_or_else(|e| {
                println!("{}", e);
                return false;
            });

            if flag == false {
                publish_message(&response_topic, &package_message(&token, response_data));
                return;
            }
        }
        response_data["status"] = JsonValue::String(String::from("OK"));
        publish_message(&response_topic, &package_message(&token, response_data));
    });
}
