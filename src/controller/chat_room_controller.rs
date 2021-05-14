use crate::{constants, models::{
    chat_room::{CreateChatRoom, ADDChatRoomUser, DeleteRoom},
    response::ResponseBody,
}, services::chat_rooms_service, AppState};
use actix_web::{web, Error, HttpResponse};
use std::sync::Mutex;
use r2d2_redis::redis::Commands;
use crate::models::chat_room::{RedisRoom, RoomUser};
use std::collections::HashMap;

// POST api/chat-rooms
pub async fn create_room(chat_room: web::Json<CreateChatRoom>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let redis_pool = data.lock().unwrap().redis_db.clone();
    match chat_rooms_service::create_room(chat_room.0, &pool, &redis_pool).await {
        Ok(room_id) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, room_id))),
        Err(err) => Ok(err.response()),
    }
}


// POST api/chat-rooms/add-user
pub async fn add_room_user(chat_room: web::Json<ADDChatRoomUser>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let redis_pool = data.lock().unwrap().redis_db.clone();
    match chat_rooms_service::add_room_user(chat_room.0, &pool, &redis_pool).await {
        Ok(room_id) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, room_id))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/chat-rooms/list/{app_id}
pub async fn find_by_app_id(app_id: web::Path<String>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let id = app_id.into_inner().parse::<i64>().unwrap();
    match chat_rooms_service::find_by_app_id(id, &pool).await {
        Ok(rooms_list) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, rooms_list))),
        Err(err) => Ok(err.response()),
    }
}


// GET api/chat-rooms/list/{app_id}/{user_id}
pub async fn find_by_user_id(info: web::Path<(i64, String)>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let app_id = info.0.clone();
    let user_id = info.1.clone();
    let redis_pool = data.lock().unwrap().redis_db.clone();
    match chat_rooms_service::find_by_app_id_and_user_id(app_id, user_id, &pool, &redis_pool).await {
        Ok(rooms_list) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, rooms_list))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/chat-rooms/list/{app_id}/{user_id}/{room_type}
pub async fn find_by_room_type(info: web::Path<(i64, String, i8)>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let redis_pool = data.lock().unwrap().redis_db.clone();
    let app_id = info.0.clone();
    let user_id = info.1.clone();
    let room_type = info.2.clone();
    match chat_rooms_service::find_by_app_id_and_user_id_and_type(app_id, user_id, room_type, &pool, &redis_pool).await {
        Ok(rooms_list) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, rooms_list))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/chat-rooms/unread-count/{app_id}/{room_id}/{user_id}
pub async fn update_unread_count(info: web::Path<(i64, String, String)>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let mut redis_conn = data.lock().unwrap().redis_db.get().unwrap();
    let app_id = info.0.clone();
    let room_id = info.1.clone();
    let user_id = info.2.clone();
    let room_key = format!("room_{:?}_{}", &app_id, &room_id);
    let room: String = redis_conn.get(room_key.clone()).unwrap_or("ROOM_NOT_FOUND".to_string());
    if room != "ROOM_NOT_FOUND".to_string() {
        match serde_json::from_str::<RedisRoom>(&room) {
            Ok(roomData) => {
                let mut room_users: Vec<RoomUser> = vec![];
                for user in &roomData.users {
                    let mut r_user = user.clone();
                    if user.user_id == user_id {
                        r_user.unread_msg = 0;
                        room_users.push(r_user);
                    } else {
                        room_users.push(r_user)
                    }
                }
                let r_room = RedisRoom {
                    last_msg: roomData.last_msg,
                    send_at: roomData.send_at,
                    app_id: roomData.app_id,
                    users: room_users
                };
                let room_str = serde_json::to_string(&r_room).unwrap_or("error".to_string());
                if room_str != "error".to_string() {
                    redis_conn.set::<String, String, String>(room_key, room_str);
                }
            },
            Err(msg) => {
                println!("serde_json::from_str::<Value> Error {:?}", msg);
            }
        }
    }
    Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, room)))

}

// DELETE api/chat-rooms
pub async fn delete(room: web::Json<DeleteRoom>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    match chat_rooms_service::delete_room(room.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}