use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use session::WsClientSession;

mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Server running at http://localhost:{}/", PORT);

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .route("/ws", web::get().to(client_session_route))
            .service(Files::new("/", "./frontend/public"))
            .default_service(web::to(not_found))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}

async fn index() -> impl Responder {
    NamedFile::open_async("./frontend/public/index.html")
        .await
        .unwrap()
}

async fn not_found() -> impl Responder {
    NamedFile::open_async("./frontend/public/404.html")
        .await
        .unwrap()
}

async fn client_session_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WsClientSession::new(), &req, stream)
}

const PORT: u16 = 8000;
