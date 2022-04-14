use actix_cors::Cors;
use actix_web::middleware::Compress;
use actix_web::{http::{header, Method}, web::Data, App, HttpServer};
use tracing_actix_web::TracingLogger;

mod database;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema::new()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec![Method::POST.as_str(), Method::GET.as_str()])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Compress::default())
            .wrap(TracingLogger::default())
            .service(routes::graphql)
            .service(routes::graphiql)
            .service(routes::playground)
    });

    server.bind("0.0.0.0:8080")?.run().await
}
