extern crate chrono;
extern crate iron;
extern crate num;
extern crate postgres;
extern crate router;
extern crate rustc_serialize;
extern crate timer;

mod alarm;
mod curtain;
mod db;
mod http;

fn main() {
    let curtain_mgr = curtain::Manager::new();
    let alarm_srv = alarm::Service::start(&curtain_mgr).unwrap();
    http::start(&curtain_mgr, &alarm_srv).unwrap();
}
