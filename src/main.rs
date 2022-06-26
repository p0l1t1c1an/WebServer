
use rocket::fs::{FileServer};
use std::os::unix::fs::chroot;
use std::env::set_current_dir;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    chroot("/home/p0l1t1c1an/Programs/WebServer/p0l1t1c1an.github.io")?;
    set_current_dir("/")?;
    let _rocket = rocket::build()
        .mount("/", FileServer::from("/"))
        .launch().await?; 
    Ok(())
}


