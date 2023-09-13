# gift-card-rust demo application

A demo application written in [Rust](https://www.rust-lang.org/),
using [Axon Synapse](https://library.axoniq.io/synapse-quick-start/development/index.html)
and [Axon Server](https://developer.axoniq.io/axon-server-enterprise/overview).

Additionally, the application is using [fmodel-rust](../fmodel-rust) to
implement [CQRS](https://martinfowler.com/bliki/CQRS.html)
and [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html) patterns effectively.

It focuses around a simple gift-card domain, designed to show robust implementation of DDD/CQRS/ES concepts.

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