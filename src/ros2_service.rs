use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;
use std::time::Duration;
use mio::{Events, Poll, PollOpt, Ready, Token};
use ros2_client::{Message, Service};
use ros2_client::{
  Context, Node, NodeOptions, ServiceMappings,
};
use rustdds::{policy, QosPolicies, QosPolicyBuilder};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BoardConfig {
    use_wifi: bool,
    agent_ip: String,
    agent_port: u32,
    wifi_ssid: String,
    wifi_pw: String,
    refesh_rate_ms: u32,
    service_name: String,
    subscriber_name: String,
    publisher_name: String,
    node_name: String,
    #[serde(with = "BigArray")]
    pin_modes: [u8; 36],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigRequest {
    read_only: bool,
    new_config: BoardConfig
}

impl Message for SetConfigRequest {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetConfigResponse {
    active_config: BoardConfig,
    connection_error: u8,
    #[serde(with = "BigArray")]
    pin_error: [u8; 36],
}

impl Message for SetConfigResponse {}

#[derive(Clone)]
struct SetConfigService {}

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

const RESPONSE_TOKEN: Token = Token(7); // Just an arbitrary value

pub fn send_request(request: SetConfigRequest) {
    println!(">>> ros2_service starting...");
    let mut node = create_node();
    let service_qos = create_qos();

    println!(">>> ros2_service node started");

    let mut client = node
        .create_client::<SetConfigService>(
            ServiceMappings::Enhanced,
            "/add_two_ints",
            service_qos.clone(),
            service_qos.clone(),
        )
        .unwrap();

    println!(">>> ros2_service client created");

    let poll = Poll::new().unwrap();

    poll.register(&client, RESPONSE_TOKEN, Ready::readable(), PollOpt::edge())
        .unwrap();

    println!(">>> request sending...");
    match client.send_request(request) {
        Ok(id) => {
            println!(">>> request sent id={:?}", id);
        }
        Err(e) => {
            println!(">>> request sending error {:?}", e);
        }
    }

    let mut events = Events::with_capacity(100);
    poll.poll(&mut events, Some(Duration::from_secs(1)))
        .unwrap();

    for event in events.iter() {
        match event.token() {
            RESPONSE_TOKEN => {
                while let Ok(Some((id, response))) = client.receive_response() {
                    println!(
                        ">>> Response received: response: {:?} - response id: {:?}, ",
                        response, id,
                    );
                }
            }
            _ => println!(">>> Unknown poll token {:?}", event.token()),
        }
    }
}

fn create_qos() -> QosPolicies {
    let service_qos: QosPolicies = {
        QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable {
            max_blocking_time: rustdds::Duration::from_millis(100),
        })
        .history(policy::History::KeepLast { depth: 1 })
        .build()
    };
    service_qos
}

fn create_node() -> Node {
    let context = Context::new().unwrap();
    let node = context
        .new_node(
            "esp32_config_tool",
            "/rustdds",
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap();
    node
}
