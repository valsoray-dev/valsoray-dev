use actix_files::Files;
use actix_web::{
    get,
    http::header::{ContentType, CONTENT_LOCATION},
    middleware,
    web::Html,
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use log::info;

#[get("/")]
async fn index() -> impl Responder {
    Html::new(include_str!("../assets/index.html"))
}

#[get("/blog")]
async fn rickroll(req: HttpRequest) -> impl Responder {
    // if HTMX header not present, return 404 error
    if !req.headers().contains_key("hx-request") {
        return HttpResponse::NotFound()
            .content_type(ContentType::jpeg())
            .insert_header((CONTENT_LOCATION, "https://http.cat/404.jpg"))
            .body(&include_bytes!("../assets/404.jpg")[..]);
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<video src='/rickroll.mp4' autoplay loop style='width: 100%; height: 100%'></video>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::formatted_builder()
        .parse_filters(&dotenvy::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    let port = dotenvy::var("PORT").unwrap_or("8080".to_string());
    info!("Server is running at http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new(
                "%r [%s] | Time: [%T] | \"%{User-Agent}i\"",
            ))
            .service(index)
            .service(rickroll)
            .service(Files::new("/", "./assets").prefer_utf8(true))
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
