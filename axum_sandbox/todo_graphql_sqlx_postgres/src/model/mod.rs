mod todo;

use self::todo::Todo;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "OK"
    }

    async fn list(&self, ctx: &Context<'_>) -> Vec<Todo> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
            .fetch_all(pool)
            .await
            .unwrap();

        todos
    }

    async fn get(&self, ctx: &Context<'_>, id: Uuid) -> Todo {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .unwrap();

        todo
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create(&self, ctx: &Context<'_>, title: String) -> Todo {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todos (title) VALUES ($1) RETURNING *",
            title
        )
        .fetch_one(pool)
        .await
        .unwrap();

        todo
    }

    async fn update(&self, ctx: &Context<'_>, id: Uuid, title: String) -> Todo {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let todo = sqlx::query_as!(
            Todo,
            "UPDATE todos SET title = $1 WHERE id = $2 RETURNING *",
            title,
            id
        )
        .fetch_one(pool)
        .await
        .unwrap();

        todo
    }

    async fn delete(&self, ctx: &Context<'_>, id: Uuid) -> Todo {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let todo = sqlx::query_as!(Todo, "DELETE FROM todos WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await
            .unwrap();

        todo
    }
}
