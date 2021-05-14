use crate::{constants, models::{
    app_user::{AppUserDTO},
    response::ResponseBody,
}, services::app_user_service, AppState};
use actix_web::{web, Error, HttpResponse};
use std::sync::Mutex;
use r2d2_redis::redis::Commands;
use serde_json::Value;

// GET api/app-users/{app_id}
pub async fn find_by_app_id(app_id: web::Path<String>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let id = app_id.into_inner().parse::<i64>().unwrap();
    match app_user_service::find_by_app_id(id, &pool).await {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}
// POST api/app-users
pub async fn create(user_dto: web::Json<AppUserDTO>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    let create_user_result = app_user_service::create_user(user_dto.0, &pool).await;
    match create_user_result {
        Ok(res_data) => Ok(HttpResponse::Ok().json(ResponseBody::new(&res_data.message, res_data.data))),
        // Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
// GET api/app-users/online/{app_id}
pub async fn online_users(app_id: web::Path<String>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let mut conn = data.lock().unwrap().redis_db.get().unwrap();
    let mut online_users_vec: Vec<String> = vec![];
    let key = format!("user_{}_", app_id);
    let key_parent = format!("{}*", &key);
    let keys: Vec<String> = conn.keys(key_parent).unwrap_or([].to_vec());
    if !keys.is_empty() {
        let users: Vec<String> = conn.get(keys).unwrap();
        for user in users {
            let v: Value = serde_json::from_str(&user)?;
            println!("isOnline = {:?} , id = {}", v["isOnline"], v["id"]);
            if let Some(isOnline) = v["isOnline"].as_bool() {
                if isOnline == true {
                    if let Some(id) = v["id"].as_str() {
                        online_users_vec.push(id.to_string());
                    }
                }
            }
        };
    }
    Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, online_users_vec)))
}