mod ros2_service;
mod ros2_publisher;
mod types;

use serde_yaml;
use types::{FileContent, SetConfigRequest};
use ros2_service::send_request;
use ros2_publisher::send_message;
use clap::{Parser, command};


/// A tool for interacting with an ESP32 microcontroller that is running the ROS2-ESP32 interface.
#[derive(Parser, Debug)]
#[command(name = "ESP32 Config Tool")]
#[command(version = "1.0")]
struct Args {
    /// Path to a YAML config file
    file: String,
}

fn main() {
    let args = Args::parse();
    let reader = std::fs::File::open(args.file).expect("Unable to open file");
    let file_content: FileContent = serde_yaml::from_reader(reader).expect("Unable to parse file");

    let request = SetConfigRequest::new(file_content);

    println!("{:?}", request);
    //send_message(request);
    send_request(request);
}
