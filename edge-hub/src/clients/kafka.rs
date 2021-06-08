use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;

pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("queue.buffering.max.ms", "0")
        .create()
        .expect("Producer create failed")
}

