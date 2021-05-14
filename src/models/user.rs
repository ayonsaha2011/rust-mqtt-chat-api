use crate::{
    config::{db::Connection, APP_SECRET_KEY},
    models::{
        common::{DbQuery},
        user_token::{UserToken},
    },
    constants,
};
use cdrs::{
    query::*,
    frame::{
        IntoBytes,
    },
    types::{
        AsRustType, AsRust, IntoRustByName, ByName, IntoRustByIndex, ByIndex,
        from_cdrs::FromCDRSByName,
        prelude::*,
        rows::Row,
    }
};
use std::result::Result;
use argonautica::{Hasher, Verifier};
use uuid::Uuid;

// table name
pub const TABLE_NAME: &str = "users";

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub display_name: String,
    pub display_image: Option<String>,
    pub bio: Option<String>,
    pub status: i8,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, TryFromRow)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfoDTO {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub display_image:  Option<String>,
}

impl UserDTO {
    fn into_query_values(self) -> QueryValues {
        query_values!("username" => self.username, "email" => self.email, "phone" => self.phone, "password" => self.password, "display_name" => self.display_name)
    }
}
impl User {
    pub async fn signup(user: UserDTO, conn: &Connection ) -> Result<String, String> {
        let (email_count, username_count) = Self::email_or_username_exist(&user.email, &user.username, conn).await;
        println!("email_count: {:?}, username_count {:?}", email_count, username_count);
        if email_count > 0 { return Err(format!("Email ID '{}' is already registered", &user.email))}
        if username_count > 0 { return Err(format!("Username '{}' is already registered", &user.username))}
        let hashed_pwd = Self::password_hasher(&user.password);
        let user = UserDTO {
            password: hashed_pwd,
            ..user
        };
        let user_qv = user.into_query_values();
        match DbQuery::insert(&conn, TABLE_NAME,"id, username, email, phone, password, display_name, \
        status, updated_at, created_at", "uuid(), ?, ?, ?, ?, ?, 1, toTimestamp(now()), \
        toTimestamp(now())", user_qv).await {
            Ok(_) => {},
            Err(ref err) => {
                return  Err(format!("can't insert, error - {:?}", &err))
            },
        }
        Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
    }

    pub async fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        let user_to_verify = Self::find_user_by_email(&login.email, &conn).await;
        if !user_to_verify.password.is_empty()
            && Self::password_verifier(&user_to_verify.password, &login.password)
        {
            return Some(LoginInfoDTO {
                id: user_to_verify.id.to_string(),
                username: user_to_verify.username,
                email: user_to_verify.email,
                display_name: user_to_verify.display_name,
                display_image: user_to_verify.display_image
            });
        }
        None
    }


    pub async fn find_user_by_email(email: &str, conn: &Connection) -> User {
        let row = DbQuery::get_row(&conn, TABLE_NAME, "*", "email=?", query_values!(email)).await.expect("get user");
        let user: User = User::try_from_row(row).expect("into RowStruct");
        user
    }
    pub async fn logout(user_id: i32, conn: &Connection) {
        /*if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.username, "", conn);
        }*/
    }

    /*pub async fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
        true
    }

    pub async fn find_user_by_username(username: &str, conn: &Connection) -> User {
        let row = DbQuery::get_row(&conn, TABLE_NAME, "*", "username=?", query_values!(username)).await.expect("get user");
        let user: User = User::try_from_row(row).expect("into RowStruct");
        user
    }

    pub async fn find_user_by_id(id: &str, conn: &Connection) -> User {
        let row = DbQuery::get_row(&conn, TABLE_NAME, "*", "id=?", query_values!(id)).await.expect("get user");
        let user: User = User::try_from_row(row).expect("into RowStruct");
        user
    }

    pub async fn update_login_session_to_db(un: &str, login_session_str: &str, conn: &Connection) -> bool {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
        false
    }*/
    pub async fn email_or_username_exist(email: &str, username: &str, conn: &Connection) -> (i64, i64) {
        let email_count = DbQuery::get_count(&conn, TABLE_NAME, "email = ?", query_values!(email)).await.expect("get user count");
        let username_count = DbQuery::get_count(&conn, TABLE_NAME, "username = ?", query_values!(username)).await.expect("get user count");
        (email_count, username_count)
    }

    fn password_hasher(password: &str) -> String {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(password)
            .with_secret_key(APP_SECRET_KEY)
            .hash()
            .unwrap();
        hash
    }
    fn password_verifier(hash: &str, password: &str) -> bool {
        println!("hash === {}", &hash);
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(APP_SECRET_KEY)
            .verify()
            .unwrap();
        is_valid
    }
}