use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;
mod routes;
mod lightning;

#[launch]
fn rocket() -> _ {
   rocket:: build()
     .mount("/public", FileServer::from(relative!("static")))
     .mount("/", routes![routes::index, routes::create_invoice, routes::lookup_invoice])
     .attach(Template::fairing())
}
