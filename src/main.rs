mod ros2_service;
mod types;

use std::env;
use serde_yaml;
use types::{FileContent, SetConfigRequest};
use ros2_service::send_request;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Provide config file path as console argument");
    let reader = std::fs::File::open(file_path).expect("Unable to open file");
    let file_content: FileContent = serde_yaml::from_reader(reader).expect("Unable to parse file");

    let request = SetConfigRequest::new(file_content);

    println!("{:?}", request);
    send_request(request);
}
