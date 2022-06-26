
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


