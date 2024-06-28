use db::entities::member;
use db::{DbConn, DbErr, EntityTrait, Member, Set};

pub async fn list(conn: &DbConn) -> Result<Vec<member::Model>, DbErr> {
    Member::find().all(conn).await
}

pub async fn get(conn: &DbConn, id: i32) -> Result<Option<member::Model>, DbErr> {
    Member::find_by_id(id).one(conn).await
}

pub async fn create(conn: &DbConn, name: &str) -> Result<i32, DbErr> {
    let member = member::ActiveModel {
        name: Set(name.to_owned()),
        ..Default::default()
    };

    let insert_res = Member::insert(member).exec(conn).await?;
    Ok(insert_res.last_insert_id)
}
