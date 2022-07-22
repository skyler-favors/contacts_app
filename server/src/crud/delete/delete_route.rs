use rocket::fairing::AdHoc;
use crate::{crud::shared::{Db, Result}, models::PersonEntity};
use diesel::prelude::*;
use diesel::dsl::not;

#[get("/delete/id/<id>")]
async fn delete_by_id(db: Db, id: i32) -> Result<()> {
    use crate::schema::people::dsl::*;

    let _result = db.run(move |conn| {
        diesel::delete(people.filter(person_id.eq(id))).execute(conn)
    }).await?;

    Ok(())
}

#[get("/delete/all")]
async fn delete_all(db: Db) -> Result<()> {
    use crate::schema::people::dsl::*;

    let _result = db.run(move |conn| {
        diesel::delete(people.filter(active.eq(false))).execute(conn)
    }).await?;

    Ok(())
}

#[get("/toggle/trash/id/<id>")]
async fn toggle_trash_by_id(db: Db, id: i32) -> Result<()> {
    use crate::schema::people::dsl::*;
    let _result: PersonEntity = db.run(move |conn| {
        diesel::update(people.find(id))
            .set(active.eq(not(active)))
            .get_result(conn)
    }).await?;

    Ok(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .mount("/api", routes![delete_by_id, toggle_trash_by_id, delete_all])
    })
}
