use actix_web::{
    get, route,
    web::{Data, Payload},
    HttpRequest, HttpResponse,
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use crate::{database::Database, web::schema::Schema};

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
) -> crate::Result<HttpResponse> {
    let context = Database::new();
    Ok(graphql_handler(&schema, &context, req, payload).await?)
}

#[get("/graphiql")]
pub async fn graphiql() -> crate::Result<HttpResponse> {
    Ok(graphiql_handler("/graphql", None).await?)
}

#[get("/playground")]
pub async fn playground() -> crate::Result<HttpResponse> {
    Ok(playground_handler("/graphql", None).await?)
}
