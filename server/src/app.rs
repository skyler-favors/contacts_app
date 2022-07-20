use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use crate::*;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .mount("/", FileServer::from("dist"))
    })
}
