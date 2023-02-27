#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]

#[macro_use] extern crate rocket;

use std::collections;
use rocket::http::RawStr;
use rocket::serde::Deserialize;
use rocket::response::content;
use rocket::fairing::AdHoc;
use rocket::{State, response, routes, get};

static INDEX_HTML: &str = include_str!("index.html");

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Routes(collections::HashMap<String, String>);

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    routes: Routes,
    parameterized_routes: Routes,
}

#[get("/query?<search>")]
#[allow(clippy::needless_pass_by_value)] // This is required for Rocket because of FromParam
fn search(search: String, cfg: &State<Config>) -> Result<response::Redirect, response::status::NotFound<&'static str>> {
    let search_parts: Vec<&str> = search.splitn(2, ' ').collect();
    debug!("Search parts: {:?}", search_parts);
    let search_route = search_parts.get(0).ok_or(response::status::NotFound("Unparseable route"))?;
    let route = match search_parts.get(1) {
        None => cfg.routes.0.get(*search_route).ok_or(response::status::NotFound("No route"))?.clone(),
        Some(additional) => cfg.parameterized_routes.0.get(*search_route).ok_or(response::status::NotFound("No route"))?.replace("%s", RawStr::new(*additional).percent_encode().as_str()),
    };
    debug!("REDIRECT: {:?}", route);
    Ok(response::Redirect::temporary(route))
}

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(INDEX_HTML)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, search])
        .attach(AdHoc::config::<Config>())
}
