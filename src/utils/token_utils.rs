use crate::{
    config::db::Pool,
    models::{
        user::User,
        user_token::{UserToken, KEY},
    },
};
use jsonwebtoken::{TokenData, Validation};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &KEY, &Validation::default())
}

pub fn verify_token(token_data: &TokenData<UserToken>) -> Result<String, String> {
    if true {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
