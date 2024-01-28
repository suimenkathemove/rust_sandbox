use async_graphql::SimpleObject;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
}
