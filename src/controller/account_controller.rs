use crate::{constants, models::{
    user::{UserDTO, LoginDTO},
    response::ResponseBody,
}, services::account_service, AppState};
use actix_web::{web, Error, HttpResponse};
//use futures::future::{ok, Future};
use std::sync::Mutex;

// POST api/auth/signup
pub async fn signup(user_dto: web::Json<UserDTO>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    match account_service::signup(user_dto.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}


// POST api/auth/login
pub async fn login(login_dto: web::Json<LoginDTO>, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let pool = data.lock().unwrap().db.clone();
    match account_service::login(login_dto.0, &pool).await {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}
/*
// POST api/auth/logout
pub fn logout(req: HttpRequest, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        account_service::logout(authen_header, &pool);
        ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY)))
    } else {
        ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}
*/