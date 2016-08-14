use iron::prelude::*;
use iron::{status, Listening};
use iron::error::HttpResult;
use router::Router;

pub fn start() -> HttpResult<Listening> {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/:query", handler);
    Iron::new(router).http("0.0.0.0:8000")
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}