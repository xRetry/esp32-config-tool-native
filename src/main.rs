mod ros2_service;

use std::env;
use serde::{Serialize, Deserialize};
use serde_yaml;
use ros2_service::send_request;

#[derive(Serialize, Deserialize, Debug)]
enum PinMode {
    #[serde(alias = "disabled")]
    Disabled,
    #[serde(alias = "digital_input")]
    DigitalInput,
    #[serde(alias = "digital_output")]
    DigitalOutput,
    #[serde(alias = "analog_input")]
    AnalogInput,
    #[serde(alias = "analog_output")]
    AnalogOutput,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransportConfig {
    r#type: String,
    agent_ip: String,
    agent_port: i32,
    wifi_ssid: String,
    wifi_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PinConfig {
    number: u8,
    mode: PinMode,
}

#[derive(Serialize, Deserialize, Debug)]
struct TopicConfig {
    service: String,
    publisher: String,
    subcriber: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileContent {
    pins: Vec<PinConfig>, 
    transport: TransportConfig,
    topics: TopicConfig,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Provide config file path as console argument");
    let reader = std::fs::File::open(file_path).expect("Unable to open file");
    let config: FileContent = serde_yaml::from_reader(reader).expect("Unable to parse file");

    println!("{:?}", config);
    //send_request(request);
}
