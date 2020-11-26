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
    process
};
use crate::sql::operate::DATABASE;

static MODEL_TABLE: &str = "model";

pub fn response_model_set(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let model: String = get_key_value_str("model", &request_message).unwrap_or_else( || {
        println!("model is not exist");
        return String::from(""); 
    });
    let mut response_data = Object::new();
    let mut kv: Vec<KeyValue> = Vec::new();
    let mut model_table_param: Vec<KeyValue> = Vec::new();
    let models = get_array_array("body", &request_message);
    let conditions = Vec::from([KeyValue {
        key: "name".to_string(),
        value: model.to_string()
    }]);

    response_data.insert("status", JsonValue::String("FAILURE".to_string()));
    if request_message.len() == 0 || model.len() == 0 {
        publish_message(&response_topic, &package_message(&token, response_data));
        return;
    }
    kv.push(KeyValue {
        key: "name".to_string(),
        value: model.to_string()
    });
    kv.push(KeyValue {
        key: "params".to_string(),
        value: models.dump().replace("\"", "'")
    });
    model_table_param.push(KeyValue {
        key: "guid".to_string(),
        value: "TEXT".to_string()
    });
    for i in models.members() {
        let name = get_key_value_str("name", i).unwrap_or_else(|| {
            println!("name is not exist");
            return String::from(""); 
        });
        
        if name.len() == 0 {
            continue;
        }
        model_table_param.push(KeyValue {
            key: name.to_string(),
            value: "TEXT".to_string()
        });
    }
    if model_table_param.len() == 1 {
        publish_message(&response_topic, &package_message(&token, response_data));
        return;
    }

    DATABASE.with(move |lock| {
        {
            let tmp = lock.lock().unwrap_or_else(move|e|{
                println!("{}", e);
                process::exit(0);
            });
            let database = &mut *tmp.borrow_mut();
            let db_deal = |e: sqlite::Error| {
                println!("db model set error {}", e);
                return false;
            };

            match database.exists_data(MODEL_TABLE, &conditions) {
                Ok(val) => {
                    if val {
                        database.update_data(MODEL_TABLE, &conditions , &kv).unwrap_or_else(db_deal);
                    } else {
                        database.insert_one_data(MODEL_TABLE, &kv).unwrap_or_else(db_deal);
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    publish_message(&response_topic, &package_message(&token, response_data));
                    return;
                }
            }
            database.delete_table(&model).unwrap_or_else(db_deal);
            database.create_table(&model, &model_table_param).unwrap_or_else(db_deal);
        }
        response_data["status"] = JsonValue::String("OK".to_string());
        publish_message(&response_topic, &package_message(&token, response_data));
    });
}

pub fn response_model_get(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();
    let modol_names = get_array_array("body", &request_message);
    let mut kvs: Vec<Vec<KeyValue>> = Vec::new();

    for i in modol_names.members() {
        let mut kv: Vec<KeyValue> = Vec::new();
        let model = i.to_string();
        
        kv.push(KeyValue{
            key: "name".to_string(),
            value: model
        });
        kvs.push(kv);
    }

    response_data.insert("body", JsonValue::Array(Vec::new()));
    DATABASE.with(move |lock| {
        let tmp = lock.lock().unwrap_or_else(move|e|{
            println!("{}", e);
            process::exit(0);
        });
        let database = &mut *tmp.borrow_mut();

        for i in kvs {
            match database.select_data(MODEL_TABLE, &i) {
                Ok(val) =>{
                    if val.len() == 0 {
                        continue;
                    }
                    println!("{:?}", val);

                    let mut item = Object::new(); 
                    let name = val[0][0].clone();
                    let params = val[0][1].clone().replace("'", "\"");
                    let trans = json::parse(&params).unwrap_or_else(|e| {
                        println!("{}", e);
                        return JsonValue::new_array();
                    });

                    item.insert("model", JsonValue::String(name));
                    item.insert("body", trans);
                    response_data["body"].push(item).unwrap_or_else(|e| {
                        println!("{}", e);
                    });
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        publish_message(&response_topic, &package_message(&token, response_data));
    })
}

pub fn response_model_schema(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();

    response_data.insert("body", JsonValue::Array(Vec::new()));
    DATABASE.with(move |lock| {
        let tmp = lock.lock().unwrap_or_else(move|e|{
            println!("{}", e);
            process::exit(0);
        });
        let database = &mut *tmp.borrow_mut();

        match database.select_column_all(MODEL_TABLE, "name") {
            Ok(val) => {
                for result in val {
                    let name = result[0].clone();

                    response_data["body"].push(name).unwrap_or_else(|e| {
                        println!("{}", e);
                    });
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }

        publish_message(&response_topic, &package_message(&token, response_data));
    })
}

pub fn response_model_delete(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();
    let modol_names = get_array_array("body", &request_message);
    let mut kvs: Vec<Vec<KeyValue>> = Vec::new();

    for i in modol_names.members() {
        let mut kv: Vec<KeyValue> = Vec::new();
        let model = i.to_string();
        
        kv.push(KeyValue{
            key: "name".to_string(),
            value: model
        });
        kvs.push(kv);
    }

    response_data.insert("status", JsonValue::String("FAILURE".to_string()));
    DATABASE.with(move |lock| {
        let tmp = lock.lock().unwrap_or_else(move|e|{
            println!("{}", e);
            process::exit(0);
        });
        let database = &mut *tmp.borrow_mut();

        for i in kvs {
            let d = database.delete_data(MODEL_TABLE, &i).unwrap_or_else(move |e| {
                println!("{}", e);
                return false;
            });

            if d == false {
                publish_message(&response_topic, &package_message(&token, response_data));
                return;
            }
        }
        response_data["status"] = JsonValue::String("OK".to_string());
        publish_message(&response_topic, &package_message(&token, response_data));
    });

}