use net::client::Client;

pub trait Context {
    fn client<T: Client>(&self) -> T;
}
