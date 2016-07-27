#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, QueryString};
use std::str::FromStr;

fn main() {
    let mut server = Nickel::new();
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
    server.listen("127.0.0.1:6767");
}
