use std::env;
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    #[serde(alias = "disabled")]
    Disabled,
    #[serde(alias = "input")]
    Input,
    #[serde(alias = "output")]
    Output,
}

#[derive(Serialize, Deserialize, Debug)]
enum PinType {
    #[serde(alias = "digital")]
    Digital,
    #[serde(alias = "analog")]
    Analog,
}

#[derive(Serialize, Deserialize, Debug)]
struct PinConfig {
    #[serde(alias = "number")]
    pin_nr: u8,
    #[serde(alias = "type")]
    pin_type: PinType,
    direction: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pins: Vec<PinConfig>,
}

fn set_pins(pin_configs: &Vec<PinConfig>) {
    for p in pin_configs.iter() {
        println!("{:?}", p);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Provide config file path as console argument");
    let reader = std::fs::File::open(file_path).expect("Unable to open file");
    let config: Config = serde_yaml::from_reader(reader).expect("Unable to parse file");

    set_pins(&config.pins);
    println!("{:?}", config);
}
