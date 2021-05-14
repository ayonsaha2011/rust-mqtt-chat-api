use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::app_user::{ AppUser, AppUserDTO, CreateAppUser},
};
use actix_web::{
    http::{
        StatusCode,
        header::HeaderValue,
    },
    web,
};

pub async fn find_by_app_id(app_id: i64, pool: &Pool) -> Result<Vec<AppUser>, ServiceError> {
    match AppUser::find_by_app_id(app_id, &pool.clone()).await {
        Ok(users) => Ok(users),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}
pub async fn find_by_app_id_and_app_user_id(app_id: i64, app_user_id: String, pool: &Pool) -> Result<AppUser, ServiceError> {
    match AppUser::find_by_app_id_and_app_user_id(app_id, app_user_id, &pool.clone()).await {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub async fn create_user(user: AppUserDTO, pool: &Pool) -> Result<CreateAppUser, ServiceError> {
    match AppUser::insert_user(&pool.clone(), user).await {
        Ok(res_data) => Ok(res_data),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}