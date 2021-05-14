pub mod app;
pub mod db;

pub const CASSANDRA_DB_NAME: &str = "lmd_chat_new";
pub const CASSANDRA_DB_POOL_SIZE: u32 = 10;
pub const CASSANDRA_DB_POOL_MIN_IDLE: Option<u32> = Some(2);
pub const CASSANDRA_DB_POOL_MAX_LIFETIME: u32 = 60;
pub const CASSANDRA_DB_POOL_IDLE_TIMEOUT: u32 = 60;
pub const APP_SECRET_KEY: &str = "YXlvbg==t9nGEsDxjWtJYdYeExdB6/HU0vg+rT6czv6HSjVjZng=";