use actix_web::{
    get,
    http::{
        header::ContentType,
        StatusCode,
    },
    middleware::Logger,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};
use std::io::Result as IOResult;

const PORT: u16 = 3000;

pub async fn start_server() -> IOResult<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(landing_page)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello"
}

#[get("/")]
async fn landing_page() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(std::fs::read_to_string("res/landingpage.html").unwrap())
}
