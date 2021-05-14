use futures::{StreamExt};
//use log::{info, warn};

use futures::stream::FuturesUnordered;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use serde_json::{Value};

use crate::{
    config::db::Pool,
    constants,
    models::messages::{ Message as ModelMessage, AddMessage},
};
use serde::Deserializer;
use uuid::Uuid;
use futures::executor;

#[derive(Clone, Serialize, Deserialize)]
pub struct KafkaData {
    pub topic: String,
    pub user_key: String,
    pub payload: String,
}
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }
    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}
// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub async fn start(
    brokers: String,
    group_id: String,
    input_topic: String,
    pool: Pool
) {
    let context = CustomContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&input_topic])
        .expect("Can't subscribe to specified topics");

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();

    println!(" kafka consumer starting {:?} input_topic {:?}", brokers, input_topic);
    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };

                // println!("  payload == {:?}", payload);
                match serde_json::from_str::<KafkaData>(&payload) {
                    Ok(payload_data) => {
                        if payload_data.topic.starts_with("NEWMESSAGE_") {
                            println!("  payload_data.payload == {:?}", payload_data.payload);
                            match serde_json::from_str::<AddMessage>(&payload_data.payload) {
                                Ok(add_message) => {
                                    // println!("serde_json::from_str::<AddMessage> add_message {:?}", add_message);
                                    ModelMessage::add_new_msg(&pool.clone(), add_message).await;
                                },
                                Err(msg) => {
                                    // handle error here
                                    println!("serde_json::from_str::<Value> Error {:?}", msg);
                                }
                            }
                        }
                    },
                    Err(msg) => {
                        // handle error here
                        println!("serde_json::from_str::<KafkaData> Error {:?}", msg);
                    }
                }
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        // println!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
