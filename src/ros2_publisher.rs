use mio::{Events, Poll, PollOpt, Ready, Token};
use mio_extras::timer;
use ros2_client::{Context, Node, NodeOptions};
use rustdds::{
  policy::{self, Deadline, Lifespan},
  Duration, QosPolicies, QosPolicyBuilder,

};
use crate::types::SetConfigRequest;

pub fn send_message(message: SetConfigRequest) {
  let mut node = create_node();
  let topic_qos = create_qos();

  let chatter_topic = node
    .create_topic(
      "/chatter",
      String::from("ros2_esp32_interfaces::msg::dds_::SetConfigRequest_"),
      &topic_qos,
    )
    .unwrap();
  let chatter_publisher = node
    .create_publisher::<SetConfigRequest>(&chatter_topic, None)
    .unwrap();

  let mut talk_timer: timer::Timer<()> = timer::Builder::default().build();

  let poll = Poll::new().unwrap();

  poll
    .register(&talk_timer, Token(1), Ready::readable(), PollOpt::edge())
    .unwrap();

  talk_timer.set_timeout(std::time::Duration::from_secs(2), ());

  let mut events = Events::with_capacity(8);

  loop {
    poll.poll(&mut events, None).unwrap();

    for event in events.iter() {
      match event.token() {
        Token(1) => {
          chatter_publisher
            .publish(message.clone())
            .unwrap_or_else(|e| println!("publish failed: {:?}", e));
          talk_timer.set_timeout(std::time::Duration::from_secs(2), ());
        }
        _ => println!(">>> Unknown poll token {:?}", event.token()),
      } // match
    } // for
  } // loop
} // main

fn create_qos() -> QosPolicies {
  let service_qos: QosPolicies = {
    QosPolicyBuilder::new()
      .history(policy::History::KeepLast { depth: 10 })
      .reliability(policy::Reliability::Reliable {
        max_blocking_time: Duration::from_millis(100),
      })
      .durability(policy::Durability::Volatile)
      .deadline(Deadline(Duration::DURATION_INFINITE))
      .lifespan(Lifespan {
        duration: Duration::DURATION_INFINITE,
      })
      .liveliness(policy::Liveliness::Automatic {
        lease_duration: Duration::DURATION_INFINITE,
      })
      .build()
  };
  service_qos
}

fn create_node() -> Node {
  let context = Context::new().unwrap();
  let node = context
    .new_node(
      "rustdds_listener",
      "",
      NodeOptions::new().enable_rosout(true),
    )
    .unwrap();
  node
}
