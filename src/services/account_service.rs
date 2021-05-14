use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::user::{User, UserDTO, LoginDTO},
    models::user_token::UserToken,
    utils::token_utils,
};
use actix_web::{
    http::{
        StatusCode,
        header::HeaderValue,
    },
    web,
};


#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub async fn signup(user: UserDTO, pool: &Pool) -> Result<String, ServiceError> {
    match User::signup(user, &pool.clone()).await {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string()))
    }
}

pub async fn login(login: LoginDTO, pool:  &Pool) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &pool.clone()).await {
        Some(logged_user) => {
            match serde_json::from_value(json!({ "token": UserToken::generate_token(logged_user), "token_type": "bearer" })) {
                Ok(token_res) => Ok(token_res),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        },
        None => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_LOGIN_FAILED.to_string()))
    }
}
