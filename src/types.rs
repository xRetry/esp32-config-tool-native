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
    read_only: bool,
    pins: Vec<PinSetting>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigRequest {
    read_only: bool,
    #[serde(with = "BigArray")]
    pin_modes: [PinMode; 36],
}

impl Message for SetConfigRequest {}

impl SetConfigRequest {
    pub fn new(file_content: FileContent) -> Self {
        let mut pin_modes = [PinMode::Disabled; 36];
        file_content.pins.into_iter().for_each(|p| {
            pin_modes[p.number as usize] = p.mode;
        });

        SetConfigRequest {
            read_only: file_content.read_only,
            pin_modes,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigResponse {
    #[serde(with = "BigArray")]
    pin_error: [u8; 36],
    #[serde(with = "BigArray")]
    pin_modes: [PinMode; 36],
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
