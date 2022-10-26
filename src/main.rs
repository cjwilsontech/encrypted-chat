use actix_web::{HttpServer, App, web, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Server running at http://localhost:{}/", PORT);

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello World!"
}

const PORT: u16 = 8000;
