use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicAckArguments, BasicConsumeArguments, Channel,
    },
    connection::{Connection, OpenConnectionArguments},
};
use std::time;

async fn connect_rabbitmq(connection_details: &RabbitConnect) -> Connection {
    //this is for demo and teaching purposes, you would fetch this information from a config of course
    let mut res = Connection::open(
        &OpenConnectionArguments::new(
            &connection_details.host,
            connection_details.port,
            &connection_details.username,
            &connection_details.password,
        )
        .virtual_host("/"),
    )
    .await;

    while res.is_err() {
        println!("trying to connect after error");
        std::thread::sleep(time::Duration::from_millis(2000));
        res = Connection::open(&OpenConnectionArguments::new(
            &connection_details.host,
            connection_details.port,
            &connection_details.username,
            &connection_details.password,
        ))
        .await;
    }

    let connection = res.unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
    connection
}

async fn channel_rabbitmq(connection: &amqprs::connection::Connection) -> Channel {
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();
    return channel;
}

struct RabbitConnect {
    host: String,
    port: u16,
    username: String,
    password: String,
}
#[allow(unused_variables)]
async fn consume_messages(
    connection: &mut amqprs::connection::Connection,
    channel: &mut Channel,
    queue_name: &str,
) {
    // Assuming the connection and channel are already open
    let args = BasicConsumeArguments::new(queue_name, "consumer_tag");

    let (ctag, mut messages_rx) = channel.basic_consume_rx(args).await.unwrap();

    while let Some(msg) = messages_rx.recv().await {
        let a = msg.content.unwrap();
        let s = String::from_utf8_lossy(&a);

        println!("Received message: {}", s);

        // Acknowledge the message
        let args = BasicAckArguments::new(msg.deliver.unwrap().delivery_tag(), false);
        let _ = channel.basic_ack(args).await;
    }
}

#[tokio::main]
async fn main() {
    let connection_details = RabbitConnect {
        host: "localhost".to_string(),
        port: 5672,
        username: "consumer".to_string(),
        password: "crabs".to_string(),
    };

    let mut connection = connect_rabbitmq(&connection_details).await;
    let mut channel = channel_rabbitmq(&connection).await;

    consume_messages(&mut connection, &mut channel, "Newtestq").await;
}