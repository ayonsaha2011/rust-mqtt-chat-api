use crate::{constants, models::{
    messages::{AddMessage, DeleteMessage},
    response::ResponseBody,
}, services::message_service, AppState};
use actix_web::{web, Error, HttpResponse};
use std::sync::Mutex;

// GET api/messages/{room_id}
pub async fn find_by_room_id(room_id: web::Path<String>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let id = room_id.into_inner().parse::<String>().unwrap();
    match message_service::find_by_room_id(id, &pool).await {
        Ok(messages) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, messages))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/messages
pub async fn add(msg: web::Json<AddMessage>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    match message_service::add_msg(msg.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/messages
pub async fn delete(msg: web::Json<DeleteMessage>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    match message_service::delete_msg(msg.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}