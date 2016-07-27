
extern crate iron;
extern crate handlebars_iron as hbs;
extern crate urlencoded;

#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;

use router::Router;
use std::str::FromStr;

use hbs::{Template, HandlebarsEngine, DirectorySource};
use urlencoded::UrlEncodedQuery;
use std::collections::HashMap;

fn meter2feet(req: &mut Request) -> IronResult<Response> {

    let mut resp = Response::new();
    let mut data = HashMap::new();

    if let Ok(ref map) = req.get_ref::<UrlEncodedQuery>() {
        if let Some(ref vals) = map.get("meter") {
            if let Some(val) = vals.first() {
                if let Ok(meter) = f32::from_str(val) {
                    data.insert("meter".to_string(), meter.to_string());
                    data.insert("feet".to_string(), (meter * 3.2808).to_string());
                }
            }
        }
    }

    resp.set_mut(Template::new("meter2feet", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
      panic!("{}", r);
    }


    let mut router = Router::new();
    router.get("/convert/meter2feet", meter2feet);

    // make chainable
    let mut chain = Chain::new(router);
    // add templates
    chain.link_after(hbse);


    println!("Server running at http://localhost:6767/");
    Iron::new(chain).http("localhost:6767").unwrap();
}
