use crate::{models::ToDo, schema::todos, ToDoError};
use async_graphql::{Context, Object, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    pub async fn get_todos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<ToDo>> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let mytodos = todos::table
            .load::<ToDo>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to fetch user: {}", e);
                ToDoError::UserNotFound
            })?;

        Ok(mytodos)
    }
}
