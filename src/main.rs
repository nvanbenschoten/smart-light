extern crate postgres;
extern crate chrono;
extern crate iron;
extern crate router;

mod db;
mod http;

fn main() {
    println!("Hello, world!");
    db::Service::new().unwrap();
    http::start().unwrap();
}
