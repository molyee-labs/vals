use net::session::Client;

pub trait Context {
    fn client(&self) -> impl Client;
}
