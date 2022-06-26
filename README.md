# WebServer
Rust webserver that to replace my github.io site

I cannot believe that this serves all of my github.io site without needing to configure anything

```
use rocket::fs::{FileServer};
use privdrop::PrivDrop;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    PrivDrop::default()
    .chroot("/home/p0l1t1c1an/Programs/WebServer/p0l1t1c1an.github.io") // should be www directory
    .user("p0l1t1c1an") // should be nobody
    .apply()?;
    let _ = rocket::build()
        .mount("/", FileServer::from("/"))
        .launch().await?; 
    Ok(())
}
```

I do plan on building a bit more in depth site, but the site would be static so I think I just need to implement TLS

Rocket makes this really easy but I'm unsure of how secure it is or how TLS is implemented
