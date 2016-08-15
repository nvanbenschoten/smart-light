extern crate postgres;
extern crate num;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

mod db;
mod http;
mod curtain;

fn main() {
    let curtain_mgr = curtain::Manager::new();
    db::Service::new(&curtain_mgr).unwrap();
    http::start(&curtain_mgr).unwrap();
}
