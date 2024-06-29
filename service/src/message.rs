use db::entities::message;
use db::{DbConn, DbErr, EntityTrait, Set};

pub async fn list_messages(conn: &DbConn) -> Result<Vec<message::Model>, DbErr> {
    message::Entity::find().all(conn).await
}

pub async fn create_message(conn: &DbConn, content: &str, member_id: i32) -> Result<i32, DbErr> {
    let message = message::ActiveModel {
        content: Set(content.to_owned()),
        member_id: Set(member_id),
        ..Default::default()
    };

    let insert_res = message::Entity::insert(message).exec(conn).await?;
    Ok(insert_res.last_insert_id)
}
