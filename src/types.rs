use ros2_client::{Message, Service};
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;
use serde_repr::Serialize_repr;

#[derive(Serialize_repr, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
enum PinMode {
    #[serde(alias = "disabled")]
    Disabled = 0,
    #[serde(alias = "digital_input")]
    DigitalInput = 1,
    #[serde(alias = "digital_output")]
    DigitalOutput = 2,
    #[serde(alias = "analog_input")]
    AnalogInput = 3,
    #[serde(alias = "analog_output")]
    AnalogOutput = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinSetting {
    number: u8,
    mode: PinMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileContent {
    change_pins: bool,
    change_transport: bool,
    change_node: bool,
    pins: Vec<PinSetting>, 
    transport: TransportConfig,
    node: NodeConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransportConfig {
    use_wifi: bool,
    agent_ip: String,
    agent_port: String,
    wifi_ssid: String,
    wifi_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinConfig {
    #[serde(with = "BigArray")]
    pin_modes: [PinMode; 36],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeConfig {
    node_name: String,
    service_name: String,
    publisher_name: String,
    subcriber_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigRequest {
    change_pins: bool,
    new_pin_config: PinConfig,
    change_node: bool,
    //new_node_config: NodeConfig,
    change_transport: bool,
    //new_transport_config: TransportConfig,
}

impl Message for SetConfigRequest {}

impl SetConfigRequest {
    pub fn new(file_content: FileContent) -> Self {
        let mut pin_modes = [PinMode::Disabled; 36];
        file_content.pins.into_iter().for_each(|p| {
            pin_modes[p.number as usize] = p.mode;
        });

        SetConfigRequest {
            change_pins: file_content.change_pins,
            change_node: file_content.change_node,
            change_transport: file_content.change_transport,
            //new_transport_config: file_content.transport,
            //new_node_config: file_content.node,
            new_pin_config: PinConfig{ pin_modes },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigResponse {
    active_pin_config: PinConfig,
    #[serde(with = "BigArray")]
    pin_error: [u8; 36],
    //active_node_config: NodeConfig,
    node_error: u8,
    //new_transport_config: TransportConfig,
    transport_error: u8,
}

impl Message for SetConfigResponse {}

#[derive(Clone)]
pub struct SetConfigService {}

impl Service for SetConfigService {
    type Request = SetConfigRequest;
    type Response = SetConfigResponse;

    fn request_type_name() -> String {
        "ros2_esp32_interfaces::srv::dds_::SetConfig_Request_".to_owned()
    }

    fn response_type_name() -> String {
        "ros2_esp32_interfaces::srv::dds_::SetConfig_Response_".to_owned()
    }
}
