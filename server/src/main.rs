#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(routes::stage())
}

//fn main() {}
