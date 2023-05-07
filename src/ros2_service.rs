use std::time::Duration;
use mio::{Events, Poll, PollOpt, Ready, Token};
use ros2_client::{
  Context, Node, NodeOptions, ServiceMappings,
};
use rustdds::{policy, QosPolicies, QosPolicyBuilder};
use crate::types::{SetConfigService, SetConfigRequest};


const RESPONSE_TOKEN: Token = Token(7); // Just an arbitrary value

pub fn send_request(request: SetConfigRequest) {
    println!(">>> ros2_service starting...");
    let mut node = create_node();
    let service_qos = create_qos();

    println!(">>> ros2_service node started");

    let mut client = node
        .create_client::<SetConfigService>(
            ServiceMappings::Enhanced,
            "/esp32_set_config",
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

    let mut events = Events::with_capacity(1);
    poll.poll(&mut events, Some(Duration::from_secs(10)))
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
            "",
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap();
    node
}
