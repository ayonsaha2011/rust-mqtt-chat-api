// use crate::models::user::LoginInfoDTO;
use jsonwebtoken::Header;
use crate::models::user::LoginInfoDTO;

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
static TEN_WEEK: i64 = ONE_WEEK * 10;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String
}

impl UserToken {
   pub fn generate_token(login_user: LoginInfoDTO) -> String {
       let login_user_string = toml::to_string(&login_user).unwrap();
        let now = time::get_time().sec;
        let payload = UserToken {
            iat: now,
            exp: now + TEN_WEEK ,
            user: login_user_string,
        };

        jsonwebtoken::encode(&Header::default(), &payload, &KEY).unwrap()
   }
}