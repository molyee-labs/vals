use result::*;
use net::message::Message;

pub trait Connection {
    fn send(&self, m:Message) -> Result<()>;
    fn receive(&self) -> Result<Message>;
}