use std::sync::Mutex;

use actix_web::{
    get, http::header::ContentEncoding, middleware, web, App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn demo1() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .insert_header(ContentEncoding::Identity)
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(demo1)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn _run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(_config)
            .service(web::scope("/api").configure(_scope_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

// struct AppState {
//     app_name: String,
// }

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn _index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

// this function could be located in a different module
fn _scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module.
fn _config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
