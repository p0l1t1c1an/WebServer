
#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("p0l1t1c1an.github.io")))
}

