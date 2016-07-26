
extern crate iron;

use iron::prelude::*;
use iron::status;

#[macro_use]
extern crate router;

use router::Router;
use std::str::FromStr;

fn main() {
    fn meter2feet(req: &mut Request) -> IronResult<Response> {

        let meter = req.extensions.get::<Router>().unwrap().find("meter").unwrap();
        match f32::from_str(meter) {
                Ok(val) => Ok(Response::with((status::Ok, format!("{:?}", val * 3.2808)))),
                Err(_) => Ok(Response::with((status::BadRequest, format!("Couldn't parse 'meter'"))))
        }
    }

    println!("On 6767");
    Iron::new(router!(
        get "/convert/meter2feet/:meter" => meter2feet
    )).http("localhost:6767").unwrap();
}
