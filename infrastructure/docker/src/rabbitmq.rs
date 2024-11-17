use serde_json::{json, Value};
use tokio::sync::Notify;
use crate::structs::BaseConsumer;
use std::{env, future::Future};

use amqprs::{
    BasicProperties,
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback}, 
    channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments, QueueDeclareArguments}, 
    connection::{Connection, OpenConnectionArguments}
};

pub async fn consume_rabbitmq_message<F, Fut>(queue_name: &str, routing_key: &str, exchange_name: &str, handler: F) where 
    F: Fn(Value) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static {
    // open a connection to RabbitMQ server
    let connection = create_rabbitmq_connection().await;

    // open channel
    let channel = connection.open_channel(None).await.unwrap();

    // register default callback (required)
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    // declare a server-named transient queue
    // let queue_name = create_queue(&channel, queue_name).await;
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named(
            queue_name,
        ))
        .await
        .unwrap()
        .unwrap();

    // bind the queue to exchange
    // bind_queue(&channel, &queue_name, routing_key, exchange_name).await;
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .unwrap();

    // start consumer, auto ack
    let args = BasicConsumeArguments::new(&queue_name, "basic_consumer")
        .manual_ack(false)
        .finish();

    let base_consumer= BaseConsumer::new(handler);

    channel
        .basic_consume(base_consumer, args)
        .await
        .unwrap();

    // consume forever
    let guard = Notify::new();
    guard.notified().await;
}

pub async fn publish_rabbitmq_message(exchange_name: &str, routing_key: &str, message: &str) {
    // open a connection to RabbitMQ server
    let connection = create_rabbitmq_connection().await;

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();

    // make sure message is json
    let message_json = json!({ "message": message });

    // parse content to bytes
    let content = message_json.to_string().into_bytes();

    // create arguments for basic_publish
    let args = BasicPublishArguments::new(exchange_name, routing_key);
    
    // publish
    channel
        .basic_publish(BasicProperties::default(), content, args)
        .await
        .unwrap();
}

async fn create_rabbitmq_connection() -> Connection {
    let hostname = env::var("RABBITMQ_HOSTNAME")
        .unwrap_or_else(|_| "localhost".to_string());
    // create connection
    let connection = Connection::open(&OpenConnectionArguments::new(
        &hostname,
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();
    // register default callback (required)
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    connection
}

async fn _create_channel(connection: Connection) -> Channel {
    // open channel
    let channel = connection.open_channel(None).await.unwrap();
    // register default callback (required)
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();
    channel
}

async fn _create_queue(channel: &Channel, queue_name: &str) -> String {
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named(
            queue_name,
        ))
        .await
        .unwrap()
        .unwrap();
    queue_name
}

async fn _bind_queue(channel: &Channel, queue_name: &str, routing_key: &str, exchange_name: &str) {
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .unwrap();
}
