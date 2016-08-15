use iron::prelude::*;
use iron::{status, Listening};
use iron::error::HttpResult;
use router::Router;
use rustc_serialize::json;
use curtains::Manager;

pub fn start(curtain_mgr: &Manager) -> HttpResult<Listening> {
    let mut router = Router::new();

    setup_toggle(&mut router, curtain_mgr);

    Iron::new(router).http("0.0.0.0:8000")
}

#[derive(RustcEncodable)]
struct ToggleStatus {
    open: bool
}

fn setup_toggle(router: &mut Router, curtain_mgr: &Manager) {
    let curtain_mgr_clone = curtain_mgr.clone();
    router.get("/toggle", move |req: &mut Request| -> IronResult<Response> {
        let toggle = curtain_mgr_clone.toggle();
        let status = ToggleStatus{ open: toggle };
        let payload = json::encode(&status).unwrap();
        Ok(Response::with((status::Ok, payload)))
    });
}