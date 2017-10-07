#[macro_use]
extern crate iron;
extern crate router;

use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::{Url, status};
use router::Router;
use std::{env, thread, time};

fn main() {
    let mut router = Router::new();

    router.get("/", redirect, "index");
    router.get("/:delay", delay, "delay");
    router.post("/:delay", delay, "delay");

    let addr = format!("0.0.0.0:{}", get_port());
    println!("Listening on {}", addr);
    Iron::new(router).http(addr).unwrap();
}

fn get_port() -> String {
    match env::var("PORT") {
        Ok(val) => return val,
        Err(_e) => return String::from("80"),
    };
}

fn delay(req: &mut Request) -> IronResult<Response> {
    let millis_string = req.extensions
        .get::<Router>()
        .unwrap()
        .find("delay")
        .unwrap_or("0");

    let millis = itry!(u64::from_str_radix(millis_string, 10), (
        status::UnprocessableEntity,
        ":delay must be an integer",
    ));

    thread::sleep(time::Duration::from_millis(millis));

    let json = format!("{{\"delay\": {delay}}}", delay = millis);
    Ok(Response::with((ContentType::json().0, status::Ok, json)))
}

fn redirect(_req: &mut Request) -> IronResult<Response> {
    let url = Url::parse("https://github.com/royvandewater/rusty-go-slow-4-me").unwrap();

    Ok(Response::with(
        (status::TemporaryRedirect, Redirect(url.clone())),
    ))
}
