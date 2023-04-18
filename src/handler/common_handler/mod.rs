pub mod common{
    use actix_web::{HttpResponse, Responder,get};
    use serde_json::json;

    #[get("/health")]
    pub async fn health_check() -> impl Responder {
        const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, MySQL, and Actix Web";
        HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
    }

}

