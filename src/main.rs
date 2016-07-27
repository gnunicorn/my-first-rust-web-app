#[macro_use] extern crate nickel;


extern crate redis;

use nickel::{Nickel, HttpRouter, QueryString, StaticFilesHandler, Mount};
use redis::{Commands, Connection};

use std::collections::HashMap;
use std::str::FromStr;
use std::env;


pub fn get_redis() -> Connection {
    let url = redis::parse_redis_url(
            &env::var("REDIS_URL").unwrap_or("redis://localhost/".to_owned())).unwrap();
    redis::Client::open(url)
             .unwrap()
             .get_connection()
             .unwrap()
}

fn main() {
    let mut server = Nickel::new();
    server.get("/convert/currency/:currency/", middleware! { |request, response|
        let mut data = HashMap::new();
        let redis = get_redis();
        let currency = request.param("currency").unwrap().to_owned();

        data.insert("currency", currency.to_string());

        let rate : f32 = redis.get(currency).unwrap();
        data.insert("rate", rate.to_string());

        if let Some(source) = request.query().get("source") {
            if let Ok(val) = f32::from_str(source) {
                data.insert("result", (val * rate).to_string());
                data.insert("source", val.to_string());
            }
        }
        return response.render("templates/currency.tpl", &data);
    });

    server.get("/convert/meter2feet", middleware! { |request, response|
        let mut data = HashMap::new();

        if let Some(meter) = request.query().get("meter") {
            if let Ok(val) = f32::from_str(meter) {
                data.insert("feet", val * 3.2808);
                data.insert("meter", val);
            }
        }
        return response.render("templates/meter2feet.tpl", &data);
    });

    server.utilize(Mount::new("/assets/",
        StaticFilesHandler::new("assets/")));
    server.utilize(StaticFilesHandler::new("assets/"));
    server.listen("127.0.0.1:6767");
}
