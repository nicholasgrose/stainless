use std::sync::Arc;
use axum::{Router, ServiceExt};
use axum::routing::get;
use axum_server::Server;
use axum_server::tls_rustls::{RustlsAcceptor, RustlsConfig};

use crate::database::DatabaseContext;
use crate::web::routes::{graphiql, graphql, playground};

pub mod routes;
pub mod schema;

pub async fn start_server(address: &str) -> crate::Result<Server<RustlsAcceptor>> {
    let graphql_url = format!("{}/graphql", address);
    let schema = schema::new();
    let database = DatabaseContext::new("iron_db.sqlite3")?;

    let app = Router::new()
        .route("/graphql", get(graphql).post(graphql))
        .route("/graphiql", get(|| async { graphiql(&graphql_url) }))
        .route("/playground", get(|| async { playground(&graphql_url) }))
        .with_state(Arc::new(schema))
        .with_state(Arc::new(database));
    let tls_config = RustlsConfig::from_pem_file("cert.pem", "key.pem");

    axum_server::bind_rustls(address.parse()?, tls_config)
        .serve(app.into_make_service())
}
