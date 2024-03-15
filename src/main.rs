use actix_files::{Files, NamedFile};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    middleware, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use log::info;

#[get("/")]
async fn index() -> impl Responder {
    let path = "./assets/index.html";
    NamedFile::open_async(path).await
}

#[get("/admin")]
async fn admin(req: HttpRequest) -> impl Responder {
    if !req.headers().contains_key("hx-request") {
        let path = "./assets/404.jpg";
        return NamedFile::open_async(path)
            .await
            .unwrap()
            .into_response(&req)
            .customize()
            .with_status(StatusCode::NOT_FOUND);
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<video src='/rickroll.mp4' autoplay loop style='width: 100%; height: 100%'></video>")
        .customize()
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::formatted_builder()
        .parse_filters(&dotenvy::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    let port = dotenvy::var("PORT").expect("PORT must be set");
    info!("Server is running at http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(admin)
            .service(Files::new("/", "./assets").prefer_utf8(true))
            .wrap(middleware::Logger::new("%a | %r [%s] | Time: [%T]"))
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
