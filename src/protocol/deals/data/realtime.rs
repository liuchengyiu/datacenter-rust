use crate::common::common::{
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
};
use crate::sql::operate::DATABASE;

static MODEL_TABLE: &str = "model";
static DEVICE_TABLE: &str = "device";

pub fn response_realtime_set(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let deal_str_get_none = || {
        return String::new()
    } ;
    let token: String = get_token(&request_message);
    let data_row: String = get_key_value_str("data_row", &request_message).unwrap_or_else(deal_str_get_none);
    let mut response_data = Object::new();
    let datas = get_array_array("body", &request_message);
    let topics: Vec<&str> = topic.split("/").collect();
    let model_name = topics[topics.len() - 2].to_string();
    let model_guid = topics[topics.len() - 1].to_string();
    let guid =  model_guid.replace(&model_name, "").replace("_", "");
    let mut kv: Vec<KeyValue> = Vec::new();

    response_data.insert("status", JsonValue::String("FAILURE".to_string()));
    if data_row.len() == 0 || !data_row.eq("single") || datas.len() == 0 
                || model_name.len() == 0 || model_guid.len() < 5 {
        println!("{} {} {} {} ", data_row, datas, model_name, model_guid);
        publish_message(&response_topic, &package_message(&token, response_data));
        return;
    }

    for row in datas.members() {
        let name = get_key_value_str("name", row).unwrap_or_else(deal_str_get_none);

        if name.len() == 0 {
            publish_message(&response_topic, &package_message(&token, response_data));
            return;
        }
        kv.push(KeyValue {
            key: name,
            value: row.dump().replace("\"", "'")
        })
    }
    kv.push(KeyValue {
        key: "guid".to_string(),
        value: guid.clone()
    });
    let condition = Vec::from([ KeyValue {key: "guid".to_string(), value: guid.clone()}]);
    println!("{:?}", kv);

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
        let flag = database.check_table(&model_name).unwrap_or_else(db_deal);

        if flag == false {
            publish_message(&response_topic, &package_message(&token, response_data));
            return;
        }
        let flag = database.exists_data(MODEL_TABLE, &Vec::from([
            KeyValue {key: "name".to_string(), value: model_name.clone() }
        ])).unwrap_or_else(db_deal);
        if flag == false {
            publish_message(&response_topic, &package_message(&token, response_data));
            return;
        }
        let flag = database.exists_data(DEVICE_TABLE, &condition).unwrap_or_else(db_deal);
        if flag == false {
            publish_message(&response_topic, &package_message(&token, response_data));
            return;
        }
        match database.exists_data(&model_name, &condition) {
            Ok(val) => {
                if val {
                    database.update_data(&model_name, &condition, &kv).unwrap_or_else(db_deal);
                } else {
                    database.insert_one_data(&model_name, &kv).unwrap_or_else(db_deal);
                }
            },
            Err(e) => {
                println!("{}", e);
                publish_message(&response_topic, &package_message(&token, response_data));
                return;
            }
        }

        response_data["status"] = JsonValue::String("OK".to_string());
        publish_message(&response_topic, &package_message(&token, response_data));
    });
}

