use rocket::serde::json::Json;
use crate::models::{User, UserEntity};
use crate::schema::users;
use rocket::fs::NamedFile;
use rocket::form::Form;
use rocket::State;

use rocket_sync_db_pools::diesel;
use diesel::prelude::*;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};

#[database("diesel")]
struct Db(diesel::PgConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

// uses db connection pool
// fast and correct
#[get("/list")]
async fn list(db: Db) -> Result<Json<Vec<UserEntity>>> {
    let users = db.run(move |conn| {
        users::table.load(conn)
    }).await?;

    Ok(Json(users))
}

// the form for creating the user
#[get("/create")]
async fn create_user() -> Option<NamedFile> {
    NamedFile::open("create_acc.html").await.ok()
}

// displays the created user
#[post("/create", data="<user>")]
async fn created_user(db: Db, user: Form<User>) -> Result<Created<String>> {
    let new_user = User {
        name: user.name.clone(),
        email: user.email.clone(),
        age: user.age,
    };

    db.run(move |conn| {
        diesel::insert_into(users::table).values(new_user).execute(conn)
    }).await?;

    let result = format!("User: {}, {} {}", user.name, user.email, user.age);

    Ok(Created::new("/create").body(result))

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/", routes![index, list, create_user, created_user])
    })
}
