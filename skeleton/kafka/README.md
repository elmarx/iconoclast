# Kafka Adapter

The kafka-adapter connects to the *inbound ports* of the application.

## rdkafka

Most code in this project deals with turning [
`rdkafka::message::BorrowedMessage`](https://docs.rs/rdkafka/latest/rdkafka/message/struct.BorrowedMessage.html)
into [domain](../domain)-structs and routing them to their respective inbound port.

Since `KafkaListener` implements `iconoclast::kafka::MessageHandler`,
`iconoclast::kafka::Consumer` can use it and setup consuming from rdkafka and hands over messages. 
