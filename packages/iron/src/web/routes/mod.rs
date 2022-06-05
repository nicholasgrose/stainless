use actix_web::{
    dev::HttpServiceFactory,
    get, route, services,
    web::{Data, Payload},
    HttpRequest, HttpResponse,
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use crate::{database::DatabaseContext, web::schema::Schema};

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql<'a>(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
    database_context: Data<DatabaseContext>,
) -> actix_web::Result<HttpResponse> {
    graphql_handler(&schema, &database_context, req, payload).await
}

#[get("/graphiql")]
pub async fn graphiql() -> actix_web::Result<HttpResponse> {
    graphiql_handler("/graphql", None).await
}

#[get("/playground")]
pub async fn playground() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

pub fn all() -> impl HttpServiceFactory {
    services![graphql, graphiql, playground]
}
