# RabbitMQ Rust Example

This project demonstrates a basic implementation of a RabbitMQ publisher and consumer using Rust. The publisher (`rabbit_publisher`) sends a simple string message to a RabbitMQ queue, and the consumer (`rabbit_consumer`) retrieves the message from the queue and displays it in the console.

## Prerequisites

Before running the provided Rust code, ensure that you have the following prerequisites met:

- RabbitMQ server installed and running on your machine.
- RabbitMQ Management Plugin enabled. This plugin provides a web-based UI to manage and monitor RabbitMQ server.

## Getting Started

To get started with this RabbitMQ Rust example, follow these steps:

1. **Install RabbitMQ**: Follow the official [RabbitMQ installation guide](https://www.rabbitmq.com/download.html) to install RabbitMQ on your local machine.

2. **Enable Management Plugin**: Run the following command to enable the RabbitMQ Management Plugin:

3. **Import RabbitMQ Configuration**:
- You can import the provided `rabbitmq_poc_definitions.json` file into RabbitMQ to set up the necessary exchanges, queues, and bindings.
- To import the configuration, navigate to the "Overview" tab in the RabbitMQ Management UI and use the "Import/Export definitions" feature.
- Alternatively, you can send a POST request to `http://localhost:15672/api/definitions` with the JSON payload from the `rabbitmq_poc_definitions.json` file. This can be done using a tool like `curl`:
  ```
  curl -u guest:guest -X POST -H "Content-Type: application/json" -d @rabbitmq_poc_definitions.json http://localhost:15672/api/definitions
  ```
  Replace `guest:guest` with your RabbitMQ username and password if you have changed the defaults.

4. **Build and Run the Rust Code**:
- Navigate to the `rabbit_publisher` directory and build the publisher application:
  ```
  cd rabbit_publisher
  cargo build --release
  ```
- Run the publisher to send a message:
  ```
  cargo run
  ```
- In a separate terminal, navigate to the `rabbit_consumer` directory and build the consumer application:
  ```
  cd rabbit_consumer
  cargo build --release
  ```
- Run the consumer to receive and display the message:
  ```
  cargo run
  ```

## Understanding the Code

- The `rabbit_publisher` Rust project contains the code for the message publisher. It connects to RabbitMQ, publishes a string message to a predefined exchange, and then closes the connection.

- The `rabbit_consumer` Rust project contains the code for the message consumer. It connects to RabbitMQ, subscribes to a queue, waits for messages, displays them in the console, and acknowledges them.

This example is intended to demonstrate the basic interaction between a publisher and a consumer using RabbitMQ with Rust. It is not meant for production use but rather as a starting point for understanding and building more complex messaging systems.
