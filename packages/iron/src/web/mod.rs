use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::{header, Method},
    middleware::Compress,
    web::Data,
    App, HttpServer,
};
use tracing_actix_web::TracingLogger;

pub mod routes;
pub mod schema;
pub mod tls;

pub fn start_server(address: &str) -> crate::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema::new()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec![Method::POST, Method::GET])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Compress::default())
            .wrap(TracingLogger::default())
            .service(routes::all())
    });
    let tls_config = tls::load_tls_config()?;

    Ok(server.bind_rustls(address, tls_config)?.run())
}
