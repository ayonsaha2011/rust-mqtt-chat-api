use crate::{
    config::db::Connection,
    constants,
    models::{
        common::{DbQuery},
        chat_room::{LastMessage, ChatRoom}
    },
};
use cdrs::{
    query::*,
    frame::{
        IntoBytes,
    },
    types::{
        AsRustType, AsRust, IntoRustByName, ByName, IntoRustByIndex, ByIndex,
        from_cdrs::FromCDRSByName,
        prelude::*,
        rows::Row,
    }
};
use std::result::Result;
use uuid::Uuid;

// table name
pub const TABLE_NAME: &str = "chat_room_messages";

#[derive(Clone, Debug, TryFromRow, Serialize, Deserialize)]
pub struct Message {
    pub app_id: i64,
    pub room_id: String,
    pub msg_owner: String,
    pub owner_name:  Option<String>,
    pub msg_id: Uuid,
    pub reply_on_id: Option<Uuid>,
    pub content: String,
    pub url: Option<String>,
    pub read_by_users: Option<Vec<String>>,
    pub message_type: i8,  // 1 = text , 2 = image ..
    pub system_message: bool,
    pub status: i8, // 1= active, 2 = deleted
    pub send_at: i64,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Clone, Debug, TryFromRow, Serialize, Deserialize)]
pub struct ResMessage {
    pub msg_owner: String,
    pub owner_name:  Option<String>,
    pub msg_id: Uuid,
    pub reply_on_id: Option<Uuid>,
    pub content: String,
    pub url: Option<String>,
    pub message_type: i8,  // 1 = text , 2 = image ..
    pub send_at: i64,
    pub created_at: i64,
}
#[derive(Debug, Serialize, Deserialize, TryFromRow)]
pub struct AddMessage {
    pub app_id: i64,
    pub room_id: String,
    pub msg_owner: String,
    pub owner_name:  Option<String>,
    pub msg_id: Option<Uuid>,
    pub reply_on_id: Option<Uuid>,
    pub content: String,
    pub url: Option<String>,
    pub message_type: Option<i8>,
    pub send_at: i64
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMessage {
    pub room_id: String,
    pub send_at: i64
}
impl AddMessage {
    fn into_query_values(self) -> QueryValues {
        let message_type = self.message_type.unwrap_or(1);
        let msg_id = self.msg_id.unwrap_or(Uuid::new_v4());
        println!("msg_id === {:?}", &msg_id);
        query_values!("app_id" => self.app_id, "room_id" => self.room_id, "msg_owner" => self
        .msg_owner, "content" => self.content, "url" => self.url, "message_type" => message_type, "msg_id" => msg_id.to_string(), "reply_on_id" => self.reply_on_id, "send_at" => self.send_at, "owner_name" => self.owner_name)
    }
}

impl Message {
    pub async fn find_by_room_id(room_id: String, conn: &Connection) ->  Result<Vec<ResMessage>, String> {
        let rows = DbQuery::get_rows(&conn, TABLE_NAME, "msg_owner, owner_name, msg_id, reply_on_id, content, url, message_type, send_at, created_at", "room_id=? AND status=1", query_values!(room_id)).await.expect("get user");
        let mut messages: Vec<ResMessage> = vec![];
        for row in rows {
            let new_row: ResMessage = ResMessage::try_from_row(row).expect("into RowStruct");
            messages.push(new_row)
        }
        messages.sort_by_key(|m| m.msg_id);
        messages.sort_by(|m1, m2| m1.send_at.cmp(&m2.send_at));
        Ok(messages)
    }

    pub async fn add_new_msg(conn: &Connection, msg: AddMessage) -> Result<String, String> {
        println!("add_new_msg call");
        let last_msg: LastMessage = LastMessage {
            msg_owner: msg.msg_owner.clone(),
            content: msg.content.clone()
        };
        let room_id = msg.room_id.clone();
        let app_id = msg.app_id.clone();
        let db_insert = DbQuery::insert(&conn, TABLE_NAME, "app_id, room_id, msg_owner, owner_name, msg_id, reply_on_id, content, url, message_type, system_message, status, send_at, updated_at, created_at", "?, ?, ?, ?, uuid(), ?, ?, ?, ?, false, 1, ?, toTimestamp(now()), toTimestamp(now())", msg.into_query_values()).await;
        match db_insert {
            Ok(is_inserted) => {
                if is_inserted {
                    // ChatRoom::update_last_msg(&conn, last_msg, app_id, room_id).await;
                    return  Ok(constants::MESSAGE_MSG_CREATED_SUCCESS.to_string())
                } else {
                    return  Ok(constants::MESSAGE_MSG_NOT_CREATED.to_string())
                }
            },
            Err(ref err) => {
                println!("can't insert, error - {:?}", &err);
                return  Err(constants::MESSAGE_MSG_NOT_CREATED.to_string())
            },
        }
    }
    pub async fn delete_msg(conn: &Connection, msg: DeleteMessage) -> Result<String, String> {
        let db_update = DbQuery::delete(&conn, TABLE_NAME, "room_id=? AND send_at=?", query_values!("room_id" => msg.room_id, "send_at" => msg.send_at)).await;
        match db_update {
            Ok(is_deleted) => {
                if is_deleted {
                    return  Ok(constants::MESSAGE_MSG_DELETED_SUCCESS.to_string())
                } else {
                    return  Ok(constants::MESSAGE_MSG_NOT_DELETED.to_string())
                }
            },
            Err(ref err) => {
                println!("can't insert, error - {:?}", &err);
                return  Ok(constants::MESSAGE_MSG_NOT_DELETED.to_string())
            },
        }
    }
}