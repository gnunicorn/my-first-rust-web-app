#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, QueryString};
use nickel::status::StatusCode;
use std::str::FromStr;

fn main() {
    let mut server = Nickel::new();
    server.get("/convert/meter2feet", middleware! { |request|
        if let Some(meter) = request.query().get("meter") {
            match f32::from_str(meter) {
                Ok(val) => (StatusCode::Ok, format!("{:?}", val * 3.2808)),
                Err(_) => (StatusCode::BadRequest, format!("Couldn't parse 'meter'"))
            }
        } else {
            (StatusCode::BadRequest, format!("You have to supply 'meter' parameter"))
        }
    });
    server.listen("127.0.0.1:6767");
}
