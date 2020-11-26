use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use super::mqtt_h::MqttPaho;
use paho_mqtt::{
    MQTT_VERSION_3_1_1,
    MQTT_VERSION_3_1
};

use crate::protocol::register::dispatch_message;
use lazy_static::lazy_static;

thread_local! {
    pub static MQTTCLIENT: Arc<Mutex<RefCell<MqttPaho>>> = Arc::new(Mutex::new(
        RefCell::new(MqttPaho::new("tcp://192.168.1.103:1883", "rustc1", dispatch_message))))
}
lazy_static! {
    pub static ref  MQTTCLIENTSEND: Arc<Mutex<RefCell<MqttPaho>>> = Arc::new(Mutex::new(
        RefCell::new(MqttPaho::new("tcp://192.168.1.103:1883", "rustc3", dispatch_message))));
}
pub fn publish_message(topic: &str, data: &str) {
    let locked = MQTTCLIENTSEND.lock().unwrap();
    let client = &mut *locked.borrow_mut();

    client.publish(topic, data);
    println!("publish");
}

pub fn init() {
    MQTTCLIENT.with(|lock| {
        let locked = lock.lock().unwrap();
        let client = &mut *locked.borrow_mut();

        client.connect_broke(20, MQTT_VERSION_3_1, true);
    });
    let locked = MQTTCLIENTSEND.lock().unwrap();
    let client = &mut *locked.borrow_mut();
    
    client.set_publish_client("tcp://192.168.1.103:1883");
    thread::sleep(Duration::from_millis(2000));
}