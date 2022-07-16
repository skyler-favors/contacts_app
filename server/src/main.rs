#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate dotenv;

pub mod helper;
pub mod models;
pub mod routes;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(routes::read::stage())
        .attach(routes::create::stage())
        //.attach(routes::delete::stage())
        //.attach(routes::update::stage())
        .attach(routes::app::stage())
}
