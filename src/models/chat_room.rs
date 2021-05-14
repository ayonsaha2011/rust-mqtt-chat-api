use crate::{
    config::db::Connection,
    constants,
    models::common::{DbQuery},
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
use std::thread;
use std::collections::HashMap;
use crate::config::db::RedisPool;
use r2d2_redis::redis::Commands;

// table name
pub const TABLE_NAME: &str = "chat_rooms";

#[derive(Clone, Debug, TryFromRow, Serialize, Deserialize)]
pub struct ChatRoom {
    pub app_id: i64,
    pub room_id: String,
    pub room_name: String,
    pub room_user_id: String,
    pub app_user_id: String,
    pub room_owner: String,
    pub banner: Option<String>,
    pub about: Option<String>,
    pub unread_msg: Option<i32>,
    pub last_msg: Option<HashMap<String, String>>,
    pub room_type: i8, // 1 = privet, 2 = group
    pub status: i8,
    pub is_private: bool,
    pub updated_at: i64,
    pub created_at: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResChatRoom {
    pub app_id: i64,
    pub room_id: String,
    pub room_name: String,
    pub room_user_id: String,
    pub app_user_id: String,
    pub room_owner: String,
    pub banner: Option<String>,
    pub about: Option<String>,
    pub unread_msg: Option<i32>,
    pub last_msg: Option<HashMap<String, String>>,
    pub users: Option<Vec<RoomUser>>,
    pub room_type: i8, // 1 = privet, 2 = group
    pub status: i8,
    pub is_private: bool,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateChatRoom {
    pub app_id: i64,
    pub room_name: String,
    pub room_owner: String,
    pub users: Vec<User>,
    pub banner: Option<String>,
    pub about: Option<String>,
    pub room_type: i8,
    pub is_private: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ADDChatRoomUser {
    pub room_id: String,
    pub users: Vec<User>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LastMessage {
    pub msg_owner: String,
    pub content: String
}
#[derive(Clone, Serialize, Deserialize)]
pub struct RoomID {
    room_id: String
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteRoom {
    pub app_id: i64,
    pub room_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomUser {
    pub user_id: String,
    pub unread_msg: i32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct RedisRoom {
    pub last_msg: String,
    pub send_at: i64,
    pub app_id: i64,
    pub users: Vec<RoomUser>,
}

impl CreateChatRoom {
    fn into_query_values(self, room_id: String, user_id: String) -> QueryValues {
        let room_user_id = format!("{}_{}", room_id, user_id);
        let app_user_id = format!("{:?}_{}", self.app_id.clone(), user_id);

        if self.room_type == 1 {
            let room_name = if self.users[0].user_id == user_id { self.users[1].user_name.clone() } else
            { self.users[0].user_name.clone() };
            query_values!("app_id" => self.app_id, "room_id" => room_id, "app_user_id" => app_user_id,
            "room_name" => room_name, "room_user_id" => room_user_id, "room_owner" => self
            .room_owner, "banner"=> self.banner, "about"=> self.about, "room_type"=> self.room_type, "is_private"=> self.is_private)
        } else {
            query_values!("app_id" => self.app_id, "room_id" => room_id, "app_user_id" => app_user_id,
            "room_name" => self.room_name, "room_user_id" => room_user_id, "room_owner" => self
            .room_owner, "banner"=> self.banner, "about"=> self.about, "room_type"=> self.room_type, "is_private"=> self.is_private)
        }
    }
}

impl ChatRoom {
    pub async fn find_by_app_id(app_id: i64, conn: &Connection) -> Result<Vec<ChatRoom>, String> {
        let rows = DbQuery::get_rows(&conn, TABLE_NAME, "*", "app_id=?", query_values!(app_id)).await.expect("get user");
        let mut rooms: Vec<ChatRoom> = vec![];
        for row in rows {
            let new_row: ChatRoom = ChatRoom::try_from_row(row).expect("into RowStruct");
            rooms.push(new_row)
        }
        Ok(rooms)
    }
    pub async fn find_by_app_id_and_user_id(app_id: i64, user_id: String, conn: &Connection, redis_pool: &RedisPool) -> Result<Vec<ResChatRoom>, String> {
        let mut redis_conn = redis_pool.get().unwrap();
        let app_user_id = format!("{:?}_{}", app_id.clone(), user_id);
        let rows = DbQuery::get_rows(&conn, TABLE_NAME, "*", "app_id=? AND app_user_id=?", query_values!("app_id" => app_id, "app_user_id" => app_user_id)).await.expect("get user");
        let mut rooms: Vec<ResChatRoom> = vec![];
        for row in rows {
            let new_row: ChatRoom = ChatRoom::try_from_row(row).expect("into RowStruct");
            let mut res_chat_room = ResChatRoom {
                app_id: new_row.app_id,
                room_id: new_row.room_id,
                room_name: new_row.room_name,
                room_user_id: new_row.room_user_id,
                app_user_id: new_row.app_user_id,
                room_owner: new_row.room_owner,
                banner: new_row.banner,
                about: new_row.about,
                unread_msg: None,
                last_msg: None,
                users: None,
                room_type: new_row.room_type,
                status: new_row.status,
                is_private: new_row.is_private,
                updated_at: new_row.updated_at,
                created_at: new_row.created_at
            };
            let room_key = format!("room_{:?}_{}", &res_chat_room.app_id, &res_chat_room.room_id);
            let room: String = redis_conn.get(room_key).unwrap_or("ROOM_NOT_FOUND".to_string());
            if room != "ROOM_NOT_FOUND".to_string() {
                match serde_json::from_str::<RedisRoom>(&room) {
                    Ok(roomData) => {
                        let mut last_msg:HashMap<String, String> = HashMap::new();
                        last_msg.insert("msg".to_string(), roomData.last_msg);
                        last_msg.insert("send_at".to_string(), roomData.send_at.to_string());
                        res_chat_room.last_msg = Some(last_msg);
                        for user in &roomData.users {
                            if user.user_id == user_id {
                                res_chat_room.unread_msg = Some(user.unread_msg)
                            }
                        }
                        res_chat_room.users = Some(roomData.users);
                    },
                    Err(msg) => {
                        println!("serde_json::from_str::<Value> Error {:?}", msg);
                    }
                }
            }
            rooms.push(res_chat_room)
        }
        Ok(rooms)
    }
    pub async fn find_by_app_id_and_user_id_and_type(app_id: i64, user_id: String, room_type: i8, conn: &Connection, redis_pool: &RedisPool) -> Result<Vec<ResChatRoom>, String> {
        let mut redis_conn = redis_pool.get().unwrap();
        let app_user_id = format!("{:?}_{}", app_id.clone(), &user_id);
        let rows = DbQuery::get_rows(&conn, TABLE_NAME, "*", "app_id=? AND app_user_id=? AND room_type=? ALLOW FILTERING", query_values!("app_id" => app_id, "app_user_id" => app_user_id, "room_type" => room_type)).await.expect("get user");
        let mut rooms: Vec<ResChatRoom> = vec![];
        for row in rows {
            let new_row: ChatRoom = ChatRoom::try_from_row(row).expect("into RowStruct");
            let mut res_chat_room = ResChatRoom {
                app_id: new_row.app_id,
                room_id: new_row.room_id,
                room_name: new_row.room_name,
                room_user_id: new_row.room_user_id,
                app_user_id: new_row.app_user_id,
                room_owner: new_row.room_owner,
                banner: new_row.banner,
                about: new_row.about,
                unread_msg: None,
                last_msg: None,
                users: None,
                room_type: new_row.room_type,
                status: new_row.status,
                is_private: new_row.is_private,
                updated_at: new_row.updated_at,
                created_at: new_row.created_at
            };
            let room_key = format!("room_{:?}_{}", &res_chat_room.app_id, &res_chat_room.room_id);
            let room: String = redis_conn.get(room_key).unwrap_or("ROOM_NOT_FOUND".to_string());
            if room != "ROOM_NOT_FOUND".to_string() {
                match serde_json::from_str::<RedisRoom>(&room) {
                    Ok(roomData) => {
                        let mut last_msg:HashMap<String, String> = HashMap::new();
                        last_msg.insert("msg".to_string(), roomData.last_msg);
                        last_msg.insert("send_at".to_string(), roomData.send_at.to_string());
                        res_chat_room.last_msg = Some(last_msg);
                        for user in &roomData.users {
                            if user.user_id == user_id {
                                res_chat_room.unread_msg = Some(user.unread_msg)
                            }
                        }
                        res_chat_room.users = Some(roomData.users);
                    },
                    Err(msg) => {
                        println!("serde_json::from_str::<Value> Error {:?}", msg);
                    }
                }
            }

            rooms.push(res_chat_room)
        }
        Ok(rooms)
    }

    pub async fn create_room(conn: &Connection, chat_room: CreateChatRoom, redis_pool: &RedisPool) -> Result<RoomID, String> {
        let mut redis_conn = redis_pool.get().unwrap();
        let mut room_users: Vec<RoomUser> = vec![];
        let room_id = if chat_room.room_type == 1 {
            let mut users = [chat_room.users[0].user_id.clone(), chat_room.users[1].user_id.clone()];
            users.sort();
            format!("{:?}_{}_{}", chat_room.app_id, users[0], users[1])
        } else {
            Uuid::new_v4().to_string()
        };
        for user in &chat_room.users {
            let tmp_chat_room = chat_room.clone();
            let values = tmp_chat_room.into_query_values(room_id.clone(), user.user_id.clone());
            let thread_session = conn.clone();
            let room_user = RoomUser {
                user_id: user.user_id.clone(),
                unread_msg: 0
            };
            room_users.push(room_user);
            DbQuery::insert(&thread_session, TABLE_NAME, "app_id, room_id, app_user_id, room_name, room_owner, room_user_id, banner, about, room_type, is_private, status, updated_at, created_at", "?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, toTimestamp(now()), toTimestamp(now())", values).await.expect("thread error");
        }
        let room_key = format!("room_{:?}_{}", &chat_room.app_id, &room_id);
        let r_room = RedisRoom {
            last_msg: "".to_string(),
            send_at: 0,
            app_id: *&chat_room.app_id,
            users: room_users
        };
        let room_str = serde_json::to_string(&r_room).unwrap_or("error".to_string());
        // redis_conn.set(room_key, room_str).unwrap();
        redis_conn.set::<String, String, String>(room_key, room_str);
        Ok(RoomID{room_id})
    }

    pub async fn add_room_user(conn: &Connection, chat_room_user: ADDChatRoomUser, redis_pool: &RedisPool) -> Result<RoomID, String> {
        let mut redis_conn = redis_pool.get().unwrap();
        let mut room_new_users: Vec<RoomUser> = vec![];
        let room_id = chat_room_user.room_id.clone();
        let row = DbQuery::get_row(&conn, TABLE_NAME, "*", "room_id=?", query_values!(chat_room_user.room_id)).await.expect("get user");
        match ChatRoom::try_from_row(row) {
            Ok(chat_room) => {
                let add_user: CreateChatRoom = CreateChatRoom {
                    app_id: chat_room.app_id,
                    room_name: chat_room.room_name,
                    room_owner: chat_room.room_owner,
                    users: chat_room_user.users,
                    banner: chat_room.banner,
                    about: chat_room.about,
                    room_type: chat_room.room_type,
                    is_private: chat_room.is_private,
                };
                for user in &add_user.users {
                    let tmp_chat_room = add_user.clone();
                    let values = tmp_chat_room.into_query_values(room_id.clone(), user.user_id.clone());
                    let thread_session = conn.clone();
                    let room_user = RoomUser {
                        user_id: user.user_id.clone(),
                        unread_msg: 0
                    };
                    room_new_users.push(room_user);
                    DbQuery::insert(&thread_session, TABLE_NAME, "app_id, room_id, app_user_id, room_name, room_owner, room_user_id, banner, about, room_type, is_private, unread_msg, status, updated_at, created_at", "?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 1, toTimestamp(now()), toTimestamp(now())", values).await.expect("thread error");
                }
                let room_key = format!("room_{:?}_{}", &chat_room.app_id, &room_id);
                let room: String = redis_conn.get(room_key.clone()).unwrap_or("ROOM_NOT_FOUND".to_string());
                if room != "ROOM_NOT_FOUND".to_string() {
                    match serde_json::from_str::<RedisRoom>(&room) {
                        Ok(roomData) => {
                            for user in roomData.users {
                                room_new_users.push(user);
                            }
                            let r_room = RedisRoom {
                                last_msg: roomData.last_msg,
                                send_at: roomData.send_at,
                                app_id: roomData.app_id,
                                users: room_new_users
                            };
                            let room_str = serde_json::to_string(&r_room).unwrap_or("error".to_string());
                            redis_conn.set::<String, String, String>(room_key, room_str);
                        },
                        Err(msg) => {
                            println!("serde_json::from_str::<Value> Error {:?}", msg);
                        }
                    }
                }
                return Ok(RoomID{room_id})
            },
            Err(ref err) => {
                println!("can't insert, error - {:?}", &err);
                return  Err(constants::CHAT_ROOM_NOT_UPDATED.to_string())
            },
        }
        Ok(RoomID{room_id})
    }
    pub async fn delete_room(conn: &Connection, room: DeleteRoom) -> Result<String, String> {
        let rows = DbQuery::get_rows(&conn, TABLE_NAME, "*", "app_id=? AND room_id=?", query_values!("app_id" => room.app_id, "room_id" => room.room_id)).await.expect("get user");
        for row in rows {
            let new_row: ChatRoom = ChatRoom::try_from_row(row).expect("into RowStruct");
            let thread_session = conn.clone();
            DbQuery::delete(&thread_session, TABLE_NAME, "app_id=? AND room_user_id=?", query_values!("app_id" => new_row.app_id, "room_user_id" => new_row.room_user_id)).await.expect("thread error");
        }

        Ok(constants::MESSAGE_MSG_DELETED_SUCCESS.to_string())
    }
    pub async fn update_last_msg(conn: &Connection, last_msg: LastMessage, app_id: i64, room_id: String) -> Result<String, String> {
        let mut contacts = HashMap::new();
        contacts.insert("msg_owner", last_msg.msg_owner);
        contacts.insert("content", last_msg.content);
        let db_update = DbQuery::update(&conn, TABLE_NAME, "last_msg=?", "app_id=? AND room_id=?", query_values!("last_msg" => contacts, "app_id" => app_id, "room_id" => room_id)).await;
        match db_update {
            Ok(is_updated) => {
                if is_updated {
                    return  Ok(constants::CHAT_ROOM_UPDATED_SUCCESS.to_string())
                } else {
                    return  Ok(constants::CHAT_ROOM_NOT_UPDATED.to_string())
                }
            },
            Err(ref err) => {
                println!("can't insert, error - {:?}", &err);
                return  Ok(constants::CHAT_ROOM_NOT_UPDATED.to_string())
            },
        }
    }
}