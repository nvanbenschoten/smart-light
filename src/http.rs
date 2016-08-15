use iron::prelude::*;
use iron::{status, Handler, Listening};
use iron::error::HttpResult;
use router::Router;
use rustc_serialize::json;
use curtains::Manager;

pub fn start(curtain_mgr: &Manager) -> HttpResult<Listening> {
    let mut router = Router::new();

    router.get("/status", with_manager(curtain_mgr, status));
    router.post("/toggle", with_manager(curtain_mgr, toggle));

    Iron::new(router).http("0.0.0.0:8000")
}

#[derive(RustcEncodable)]
struct ToggleStatus {
    open: bool
}

fn status(_: &mut Request, curtain_mgr: &Manager) -> IronResult<Response> {
    let is_open = curtain_mgr.is_open();
    let status = ToggleStatus{ open: is_open };
    let payload = json::encode(&status).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn toggle(_: &mut Request, curtain_mgr: &Manager) -> IronResult<Response> {
    let toggle = curtain_mgr.toggle();
    let status = ToggleStatus{ open: toggle };
    let payload = json::encode(&status).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn with_manager<F1>(curtain_mgr: &Manager, f: F1) -> Box<Handler>
    where F1: Fn(&mut Request, &Manager) -> IronResult<Response> + Send + Sync + 'static {

    // Moving an immutable clone of the curtain::Manager into the closure
    // is required to create an Fn instead of an FnMut.
    let curtain_mgr_clone = curtain_mgr.clone();
    return Box::new(move |req: &mut Request| -> IronResult<Response> {
        f(req, &curtain_mgr_clone)
    });
}