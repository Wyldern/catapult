#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;

use std::collections;
use rocket::response;
use rocket::response::content;
use rocket::fairing::AdHoc;
use rocket::State;
use rocket::http::uri::Uri;

static INDEX_HTML: &str = include_str!("index.html");

struct Routes(collections::HashMap<String, String>);
struct ParameterizedRoutes(collections::HashMap<String, String>);

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S.%3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // General logging
        .level(if cfg!(debug_assertions) { log::LevelFilter::Info } else { log::LevelFilter::Warn })
        .level_for("catapult", if cfg!(debug_assertions) { log::LevelFilter::Debug } else { log::LevelFilter::Info })
        // Rocket-specific launch logs (includes important stuff like port/address)
        .level_for("launch", log::LevelFilter::Info)
        .level_for("launch_", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[get("/query?<search>")]
#[allow(clippy::needless_pass_by_value)] // This is required for Rocket because of FromParam
fn search(search: String, routes: State<Routes>, param_routes: State<ParameterizedRoutes>) -> Result<response::Redirect, response::status::NotFound<&'static str>> {
    let search_parts: Vec<&str> = search.splitn(2, ' ').collect();
    debug!("Search parts: {:?}", search_parts);
    let search_route = search_parts.get(0).ok_or(response::status::NotFound("Unparseable route"))?;
    let route = match search_parts.get(1) {
        None => routes.0.get(*search_route).ok_or(response::status::NotFound("No route"))?.clone(),
        Some(additional) => param_routes.0.get(*search_route).ok_or(response::status::NotFound("No route"))?.replace("%s", &Uri::percent_encode(*additional)),
    };
    debug!("REDIRECT: {:?}", route);
    Ok(response::Redirect::temporary(route))
}

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(INDEX_HTML)
}

fn parse_route_table(rocket: &rocket::Rocket, key: &str) -> collections::HashMap<String, String> {
    rocket.config()
        .get_table(key).unwrap_or_else(|_| panic!(format!("Route config key '{}' required.", key)))
        .iter()
        .map(|(k, v)| { (k.clone(), String::from(v.as_str().unwrap())) })
        .collect()
}

fn main() {
    setup_logger().expect("unable to configure logging");
    rocket::ignite()
        .mount("/", routes![index, search])
        .attach(AdHoc::on_attach("Route Config", |rocket| {
            let routes = parse_route_table(&rocket, "routes");
            debug!("Routes: {:?}", routes);
            Ok(rocket.manage(Routes(routes)))
        }))
        .attach(AdHoc::on_attach("Parameterized Route Config", |rocket| {
            let routes = parse_route_table(&rocket, "parameterized_routes");
            debug!("Parameterized Routes: {:?}", routes);
            Ok(rocket.manage(ParameterizedRoutes(routes)))
        }))
        .launch();
}
