use std::time::Duration;

use rdkafka::{
    config::FromClientConfig, consumer::Consumer, groups::GroupList, util::Timeout, ClientConfig,
};

fn fetch_group_list(url: &str) -> GroupList {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", url.to_string());

    let client =
        rdkafka::consumer::BaseConsumer::from_config(&config).expect("Unable to create consumer");

    client
        .client()
        .fetch_group_list(None, Timeout::After(Duration::from_secs(15)))
        .expect("Unable to fetch group list")
}

fn main() {
    let url = std::env::args().nth(1).expect("Missing URL");
    let groups = fetch_group_list(&url);
    println!("Group Count: {}", groups.groups().len());
    println!("{:?}", groups.groups());
}
