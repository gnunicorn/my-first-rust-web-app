#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, QueryString};
use std::str::FromStr;

fn main() {
    let mut server = Nickel::new();
    server.get("/convert/meter2feet", middleware! { |request|
        if let Some(meter) = request.query().get("meter") {
            match f32::from_str(meter) {
                Ok(val) => format!("{:?}", val * 3.2808),
                Err(_) => format!("Couldn't parse 'meter'")
            }
        } else {
            format!("You have to supply 'meter' parameter")
        }
    });
    server.listen("127.0.0.1:6767");
}
