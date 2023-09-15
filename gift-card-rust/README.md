# gift-card-rust demo application

A demo application written in [Rust](https://www.rust-lang.org/),
using [Axon Synapse](https://library.axoniq.io/synapse-quick-start/development/index.html)
and [Axon Server](https://developer.axoniq.io/axon-server-enterprise/overview).

Additionally, the application is using [fmodel-rust](../fmodel-rust) to
implement [CQRS](https://martinfowler.com/bliki/CQRS.html)
and [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html) patterns effectively.

It focuses around a simple gift-card domain, designed to show robust implementation of DDD/CQRS/ES concepts.

## The structure of the application

A bit simplified, this architectural style is characterized by two key architectural attributes:

- There is a core with the core business logic, and a shell that handles interactions with the outside world, such as
  persisting data in databases or providing UIs to interact with end users.
- The shell can call the core, but the core cannot call the shell and the core is even unaware of the existence of the
  shell. This is also known as the Dependency Rule(s).

- It is similar to `Hexagonal Architecture`, `Ports and Adapters`, `Clean Architecture`, `Onion Architecture` which have
  these two attributes in common.

In this demo we have categorized the code into three layers:

- `Domain` - **pure** declaration of the program logic, the core, the inner layer, the part that is not aware of the
  outside world, the part that is testable in isolation, the part that is reusable.
  - [gift_card_api.rs](src/gift_card_api.rs) (`command side + query side`)
  - [gift_card_command_handler.rs](src/gift_card_command_handler.rs) (`command side`)
  - [gift_card_event_handler.rs - TODO] (`query side`)
- `Application` - orchestrates the execution of the logics, by loading the state from the `Infrastructure / Fetch`,
  calling the `Domain` logic, and persisting the state back to the `Infrastructure / Save`.
  - [main.rs](src/main.rs)
- `Adapters/Infrastructure` - infrastructure code, the shell, the outer layer, the part that is aware of the outside
  world.
  - [gift_card_aggregate_controller.rs](src/gift_card_aggregate_controller.rs) (`command side`) - HTTP command
    subscriber/callback
  - [gift_card_command_gateway.rs](src/gift_card_command_gateway.rs) (`command side`) - HTTP command publisher
  - [gift_card_event_repository.rs](src/gift_card_event_repository.rs) (`command side`) - Axon Server event repository
  - [gift_card_event_handler_materialized_view_controller.rs - TODO] (`query side`) - HTTP event subscriber/callback
  - [gift_card_view_state_repository.rs - TODO] (`query side`) - Postgres DB view state repository
  - [gift_card_controller.rs](src/gift_card_controller.rs) (`command side + query side`) - HTTP/REST API, facing users

Additionally, the components are split per command and query side of the application. This is CQRS pattern influencing
the structure of the application.

## Running Axon Server and Synapse in Docker

Set the correct :

- [download latest version](https://download.axoniq.io/axonserver/axon-synapse.zip) of Synapse JAR and copy it
  to [synapse](synapse) folder. Check the [synapse/Dockerfile](synapse/Dockerfile) for the exact name of the JAR file.

```bash
docker-compose up -d
```

> To shut it down:

```bash
docker-compose down -v
```

## Run the application

```bash
cargo run
```

---

Created with :heart: by [AxonIQ](http://axoniq.io)

[axon]: https://axoniq.io/