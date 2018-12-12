use crate::net::message::Message;
use crate::result::*;

pub trait Connection {
    fn send(&self, m: Message) -> Result<()>;
    fn receive(&self) -> Result<Message>;
}
