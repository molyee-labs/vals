use net::message::Message;
use result::*;

pub trait Connection {
    fn send(&self, m: Message) -> Result<()>;
    fn receive(&self) -> Result<Message>;
}
