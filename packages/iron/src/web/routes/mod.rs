use std::sync::Arc;
use axum::extract::State;
use axum::http::{Request, Response};
use hyper::Body;
use crate::{database::DatabaseContext, web::schema::Schema};

pub async fn graphql(
    request: Request<Body>,
    schema_state: State<Arc<Schema>>,
    database_state: State<Arc<DatabaseContext>>,
) -> Response<Body> {
    let State(schema) = schema_state;
    let State(database) = database_state;

    juniper_hyper::graphql(schema, database, request).await
}

macro_rules! give_both_endpoints {
    ($handler:path, $address:ident) => { $handler($address, Some($address)) };
}

pub async fn graphiql(graphql_address: &str) -> Response<Body> {
    give_both_endpoints!(juniper_hyper::graphiql, graphql_address)
}

pub async fn playground(graphql_address: &str) -> Response<Body> {
    give_both_endpoints!(juniper_hyper::playground, graphql_address)
}
