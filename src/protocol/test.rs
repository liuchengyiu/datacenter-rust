use crate::mqtt_lib::mqtt_init::{
    init
};
use super::init::register_static_func;
use std::{
    thread,
    time::Duration,
    process
};
use crate::sql::operate::DATABASE;

pub fn test() {
    {
        DATABASE.with(|lock| {
            let tmp = lock.lock().unwrap_or_else(move|e|{
                println!("{}", e);
                process::exit(0);
            });
            let database = &mut *tmp.borrow_mut();
    
            database.init_base_table();
        });
    }
    init();
    register_static_func();
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
    // handle.join().unwrap();
}