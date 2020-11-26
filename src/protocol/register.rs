use std::{
    collections::HashMap,
    cell::RefCell,
    sync::{Arc, Mutex}
};
use lazy_static::lazy_static;
pub struct Register
{
    pub callbacks: HashMap<String, &'static (dyn Fn(& str, & str) + Sync)>
}

impl Register
{
    pub fn new() -> Register {
        Register {
            callbacks: HashMap::new() ,
        }
    }

    pub fn deal(&mut self, topic: &str, payload: &str) {
        let tran = topic.to_string().clone();
        let trans:Vec<&str> = tran.split("/").collect();
        let mut traned: String = String::new();
        let len = trans.len();
        let flag = trans[1] == "notify";

        for i in 0..len {
            if i == 0 {
                traned.push_str("+/");
                continue;
            }
            if i >= len - 2 && flag{
                traned.push_str("+/");
                continue;
            }
            traned.push_str(trans[i]);
            traned.push('/');
        }
        traned.pop();
        println!("{}", traned);
        println!("hashmap len {}", self.callbacks.len());
        match self.callbacks.get(&traned) {
            None => {
                println!("no match function!");
            },
            Some(func) => {
                func(topic, payload);
            }
        }
    }

    pub fn add_callback(&mut self, match_str: &str, match_func: &'static (dyn Fn(& str, & str) + Sync)) {
        println!("register {}", match_str);
        self.callbacks.insert(match_str.to_string().clone(), match_func);
        println!("after insert {}", self.callbacks.len());
    }

    pub fn delete_callback(&mut self, match_str: &str) {
        self.callbacks.remove(match_str);
    }

    pub fn update_callback(&mut self, match_str: &str, match_func: &'static (dyn Fn(& str, & str) + Sync)) {
        self.callbacks.insert(match_str.to_string().clone(), match_func);
    }
}

lazy_static! {
    pub static ref REGISTER: Arc<Mutex<RefCell<Register >>> = Arc::new(Mutex::new(RefCell::new(Register::new())));
}
// pub static mut REGISTER: Option<Arc<Mutex<RefCell<Register >>>> = None;


pub fn dispatch_message(topic: &str, payload: &str) -> bool {
    let tmp = REGISTER.lock();
    let tmp_ = tmp.unwrap();
    let deals = &mut *tmp_.borrow_mut();
    
    deals.deal(topic, payload);

    true
}

pub fn register_func(match_str: &str, match_func: &'static (dyn Fn(& str, & str) + Sync)) {
    let tmp = REGISTER.lock();
    let tmp_ = tmp.unwrap();
    let deals = &mut *tmp_.borrow_mut();
    
    deals.add_callback(match_str, match_func);
}


