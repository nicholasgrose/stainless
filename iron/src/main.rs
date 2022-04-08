use actix_cors::Cors;
use actix_web::{App, http::header, HttpServer, middleware, web::Data};
use tracing_actix_web::TracingLogger;

mod database;
mod schema;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema::new()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(TracingLogger::default())
            .service(routes::graphql)
            .service(routes::graphiql)
            .service(routes::playground)
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
// now go to http://127.0.0.1:8080/playground or graphiql and execute
//{  apiVersion,  user(id: 2){id, name}}
