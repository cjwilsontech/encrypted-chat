use actix_web::{HttpServer, App, web, Responder, HttpRequest, HttpResponse, Error};
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
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello World!"
}

async fn client_session_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(
        WsClientSession {
            id: 0,
        },
        &req,
        stream,
    )
}

const PORT: u16 = 8000;
