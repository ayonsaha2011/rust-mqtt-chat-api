use actix_web::HttpResponse;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .body("pong!".to_string())
}
