mod decider;
mod aggregate;
mod decider_example;
mod aggregate_example;
mod view;
mod view_example;
mod materialized_view;
mod materialized_view_example;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    decider_example::main();
    aggregate_example::main().await;
    view_example::main();
    materialized_view_example::main().await;
}
