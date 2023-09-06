pub mod commands;
pub mod events;
pub mod queries;

pub trait AxonMessage {
    fn name() -> &'static str;
}
