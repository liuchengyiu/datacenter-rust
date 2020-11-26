use paho_mqtt as mqtt;
use mqtt::{
    CreateOptionsBuilder,
    AsyncClient,
    Message,
    CreateOptions,
};
use std::{
    process,
    thread,
    time::Duration,
};
pub struct MqttPaho
{
    pub client: AsyncClient
}

const TOPICS: &[&str] = &[
                        "+/get/request/database/version",
                        "+/get/request/database/modelschema",
                        "+/get/request/database/model",
                        "+/action/request/database/deletemodel",
                        "+/set/request/database/model",
                        "+/set/request/database/register",
                        "+/get/request/database/register",
                        "+/get/request/database/guid",
                        "+/action/request/database/unregister",
                        "+/notify/event/database/+/+",
                        "+/get/request/database/realtime"
                        ];
const QOS: &[i32] = &[1,1,1,1,1,1,1,1,1,1,1]; 

fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
    cli.subscribe_many(TOPICS, QOS);
}

fn on_connect_failure(cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
    println!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
    cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

impl MqttPaho
{
    pub fn publish(&mut self, topic: &str, data: &str) {
        let message = Message::new_retained(topic, data, 1);

        self.client.publish(message);
    }

    pub fn set_client(&mut self, client: mqtt::AsyncClient) {
        self.client = client;
    }

    pub fn new<T>(host: &str, client_id: &str, callback: T) -> MqttPaho 
    where 
        T: Fn(&str, &str) -> bool + 'static
    {
        let create_opts: CreateOptions = CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(client_id)
            .finalize();
        
        let mut cli = AsyncClient::new(create_opts).unwrap_or_else(|e| {
                println!("Error creating the client: {:?}", e);
                process::exit(1);
        });
        cli.set_connected_callback(|_cli: &AsyncClient| {
            println!("Connected.");
        });
        cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
            println!("Connection lost. Attempting reconnect.");
            thread::sleep(Duration::from_millis(2500));
            cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
        });
        cli.set_message_callback(move |_cli,msg| {
            if let Some(msg) = msg {
                let topic = msg.topic();
                let payload_str = msg.payload_str();
                println!("{} - {}", topic, payload_str);
                callback(&topic.to_string(), &payload_str.to_string());
            }
        });
        MqttPaho {
            client: cli,
        }
    }

    pub fn set_publish_client(&mut self,host: &str ) {
        let create_opts: CreateOptions = CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id("rust2")
            .finalize();
        let mut cli = AsyncClient::new(create_opts).unwrap_or_else(|e| {
                println!("Error creating the client: {:?}", e);
                process::exit(1);
        });
        cli.set_connected_callback(|_cli: &AsyncClient| {
            println!("Connected.");
        });
        cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
            println!("Connection lost. Attempting reconnect.");
            thread::sleep(Duration::from_millis(2500));
            cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
        });
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .mqtt_version(paho_mqtt::MQTT_VERSION_3_1)
            .clean_session(true)
            .finalize();
        cli.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);
        self.set_client(cli);
    }

    pub fn connect_broke(&mut self, keep_alive_interval: u16, mqtt_version: u32, clean_session: bool) {
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(keep_alive_interval as u64))
            .mqtt_version(mqtt_version)
            .clean_session(clean_session)
            .finalize();
        
        self.client.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);
    }

}