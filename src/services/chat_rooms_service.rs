use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::chat_room::{ ChatRoom, ResChatRoom, CreateChatRoom, RoomID, ADDChatRoomUser, DeleteRoom},
};
use actix_web::{
    http::{
        StatusCode,
        header::HeaderValue,
    },
    web,
};
use crate::config::db::RedisPool;

pub async fn find_by_app_id(app_id: i64, pool: &Pool) -> Result<Vec<ChatRoom>, ServiceError> {
    match ChatRoom::find_by_app_id(app_id, &pool.clone()).await {
        Ok(users) => Ok(users),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}
pub async fn find_by_app_id_and_user_id(app_id: i64, user_id: String, pool: &Pool, redis_pool: &RedisPool) -> Result<Vec<ResChatRoom>, ServiceError> {
    match ChatRoom::find_by_app_id_and_user_id(app_id, user_id, &pool.clone(), &redis_pool).await {
        Ok(rooms) => Ok(rooms),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}
pub async fn find_by_app_id_and_user_id_and_type(app_id: i64, user_id: String, room_type: i8, pool: &Pool, redis_pool: &RedisPool) -> Result<Vec<ResChatRoom>, ServiceError> {
    match ChatRoom::find_by_app_id_and_user_id_and_type(app_id, user_id, room_type, &pool.clone(), &redis_pool).await {
        Ok(rooms) => Ok(rooms),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}
pub async fn create_room(room: CreateChatRoom, pool: &Pool, redis_pool: &RedisPool) -> Result<RoomID, ServiceError> {
    match ChatRoom::create_room(&pool.clone(), room, &redis_pool).await {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}
pub async fn add_room_user(room: ADDChatRoomUser, pool: &Pool, redis_pool: &RedisPool) -> Result<RoomID, ServiceError> {
    match ChatRoom::add_room_user(&pool.clone(), room, &redis_pool).await {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}
pub async fn delete_room(room: DeleteRoom, pool: &Pool) -> Result<String, ServiceError> {
    match ChatRoom::delete_room(&pool.clone(), room).await {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}