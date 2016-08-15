extern crate postgres;
extern crate num;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

mod alarm;
mod db;
mod http;
mod curtain;

fn main() {
    let curtain_mgr = curtain::Manager::new();
    let alarm_srv = alarm::Service::start(&curtain_mgr).unwrap();
    http::start(&curtain_mgr, &alarm_srv).unwrap();
}
