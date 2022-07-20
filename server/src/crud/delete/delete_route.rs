use rocket::fairing::AdHoc;
use crate::{crud::shared::{Db, Result}, models::PersonEntity};
use diesel::prelude::*;

#[get("/delete/id/<id>")]
async fn delete_by_id(db: Db, id: i32) -> Result<()> {
    use crate::schema::people::dsl::*;

    let _result = db.run(move |conn| {
        diesel::delete(people.filter(person_id.eq(id))).execute(conn)
    }).await?;

    Ok(())
}
/*
#[get("/delete/name/<name>")]
async fn delete_by_name(db: Db, name: String) -> Result<()> {

    Ok(())
}
*/
#[get("/deactivate/id/<id>")]
async fn deactivate_by_id(db: Db, id: i32) -> Result<()> {
    use crate::schema::people::dsl::*;

    let _result: PersonEntity = db.run(move |conn| {
        diesel::update(people.find(id))
            .set(active.eq(false))
.get_result(conn)
    }).await?;

    Ok(())
}

#[get("/activate/id/<id>")]
async fn activate_by_id(db: Db, id: i32) -> Result<()> {
    use crate::schema::people::dsl::*;

    let _result: PersonEntity = db.run(move |conn| {
        diesel::update(people.find(id))
            .set(active.eq(true))
            .get_result(conn)
    }).await?;

    Ok(())
}

/* #[get("/deactivate/name/<name>")]
async fn deactivate_by_name(db: Db, name: String) -> Result<()> {

    Ok(())
} */

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .mount("/api", routes![delete_by_id, deactivate_by_id, activate_by_id])
    })
}
