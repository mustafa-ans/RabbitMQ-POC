use amqprs::DELIVERY_MODE_PERSISTENT;
use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use std::{thread, time};


async fn connect_rabbitmq(connection_details: &RabbitConnect) -> Connection {
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

async fn send(
    connection: &mut amqprs::connection::Connection,
    channel: &mut Channel,
    connection_details: &RabbitConnect,
    exchange: &str,
    result: &str,
) {
    if !connection.is_open() {
        println!("Connection not open");
        *connection = connect_rabbitmq(connection_details).await;
        *channel = channel_rabbitmq(&connection).await;
        println!("{}", connection);
    }

    if !channel.is_open() {
        println!("channel is not open, does exchange Newtestex exist on rabbitMQ?");
        *channel = channel_rabbitmq(&connection).await;
    } else {
         // Specify the exchange name "Newtestex" and the routing key "route_to_newtestq"
        let args = BasicPublishArguments::default()
            .exchange(exchange.to_string())
            .routing_key("route_to_newtestq".to_string())
            .finish();
        channel
            .basic_publish(
                BasicProperties::default()
                    .with_delivery_mode(DELIVERY_MODE_PERSISTENT)
                    .finish(),
                result.into(),
                args,
            )
            .await
            .unwrap();
    }
}

async fn send_message_to_queue(
    connection: &mut amqprs::connection::Connection,
    channel: &mut Channel,
    connection_details: &RabbitConnect,
) {
    let message = "Message from publisher";
    println!("Sending message: {}", message);

    send(connection, channel, connection_details, "Newtestex", &message).await;
}

struct RabbitConnect {
    host: String,
    port: u16,
    username: String,
    password: String,
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

    loop {
        send_message_to_queue(
            &mut connection,
            &mut channel,
            &connection_details
        )
        .await;
        thread::sleep(time::Duration::from_millis(2000));
        
    }
}
