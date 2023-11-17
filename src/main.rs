use actix_files::Files;
use actix_web::{
    get, http::header::ContentType, middleware, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use dotenv::dotenv;
use log::info;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("./static/index.html"))
}

#[get("/admin")]
async fn admin(req: HttpRequest) -> impl Responder {
    if !req.headers().contains_key("hx-request") {
        let image_data = include_bytes!("./static/404.jpg");
        return HttpResponse::NotFound()
            .content_type(ContentType::jpeg())
            .body(&image_data[..]);
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<video src='/rickroll.mp4' autoplay loop style='width: 100%; height: 100%'></video>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("PORT").expect("PORT must be set");
    info!("Server is running at http://localhost:{}", &port);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(admin)
            .service(Files::new("/", "./src/static").prefer_utf8(true))
            .wrap(middleware::Logger::new("%a | %r [%s] | Time: [%T]"))
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