pub fn response_realtime_get(topic: &str, payload: &str) {
    let response_topic = gen_topic(topic);
    let request_message = json::parse(payload).unwrap_or_else(move |e| {
        println!("parse message {}", e);
        return JsonValue::new_object()
    });
    let token: String = get_token(&request_message);
    let mut response_data = Object::new();
    let types = get_array_array("body", &request_message);
    let mut kvs: Vec<Vec<KeyValue>> = Vec::new();

    response_data.insert("body", JsonValue::new_array());
    for t in types.members() {
        let dev_flag = t.has_key("dev");
        let model_flag = t.has_key("modelname");
        let mut kv: Vec<KeyValue> = Vec::new();

        if dev_flag == model_flag  {
            continue;
        }

        if dev_flag {
            let dev = get_key_value_str("dev", t).unwrap_or_else(|| {
                println!("null");
                return String::new();
            });
            let body = get_array_array("body", t);
            let ds: Vec<&str> = dev.split("_").collect();
            if ds.len() < 2 {
                continue;
            }
            let model = ds[0].to_string();
            let guid = ds[1].to_string();

            kv.push(KeyValue {
                key: "type".to_string(),
                value: "dev".to_string()
            });
            kv.push(KeyValue {
                key: "model".to_string(),
                value: model
            });
            kv.push(KeyValue {
                key: "guid".to_string(),
                value: guid
            });
            
            if body.is_empty() == false {
                for d in body.members() {
                    kv.push(KeyValue {
                        key: d.to_string(),
                        value: d.to_string()
                    });
                }
            }
            kvs.push(kv);
            continue;
        }

        if model_flag {
            let model = get_key_value_str("modelname", t).unwrap_or_else(|| {
                println!("null");
                return String::new();
            });

            if model.len() == 0 {
                continue;
            }
            kv.push(KeyValue {
                key: "type".to_string(),
                value: "model".to_string()
            });
            kv.push(KeyValue {
                key: "model".to_string(),
                value: model
            });
            kvs.push(kv);
        }
    }

    DATABASE.with(move |lock| {
        let tmp = lock.lock().unwrap_or_else(move|e|{
            println!("{}", e);
            process::exit(0);
        });
        let database = &mut *tmp.borrow_mut();
        let db_deal = |e: sqlite::Error| {
            println!("db model set error {}", e);
            return Vec::new();
        };
        let deal_parse = |e: json::Error| {
            println!("{}",e);
            return JsonValue::new_object();
        };
        let deal_insert =  |e: json::Error| {
            println!("{}",e);
            return;
        };
    
        for kv in kvs {
            if kv.len() < 2 {
                continue;
            }
            if kv[0].value == "model" {
                let model_name = kv[1].value.clone();
                let results = database.select_all(&model_name).unwrap_or_else(db_deal);
                let mut item = JsonValue::new_object();

                item.insert("modelname", JsonValue::String(model_name.clone())).unwrap_or_else(deal_insert);
                item.insert("body", JsonValue::new_array()).unwrap_or_else(deal_insert);
                for data in results {
                    let mut son = JsonValue::new_object();
                    let mut dev = model_name.clone();

                    dev.push('_');
                    dev.push_str(&data[0].clone());
                    son.insert("dev", JsonValue::String(dev)).unwrap_or_else(deal_insert);
                    son.insert("body", JsonValue::new_array()).unwrap_or_else(deal_insert);
                    for s in 1..data.len() {
                        let tmp = json::parse(&data[s].replace("'", "\"")).unwrap_or_else(deal_parse);
                            
                        son["body"].push(tmp).unwrap_or_else(deal_insert);
                    }
                    item["body"].push(son).unwrap_or_else(deal_insert);
                }
                response_data["body"].push(item).unwrap_or_else(deal_insert);
                continue;
            } else if kv[0].value == "dev" {
                if kv.len() < 3 {
                    continue;
                }
                let model_name = kv[1].value.clone();
                let param = Vec::from([kv[2].clone()]);
                let mut item = JsonValue::new_object();
                let mut dev = model_name.clone();

                dev.push('_');
                dev.push_str(&kv[2].value.clone());
                item.insert("dev", dev.clone()).unwrap_or_else(deal_insert);
                item.insert("body", JsonValue::new_array()).unwrap_or_else(deal_insert);
                if kv.len() == 3 {
                    let result = database.select_data(&model_name, &param)
                        .unwrap_or_else(db_deal);
                    for data in result {
                        for s in 1..data.len() {
                            let tmp = json::parse(&data[s].replace("'", "\"")).unwrap_or_else(deal_parse);
                            
                            item["body"].push(tmp).unwrap_or_else(deal_insert);
                        }
                        break;
                    }
                } else if kv.len() > 3 {
                    let mut condition: Vec<String> = Vec::new();

                    for k in 3..kv.len() {
                        condition.push(kv[k].value.clone());
                    }
                    let result = database.select_by_colunm(&model_name, condition.clone(), &param)
                                            .unwrap_or_else(db_deal);
                    for data in result {
                        for s in 0..data.len() {
                            let tmp = json::parse(&data[s].replace("'", "\"")).unwrap_or_else(deal_parse);

                            item["body"].push(tmp).unwrap_or_else(deal_insert);
                        }
                        break;
                    }
                }
                response_data["body"].push(item).unwrap_or_else(deal_insert);
            }
        }
        publish_message(&response_topic, &package_message(&token, response_data));
    });

}

