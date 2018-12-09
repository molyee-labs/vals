use secure::access::Authority;
use net::session::Session;
use net::connection::Connection;

pub trait Client {
    fn session(&self) -> Session;
    fn auth(&self) -> Authority;
    fn conn(&self) -> Connection;
}
