#[macro_use]
extern crate rocket;

use futures::future;
use rocket::config::TlsConfig;
use rocket::Build;
use rocket::Rocket;
use rocket::State;
use std::env;
use std::fmt::Display;
use std::str::FromStr;

const DEFAULT_PORT: u16 = 9080;
const DEFAULT_SSL_PORT: u16 = 9443;
const DEFAULT_VERSION: &str = "0";
const DEFAULT_ZONE: &str = "local";
const DEFAULT_REGION: &str = "local-0";
const DEFAULT_HOSTNAME: &str = "localhost";

struct HelloWorld {
    version: String,
    zone: String,
    region: String,
    hostname: String,
    proto: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!\n"
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "{\"state\": \"READY\"}"
}

#[get("/version")]
fn version(helloworld: &State<HelloWorld>) -> String {
    format!(
        "version: {}, zone {}, region {}, instance {}, proto, {}\n",
        helloworld.version,
        helloworld.zone,
        helloworld.region,
        helloworld.hostname,
        helloworld.proto
    )
}

fn parse_env<T>(variable: &str) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    env::var(variable)
        .map_err(|error| error!("{error}: {variable}"))
        .ok()
        .and_then(|raw| {
            raw.parse::<T>()
                .map_err(|error| error!("{error}: {raw}"))
                .ok()
        })
}
// #[launch]
#[rocket::main]
async fn main() {
    // Recall that an uninspected `Error` will cause a pretty-printed panic,
    // so rest assured errors do not go undetected when using `#[launch]`.
    //    let _ = http_rocket().launch().await;
    //let http_rocket = http_rocket();
    if env::var("SSL_PORT").is_ok() {
        // let https_rocket = https_rocket();
        let _ = future::join(http_rocket().launch(), https_rocket().launch()).await;
    } else {
        let _ = http_rocket().launch().await;
    }
}

fn http_rocket() -> Rocket<Build> {
    let port: u16 = parse_env("PORT").unwrap_or(DEFAULT_PORT);

    let helloworld: HelloWorld = HelloWorld {
        version: parse_env("SERVICE_VERSION").unwrap_or(DEFAULT_VERSION.into()),
        zone: parse_env("ZONE").unwrap_or(DEFAULT_ZONE.into()),
        region: parse_env("REGION").unwrap_or(DEFAULT_REGION.into()),
        hostname: parse_env("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.into()),
        proto: "http".into(),
    };

    let figment = rocket::Config::figment().merge(("port", port));

    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/", routes![healthz])
        .mount("/", routes![version])
        .manage(helloworld)
}

fn https_rocket() -> Rocket<Build> {
    let ssl_port: u16 = parse_env("SSL_PORT").unwrap_or(DEFAULT_SSL_PORT);
    let ssl_key: String = parse_env("SSL_KEY").unwrap();
    let ssl_cert: String = parse_env("SSL_CERT").unwrap();

    let helloworld: HelloWorld = HelloWorld {
        version: parse_env("SERVICE_VERSION").unwrap_or(DEFAULT_VERSION.into()),
        zone: parse_env("ZONE").unwrap_or(DEFAULT_ZONE.into()),
        region: parse_env("REGION").unwrap_or(DEFAULT_REGION.into()),
        hostname: parse_env("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.into()),
        proto: "tls".into(),
    };

    let tls_config = TlsConfig::from_paths(ssl_cert, ssl_key);

    let figment = rocket::Config::figment()
        .merge(("port", ssl_port))
        .merge(("tls", tls_config));

    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/", routes![healthz])
        .mount("/", routes![version])
        .manage(helloworld)
}
