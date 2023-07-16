mod notes;

use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let uri = "mongodb://admin:admin@localhost:27017";

    let client = Client::with_uri_str(uri)
        .await
        .expect("failed to connect to MongoDB");

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "PUT"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE]);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(
                web::scope("/api/note")
                    .service(notes::routes::handle_all_notes)
                    .service(notes::routes::handle_create_note),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
