#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

extern crate r2d2;
extern crate actix_rt;
extern crate argonautica;
extern crate env_logger;
extern crate serde;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate derive_more;
extern crate jsonwebtoken;
extern crate uuid;
extern crate time;
extern crate toml;
extern crate r2d2_redis;

mod controller;
mod config;
mod constants;
mod error;
mod middleware;
mod models;
mod services;
mod utils;

use actix_web::{web, HttpServer, App};
use std::{io, env, thread};
use std::sync::{Arc, Mutex};
use futures::executor;
use r2d2_redis::{r2d2 as r_r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;
pub struct AppState {
    db: Arc<config::db::Connection>,
    redis_db: r_r2d2::Pool<RedisConnectionManager>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info,info,warn");

    env_logger::init();
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    println!("db_url = {:?}", db_url);

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let r_pool = r_r2d2::Pool::builder()
        .build(manager)
        .unwrap();

    let kafka_brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS not found.");
    let kafka_group_id = env::var("KAFKA_GROUP_ID").expect("KAFKA_GROUP_ID not found.");
    let kafka_topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC not found.");

    let pool = config::db::connect_db(&db_url);
    let kafka_db_pool = pool.clone();
    thread::spawn( move || {
        println!("this is kafka thread");
        executor::block_on(async move {
            utils::kafka_consumer::start(
                kafka_brokers,
                kafka_group_id,
                kafka_topic,
                kafka_db_pool
            ).await;
        });
    });

    let data = web::Data::new(Mutex::new(AppState {
        db: pool.clone(),
        redis_db: r_pool.clone()
        
    }));
    
    let sys = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::authen_middleware::Authentication)
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run();

    println!("Server is started at {}", &app_url);
    sys.await
}