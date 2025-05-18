# Project Iconoclast Demo

This is a demo for an iconoclast-style project with some examples.

This should give an impression how a service looks like (testing, general setup).

In-depth description may be found in the skeleton.

## How to run the demo

- start the demo-services: `docker-compose up -d`
- run migrations: `sqlx migrate run`
- create the demo-topic: `echo "Hello World" | kcat -b localhost:9092 -t hello`
- run the demo: `cargo run`
- open http://localhost:8080