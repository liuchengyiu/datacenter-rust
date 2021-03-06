use crate::sql::operate::DATABASE;
use crate::common::common::KeyValue;

pub fn test() {
    DATABASE.with(|database| {
        let  rc = database.lock().unwrap();
        let db = &mut *rc.borrow_mut();
        let mut kvs: Vec<KeyValue> = Vec::new();
        let mut params: Vec<KeyValue> = Vec::new();
        let mut ups: Vec<KeyValue> = Vec::new();

        kvs.push(KeyValue {
            key: "id".to_string(),
            value: "TEXT".to_string()
        });
        params.push(KeyValue {
            key: "id".to_string(),
            value: "fuckyou".to_string()
        });
        ups.push(KeyValue {
            key: "id".to_string(),
            value: "loveyou".to_string()
        });
        {
            match db.create_table("fuckyou", &kvs) {
                Err(e) => {
                    panic!(e);
                },
                Ok(val) => {
                    if val {
                        panic!("item is not error");
                    } else {
                        println!("create function exec success");
                    }
                }
            }
            match db.check_table("fuckyou") {
                Err(e) => {
                    panic!(e);
                },
                Ok(val) => {
                    println!("create table is exist");
                    if val == false {
                        panic!("create table is not exist");
                    }
                }
            };
            match db.insert_one_data("fuckyou", &params) {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{}", val);
                    if val == true {
                        println!("insert data success");
                    }
                }
            };
            match db.select_data("fuckyou", &params) {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{:?}", val);
                    println!("select data success");
                }
            };
            match db.update_data("fuckyou", &params, &ups) {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{}", val);
                    if val == true {
                        println!("update data success");
                    }
                }
            };
            match db.select_all("fuckyou") {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{:?}", val);
                    println!("select data success");
                }
            };
            match db.delete_data("fuckyou", &ups) {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{}", val);
                    if val == true {
                        println!("delete data success");
                    }
                }
            };
            match db.select_data("fuckyou", &ups) {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{:?}", val);
                    println!("select data success");
                }
            };
            match db.delete_table("fuckyou") {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(_) => {
                    println!("delete table function exec ok");
                }
            };
            match db.check_table("fuckyou") {
                Err(e) => {
                    println!("{:}?", e);
                    panic!(e);
                },
                Ok(val) => {
                    println!("{}", val);
                    if val == true {
                        panic!("delete table is not success");
                    }
                }
            };
        }
    });
    println!("sql test pass!");
}