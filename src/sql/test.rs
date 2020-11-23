use crate::sql::operate::DATABASE;
use crate::common::common::KeyValue;

pub fn test() {
    DATABASE.with(|database| {
        let  db = &mut *database.borrow_mut();
        
        let mut kvs: Vec<KeyValue> = Vec::new();
        
        kvs.push(KeyValue {
            key: "id".to_string(),
            value: "TEXT".to_string()
        });
        {
            match db.create_table("fuckyou", kvs) {
                Err(e) => {
                    panic!(e);
                },
                Ok(val) => {
                    if val == -1 {
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
    println!("sql test end");
}