# WebServer
Rust webserver that to replace my github.io site

I cannot believe that this serves all of my github.io site without needing to configure anything

```
#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("p0l1t1c1an.github.io")))
}

```

I do plan on building a bit more in depth site, but the site would be static so I think i just need to implement TLS
Rocket makes this really easy but I'm unsure of how secure it is or how TLS is implemented.

