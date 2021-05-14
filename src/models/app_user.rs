use crate::{
    config::db::Connection,
    constants,
    models::common::{DbQuery},
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
use uuid::Uuid;

// table name
pub const TABLE_NAME: &str = "app_users";

#[derive(Clone, Debug, TryFromRow, Serialize, Deserialize)]
pub struct AppUser {
    pub id: Uuid,
    pub app_id: i64,
    pub app_user_id: String,
    pub display_name: String,
    pub display_image: Option<String>,
    pub status: i8,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAppUser {
    pub message: String,
    pub data: AppUser
}

#[derive(Clone, Debug, Serialize, Deserialize, TryFromRow)]
pub struct AppUserDTO {
    pub app_id: i64,
    pub app_user_id: String,
    pub display_name: String,
    pub display_image: Option<String>,
}

impl AppUserDTO {
    fn into_query_values(self) -> QueryValues {
        let app_user_id = format!("{:?}_{}", &self.app_id, self.app_user_id);
        query_values!("app_id" => self.app_id, "app_user_id" => app_user_id, "display_name" => self
        .display_name, "display_image" => self.display_image)
    }
}

impl AppUser {
    pub async fn find_by_app_id(app_id: i64, conn: &Connection) ->  Result<Vec<AppUser>, String> {
        let rows = DbQuery::get_rows(&conn, TABLE_NAME, "*", "app_id=?", query_values!(app_id)).await.expect("get user");
        let mut users: Vec<AppUser> = vec![];
        for row in rows {
            let new_row: AppUser = AppUser::try_from_row(row).expect("into RowStruct");
            users.push(new_row)
        }
        Ok(users)
    }
    pub async fn find_by_app_id_and_app_user_id(app_id: i64, app_user_id: String, conn: &Connection) ->  Result<AppUser, String> {
        let row = DbQuery::get_row(&conn, TABLE_NAME, "*", "app_id=? AND app_user_id=?", query_values!("app_id" => app_id, "app_user_id" => app_user_id)).await.expect("get user");
        let app_user: AppUser = AppUser::try_from_row(row).expect("into RowStruct");
        Ok(app_user)
    }
    pub async fn insert_user(conn: &Connection, app_user: AppUserDTO) -> Result<CreateAppUser, String> {
        let app_user_query_values = app_user.clone().into_query_values();
        let db_insert = DbQuery::insert(&conn, TABLE_NAME, "app_id, app_user_id, id, display_name, display_image, status, updated_at, created_at", "?, ?, uuid(), ?, ?, 1, toTimestamp(now()), toTimestamp(now())", app_user_query_values).await;
        let app_user_id = format!("{:?}_{}", &app_user.app_id, app_user.app_user_id);
        let app_id = app_user.app_id;
        match db_insert {
            Ok(is_inserted) => {
                let mut msg: String = "".to_string();
                if is_inserted {
                    msg = constants::MESSAGE_APP_USER_CREATED_SUCCESS.to_string();
                } else {
                    msg = constants::MESSAGE_APP_USER_ALREADY_EXISTS.to_string();
                }
                
                match Self::find_by_app_id_and_app_user_id(app_id, app_user_id, &conn).await {
                    Ok(user) => {
                        let res_data: CreateAppUser = CreateAppUser {
                            message: msg,
                            data: user
                        };
                        return Ok(res_data)
                    },
                    Err(_) => Err(constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string()),
                }
            },
            Err(ref err) => {
                println!("can't insert, error - {:?}", &err);
                return  Err(constants::MESSAGE_APP_USER_NOT_CREATED.to_string())
            },
        }
    }
}