
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    println!("On 6767");
    Iron::new(hello_world).http("localhost:6767").unwrap();
}
