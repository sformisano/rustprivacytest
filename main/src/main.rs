use libcrate::Client;

fn main() {
    let foo = Client::builder();
    println!("{}", foo.hello());
}
