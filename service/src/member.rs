use db::entities::member;
use db::{DbConn, DbErr, EntityTrait, Set};

pub async fn list(conn: &DbConn) -> Result<Vec<member::Model>, DbErr> {
    member::Entity::find().all(conn).await
}

pub async fn get(conn: &DbConn, id: i32) -> Result<Option<member::Model>, DbErr> {
    member::Entity::find_by_id(id).one(conn).await
}

pub async fn create(conn: &DbConn, name: &str) -> Result<i32, DbErr> {
    let member = member::ActiveModel {
        name: Set(name.to_owned()),
        ..Default::default()
    };

    let insert_res = member::Entity::insert(member).exec(conn).await?;
    Ok(insert_res.last_insert_id)
}
