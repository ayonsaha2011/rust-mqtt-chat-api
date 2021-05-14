use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::messages::{ Message, AddMessage, DeleteMessage, ResMessage},
};
use actix_web::{
    http::{
        StatusCode,
        header::HeaderValue,
    },
    web,
};

pub async fn find_by_room_id(room_id: String, pool: &Pool) -> Result<Vec<ResMessage>, ServiceError> {
    match Message::find_by_room_id(room_id, &pool.clone()).await {
        Ok(messages) => Ok(messages),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}
pub async fn add_msg(msg: AddMessage, pool: &Pool) -> Result<String, ServiceError> {
    match Message::add_new_msg(&pool.clone(), msg).await {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}
pub async fn delete_msg(msg: DeleteMessage, pool: &Pool) -> Result<String, ServiceError> {
    match Message::delete_msg(&pool.clone(), msg).await {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}