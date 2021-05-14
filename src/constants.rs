// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
// pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];

// app user
pub const MESSAGE_APP_USER_CREATED_SUCCESS: &str = "User created successfully";
pub const MESSAGE_APP_USER_ALREADY_EXISTS: &str = "User already exists";
pub const MESSAGE_APP_USER_NOT_CREATED: &str = "Can not created a user";

pub const MESSAGE_MSG_CREATED_SUCCESS: &str = "Message created successfully";
pub const MESSAGE_MSG_NOT_CREATED: &str = "Can not created a message";
pub const MESSAGE_MSG_DELETED_SUCCESS: &str = "Message deleted successfully";
pub const MESSAGE_MSG_NOT_DELETED: &str = "Can not deleted message";

pub const CHAT_ROOM_UPDATED_SUCCESS: &str = "Chat room updated successfully";
pub const CHAT_ROOM_NOT_UPDATED: &str = "Can not update chat room";
