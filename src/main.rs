extern crate postgres;
extern crate chrono;

mod db;

fn main() {
    db::Service::new().unwrap();
    println!("Hello, world!");
}
