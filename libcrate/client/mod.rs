pub struct Client {}

impl Client {
  pub fn builder() -> ClientBuilder {
    ClientBuilder {}
  }
}

pub struct ClientBuilder {}

impl ClientBuilder {
  pub fn hello(self) -> String {
    String::from("this is weird!")
  }
}