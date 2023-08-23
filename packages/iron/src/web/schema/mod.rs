use async_graphql::{EmptyMutation, EmptySubscription};

use crate::web::schema::query::IronQueryRoot;

pub mod game;
pub mod query;

pub type IronSchema = async_graphql::Schema<IronQueryRoot, EmptyMutation, EmptySubscription>;
