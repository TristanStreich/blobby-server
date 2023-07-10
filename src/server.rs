use actix_web::{
    get,
    http::{
        header::ContentType,
        StatusCode,
    },
    middleware::Logger,
    post,
    web::{
        Data as AppData,
        Json,
    },
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result as ActixResult,
};
use serde_json::Value as JsonValue;
use std::io::Result as IOResult;

use crate::{
    clients::GptClient,
    ENV,
};

pub async fn start_server() -> IOResult<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(AppData::new(GptClient::new()))
            .wrap(Logger::default())
            .service(hello)
            .service(landing_page)
            .service(icon)
            .service(gpt)
    })
    .bind(("0.0.0.0", ENV.port0))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello"
}

#[get("/")]
async fn landing_page() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(std::fs::read_to_string("res/landingpage.html")?))
}

#[get("/favicon.ico")]
async fn icon() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(std::fs::read("res/blob.ico")?))
}

#[post("/gpt")]
async fn gpt(request: Json<JsonValue>, client: AppData<GptClient>) -> ActixResult<Json<JsonValue>> {
    Ok(Json(client.chat(request.0).await?))
}
