# Axon Rust

This contains a [Axon Synapse rust client](/synapse-client), based on the open api generated code.
For now, we didn't publish this crate, to forking this project is the easiest way to start using it.
In addition, we added demo projects, as especially implementing the webhooks correctly can be challenging.

## Synapse

The basic synapse capabilities can be boiled down using the three different event types:
- Publishing an event message.
- Sending a command to be executed.
- Sending a query to fetch some result.
- Subscribing to some or all the event messages.
- Register a handler for one or multiple command messages.
- Registering a handler for one or more query messages.

You can find more about synapse on the [AxonIQ Library](https://library.axoniq.io/synapse-quick-start/development/index.html).

## Demo projects

Both demo project use the gift card domain, which is also used in the [jvm example project](https://github.com/AxonIQ/giftcard-demo). There are some difference between the two demo projects. For example, they use a different message format, and different http server dependencies. Please note the main purpose of the demo project is to help you on your way using the synapse api, not as example of production worthy applications.

### Demo project using fmodel-rust and rocket

Please see [readme](gift-card-rust/README.md) in the subfolder to find out more about this demo project.

### Demo project using protobuf and warp

Please see [readme](gift-card-proto/README.md) in the subfolder to find out more about this demo project.
