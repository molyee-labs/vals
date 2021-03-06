use crate::net::connection::Connection;
use crate::net::session::Session;
use secure::access::Authority;

pub trait Client {
    fn session(&self) -> Session;
    fn auth(&self) -> Authority;
    fn conn(&self) -> Connection;
}
