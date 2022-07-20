#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate dotenv;

mod models;
mod schema;
mod app;
mod crud; 

use crate::crud::shared::Db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(crud::read::stage())
        .attach(crud::create::stage())
        .attach(crud::delete::stage())
        .attach(crud::update::stage())
        .attach(app::stage())
}
